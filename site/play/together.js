// Playing together: one browser hosts, others join with a short code.
//
// The host's browser is in charge. It steps the world as usual and sends each
// batch of ticks to every guest, with the edits applied in it. Guests send
// their edits to the host instead of applying them, and step their own copy
// only as batches arrive, so every copy runs the same commands on the same
// ticks and stays identical. A joining guest starts from a save of the host's
// world. Once a second the host includes its state hash, and a guest that no
// longer matches asks for a fresh save.
//
// Browsers find each other through the PeerJS broker, then talk directly over
// WebRTC data channels. This module wraps the wasm calls on the page's `of`
// object, so the rest of the page doesn't need to know who is in charge.

const PREFIX = 'openfactor-';
const CODE_LETTERS = 'ABCDEFGHJKMNPQRSTUVWXYZ23456789';
const CODE_LENGTH = 6;
const HASH_EVERY = 60;
const CURSOR_MS = 100;
const COLORS = ['#f2b63c', '#4fc3f7', '#ef6aa8', '#7bd389', '#b39ddb', '#ff8a65', '#4dd0e1', '#dce775'];
const EDITS = ['of_place', 'of_remove', 'of_configure', 'of_research'];

function newCode() {
  const values = crypto.getRandomValues(new Uint32Array(CODE_LENGTH));
  return [...values].map((v) => CODE_LETTERS[v % CODE_LETTERS.length]).join('');
}
export const normalCode = (text) => String(text || '').toUpperCase().replace(/[^A-Z0-9]/g, '').slice(0, CODE_LENGTH);

function toBase64(bytes) {
  let text = '';
  for (let i = 0; i < bytes.length; i += 0x8000) text += String.fromCharCode(...bytes.subarray(i, i + 0x8000));
  return btoa(text);
}
function fromBase64(text) {
  const raw = atob(text);
  const bytes = new Uint8Array(raw.length);
  for (let i = 0; i < raw.length; i++) bytes[i] = raw.charCodeAt(i);
  return bytes;
}
const isRecord = (r) => Array.isArray(r) && r.length === 6 && r.every(Number.isInteger) && r[0] >= 0 && r[0] <= 3;

/**
 * `game` supplies: of (the page's mutable copy of the wasm exports), say(text),
 * replaced() after the world is swapped, leaveToOwn() to go back to the
 * player's own factory, and changed() when the session changes.
 */
export function createSession(game) {
  const { of } = game;
  const real = {};
  for (const name of [...EDITS, 'of_step', 'of_reset', 'of_load']) real[name] = of[name].bind(of);

  const session = {
    role: 'solo',
    code: null,
    peers: new Map(), // id -> { name, color, cursor, conn }
    selfColor: COLORS[0],
    lastError: null,
  };
  let peer = null, hostConn = null;
  let batches = [], resyncAsked = false, lastCursorSent = 0, lastCursorKey = '';

  const send = (conn, message) => { if (conn && conn.open) conn.send(message); };
  const broadcast = (message, except = null) => {
    for (const [id, p] of session.peers) if (id !== except) send(p.conn, message);
  };
  const tickHash = () => [of.of_hash_hi() >>> 0, of.of_hash_lo() >>> 0];

  function applyRecord(r) {
    if (r[0] === 0) real.of_place(r[1], r[2], r[3], r[4], r[5]);
    else if (r[0] === 1) real.of_remove(r[1], r[2], r[3]);
    else if (r[0] === 2) real.of_configure(r[1], r[2], r[3], r[4]);
    else real.of_research(r[1]);
  }
  function applied() {
    const length = of.of_applied_len();
    if (!length) return [];
    return Array.from(new Int32Array(of.memory.buffer, of.of_applied(), length));
  }
  function snapshot() {
    const pointer = of.of_save(), length = of.of_save_len();
    return { t: 'snap', tick: of.of_tick(), save: toBase64(new Uint8Array(of.memory.buffer, pointer, length).slice()) };
  }
  function loadSnapshot(message) {
    const bytes = fromBase64(message.save);
    new Uint8Array(of.memory.buffer, of.of_load_buffer(bytes.length), bytes.length).set(bytes);
    return real.of_load();
  }

  // --- The wrapped calls.
  for (const name of EDITS) {
    of[name] = (...args) => {
      if (session.role !== 'guest') return real[name](...args);
      const r = name === 'of_place' ? [0, ...args]
        : name === 'of_remove' ? [1, ...args, 0, 0]
        : name === 'of_configure' ? [2, ...args, 0]
        : [3, args[0], 0, 0, 0, 0];
      send(hostConn, { t: 'cmd', r });
    };
  }
  of.of_step = (ticks) => {
    if (session.role === 'guest') return drain();
    const from = of.of_tick();
    const refused = real.of_step(ticks);
    if (session.role === 'host' && ticks > 0 && session.peers.size) {
      const message = { t: 'b', f: from, n: ticks, c: applied() };
      if (Math.floor((from + ticks) / HASH_EVERY) > Math.floor(from / HASH_EVERY)) message.h = tickHash();
      broadcast(message);
    }
    return refused;
  };
  // Starting over or loading a file as host takes everyone along.
  for (const name of ['of_reset', 'of_load']) {
    of[name] = (...args) => {
      if (session.role === 'guest') return name === 'of_load' ? 1 : undefined;
      const result = real[name](...args);
      if (session.role === 'host') broadcast(snapshot());
      return result;
    };
  }

  // A guest steps only through what the host has sent.
  function drain() {
    while (batches.length) {
      const batch = batches[0];
      const tick = of.of_tick();
      if (batch.f < tick) { batches.shift(); continue; }
      if (batch.f > tick) { askResync('missed ticks'); return 0; }
      batches.shift();
      for (let i = 0; i + 6 <= batch.c.length; i += 6) {
        const record = batch.c.slice(i, i + 6);
        if (isRecord(record)) applyRecord(record);
      }
      real.of_step(batch.n);
      if (batch.h) {
        const [hi, lo] = tickHash();
        if (hi !== batch.h[0] || lo !== batch.h[1]) askResync('out of step');
      }
    }
    return 0;
  }
  function askResync(why) {
    if (resyncAsked) return;
    resyncAsked = true;
    batches = [];
    console.warn('Asking the host for a fresh copy:', why);
    send(hostConn, { t: 'resync' });
  }

  // --- Hosting.
  let library = null;
  function loadLibrary() {
    if (window.peerjs && window.peerjs.Peer) return Promise.resolve();
    library = library || new Promise((resolve, reject) => {
      const script = document.createElement('script');
      script.src = new URL('./vendor/peerjs.min.js', import.meta.url).href;
      script.onload = resolve;
      script.onerror = () => { library = null; reject(new Error('the connection library did not load')); };
      document.head.append(script);
    });
    return library;
  }
  function openPeer(id) {
    const Peer = window.peerjs && window.peerjs.Peer;
    if (!Peer) throw new Error('the connection library did not load');
    return id ? new Peer(id, { debug: 0 }) : new Peer({ debug: 0 });
  }

  async function host() {
    if (session.role !== 'solo') return;
    try { await loadLibrary(); } catch (error) { game.say('Couldn’t start a shared game: ' + error.message, 6000); return; }
    if (session.role !== 'solo') return;
    const code = newCode();
    peer = openPeer(PREFIX + code);
    session.role = 'host';
    session.code = code;
    session.state = 'connecting';
    session.selfColor = COLORS[0];
    game.changed();
    peer.on('open', () => { session.state = 'open'; game.changed(); });
    peer.on('error', (error) => {
      if (error.type === 'unavailable-id' && session.state !== 'open') { stop(); host(); return; }
      if (session.role === 'host' && session.state !== 'open') {
        stop();
        game.say('Couldn’t start a shared game: ' + describe(error), 7000);
      }
    });
    peer.on('connection', (conn) => {
      conn.on('data', (message) => onGuestMessage(conn, message));
      conn.on('close', () => dropGuest(conn));
      conn.on('error', () => dropGuest(conn));
    });
  }

  function onGuestMessage(conn, message) {
    if (!message || typeof message !== 'object') return;
    const id = conn.peer;
    if (message.t === 'hello') {
      const used = new Set([COLORS[0], ...[...session.peers.values()].map((p) => p.color)]);
      const color = COLORS.find((c) => !used.has(c)) || COLORS[session.peers.size % COLORS.length];
      const name = 'Player ' + (session.peers.size + 2);
      session.peers.set(id, { name, color, cursor: null, conn });
      send(conn, { t: 'welcome', name, color, hostColor: COLORS[0] });
      send(conn, snapshot());
      broadcast({ t: 'peers', list: peerList() });
      game.say(name + ' joined.', 4000);
      game.changed();
      return;
    }
    if (!session.peers.has(id)) return;
    if (message.t === 'cmd' && isRecord(message.r)) applyRecord(message.r);
    else if (message.t === 'resync') send(conn, snapshot());
    else if (message.t === 'cur') {
      const cursor = cursorFrom(message);
      session.peers.get(id).cursor = cursor;
      broadcast({ t: 'cur', id, ...(cursor || { none: true }) }, id);
    }
  }
  function peerList() {
    return [{ id: 'host', name: 'Host', color: COLORS[0] },
      ...[...session.peers].map(([id, p]) => ({ id, name: p.name, color: p.color }))];
  }
  function dropGuest(conn) {
    const p = session.peers.get(conn.peer);
    if (!p) return;
    session.peers.delete(conn.peer);
    broadcast({ t: 'peers', list: peerList() });
    game.say(p.name + ' left.', 4000);
    game.changed();
  }

  // --- Joining.
  async function join(rawCode) {
    if (session.role !== 'solo') return;
    const code = normalCode(rawCode);
    if (code.length !== CODE_LENGTH) { game.say('A game code has ' + CODE_LENGTH + ' letters and digits.', 4000); return; }
    try { await loadLibrary(); } catch (error) { game.say('Couldn’t join: ' + error.message, 6000); return; }
    if (session.role !== 'solo') return;
    game.beforeJoin();
    session.role = 'guest';
    session.code = code;
    session.state = 'connecting';
    batches = [];
    resyncAsked = false;
    game.changed();
    peer = openPeer();
    peer.on('open', () => {
      hostConn = peer.connect(PREFIX + code, { reliable: true });
      hostConn.on('open', () => send(hostConn, { t: 'hello' }));
      hostConn.on('data', onHostMessage);
      hostConn.on('close', () => hostGone('The host left the game.'));
      hostConn.on('error', () => hostGone('Lost the connection to the host.'));
    });
    peer.on('error', (error) => {
      const text = error.type === 'peer-unavailable' ? 'No game is open with the code ' + code + '.' : 'Couldn’t join: ' + describe(error);
      hostGone(text);
    });
  }
  let joined = false;
  function onHostMessage(message) {
    if (!message || typeof message !== 'object') return;
    if (message.t === 'welcome') {
      session.selfName = message.name;
      session.selfColor = message.color;
    } else if (message.t === 'snap') {
      const code = loadSnapshot(message);
      if (code !== 0) {
        hostGone(code === 3 || code === 2
          ? 'The host is running a different version of the game. Reload both pages and try again.'
          : 'The host’s factory couldn’t be loaded.');
        return;
      }
      batches = batches.filter((b) => b.f >= of.of_tick());
      resyncAsked = false;
      game.replaced();
      if (!joined) { joined = true; session.state = 'open'; game.say('You joined the game. Everything you build is shared.', 5000); }
      game.changed();
    } else if (message.t === 'b') {
      if (Number.isInteger(message.f) && Number.isInteger(message.n) && message.n > 0 && Array.isArray(message.c)) batches.push(message);
    } else if (message.t === 'peers' && Array.isArray(message.list)) {
      const known = new Map(session.peers);
      session.peers = new Map(message.list.filter((p) => p.name !== session.selfName)
        .map((p) => [p.id, { name: p.name, color: p.color, cursor: known.get(p.id)?.cursor || null }]));
      game.changed();
    } else if (message.t === 'cur') {
      const id = message.id || 'host';
      const entry = session.peers.get(id);
      if (entry) entry.cursor = cursorFrom(message);
    }
  }
  function hostGone(text) {
    if (session.role !== 'guest') return;
    const wasIn = joined;
    stop();
    game.say(text + (wasIn ? ' You’re back in your own factory.' : ''), 7000);
    if (wasIn) game.leaveToOwn();
  }

  // --- Both.
  function cursorFrom(message) {
    if (message.none) return null;
    const { x, y, z } = message;
    return [x, y, z].every(Number.isInteger) ? { x, y, z } : null;
  }
  /** Called every frame with the tile under the pointer, or null. */
  function pointer(cell, now) {
    if (session.role === 'solo' || now - lastCursorSent < CURSOR_MS) return;
    const key = cell ? cell.x + ',' + cell.y + ',' + cell.z : '';
    if (key === lastCursorKey) return;
    lastCursorSent = now;
    lastCursorKey = key;
    const message = { t: 'cur', ...(cell ? { x: cell.x, y: cell.y, z: cell.z } : { none: true }) };
    if (session.role === 'host') broadcast({ ...message, id: 'host' });
    else send(hostConn, message);
  }
  function cursors() {
    return [...session.peers.values()].filter((p) => p.cursor).map((p) => ({ ...p.cursor, name: p.name, color: p.color }));
  }
  function stop() {
    const wasGuest = session.role === 'guest';
    session.role = 'solo';
    session.code = null;
    session.state = null;
    session.peers = new Map();
    batches = [];
    joined = false;
    hostConn = null;
    if (peer) { try { peer.destroy(); } catch { /* already gone */ } }
    peer = null;
    game.changed();
    return wasGuest;
  }
  function leave() {
    if (session.role === 'solo') return;
    const wasGuest = joined && session.role === 'guest';
    stop();
    if (wasGuest) game.leaveToOwn();
  }
  function describe(error) {
    if (error && error.type === 'network') return 'can’t reach the connection service.';
    if (error && error.type === 'browser-incompatible') return 'this browser can’t make direct connections.';
    return (error && error.message) || 'unknown error';
  }

  addEventListener('pagehide', () => { if (peer) peer.destroy(); });
  return Object.assign(session, { host, join, leave, pointer, cursors, inviteLink: () => location.origin + location.pathname + '?join=' + session.code });
}
