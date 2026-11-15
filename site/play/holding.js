// OpenTactics holdings: a factory that one player runs for another's army.
//
// OpenTactics opens this page with `#holding=…` (or `#holdingjson=…`): JSON
// compressed with deflate-raw and written in base64url, as in OpenTactics'
// own share links. The factory gets a contract, one of the base pack's
// "Contracts" puzzles, chosen by what the tile builds. It may run for `day ×
// pace` minutes of factory time in all, and answers with `#report=…` on the
// holding's `return` page. The contract is OpenTactics' HOLDINGS.md, version 1.
//
// Everything here is plain functions of the holding and the puzzle state, so
// the simulation never sees any of it: the contract is an ordinary puzzle.

export const TICKS_PER_MINUTE = 60 * 60;
const DEFAULT_PACE = 10;
const DEFAULT_RETURN = 'https://ingmmo.com/opentactics/';
const SAVE_PREFIX = 'openfactor.holding.';

/** Contract puzzles in packs/base.gbx, by the unit class the tile builds. */
const CONTRACTS = { ground: 'contractGround', air: 'contractAir', naval: 'contractSteel' };
const DEFAULT_CONTRACT = 'contractPlates';

/** What one delivered item is worth to a contract, roughly in iron plates. Others count 1. */
const VALUES = { ironPlate: 1, gear: 2, steel: 3, circuit: 3, motor: 10, controller: 15 };

const b64url = (bytes) => {
  let text = '';
  for (let i = 0; i < bytes.length; i += 0x8000) text += String.fromCharCode(...bytes.subarray(i, i + 0x8000));
  return btoa(text).replace(/\+/g, '-').replace(/\//g, '_').replace(/=+$/, '');
};
const unb64url = (text) => Uint8Array.from(atob(text.replace(/-/g, '+').replace(/_/g, '/')), (c) => c.charCodeAt(0));

async function pipe(bytes, stream) {
  const out = new Blob([bytes]).stream().pipeThrough(stream);
  return new Uint8Array(await new Response(out).arrayBuffer());
}

/** `key=…` for a link's fragment: compressed when the browser can, plain JSON otherwise. */
export async function pack(key, value) {
  const json = new TextEncoder().encode(JSON.stringify(value));
  if (typeof CompressionStream === 'function') return key + '=' + b64url(await pipe(json, new CompressionStream('deflate-raw')));
  return key + 'json=' + b64url(json);
}

/** What a fragment like `#key=…` or `#keyjson=…` holds, or null if it holds no such thing. */
export async function unpack(key, hash) {
  const match = new RegExp('^#' + key + '(json)?=([A-Za-z0-9_-]+)$').exec(hash || '');
  if (!match) return null;
  let bytes = unb64url(match[2]);
  if (!match[1]) bytes = await pipe(bytes, new DecompressionStream('deflate-raw'));
  return JSON.parse(new TextDecoder().decode(bytes));
}

const whole = (value, fallback) => (Number.isInteger(value) ? value : fallback);
const text = (value, fallback, max = 80) => (typeof value === 'string' && value.trim() ? value.trim().slice(0, max) : fallback);
function webAddress(value) {
  try {
    const url = new URL(value);
    if (url.protocol === 'https:' || url.protocol === 'http:') return url.href.replace(/#.*$/, '');
  } catch { /* not a web address */ }
  return DEFAULT_RETURN;
}

/**
 * The holding in `hash`, checked and with defaults filled in; null when the
 * page was opened without one. Throws an Error with a message for the player
 * when the link is damaged or meant for another game.
 */
export async function readHolding(hash) {
  if (!/^#holding(json)?=/.test(hash || '')) return null;
  let raw;
  try { raw = await unpack('holding', hash); } catch { raw = null; }
  if (!raw || typeof raw !== 'object') throw new Error('That holding link is damaged. Ask for a new one in OpenTactics.');
  if (raw.kind !== 'factory') throw new Error('That holding is a ' + text(raw.kind, 'mystery', 20) + ', not a factory: it is run in another game.');
  if (whole(raw.v, 0) < 1) throw new Error('That holding link is from an unknown version of OpenTactics.');
  const tile = Array.isArray(raw.tile) && raw.tile.length === 2 && raw.tile.every(Number.isInteger) ? raw.tile : null;
  const game = text(raw.game, null, 64);
  if (!tile || !game) throw new Error('That holding link doesn’t say which game and tile it is for.');
  const pace = typeof raw.pace === 'number' && raw.pace > 0 ? Math.min(raw.pace, 24 * 60) : DEFAULT_PACE;
  return {
    v: 1,
    kind: 'factory',
    terrain: text(raw.terrain, 'factory', 20),
    builds: Array.isArray(raw.builds) ? raw.builds.filter((b) => typeof b === 'string') : [],
    game,
    map: text(raw.map, ''),
    tile,
    name: text(raw.name, text(raw.map, 'Holding') + ' ' + tile.join(',')),
    owner: whole(raw.owner, 0),
    army: text(raw.army, 'An army', 40),
    color: typeof raw.color === 'string' && /^#[0-9a-f]{3}([0-9a-f]{3})?$/i.test(raw.color) ? raw.color : '#9a9c9e',
    day: Math.max(1, whole(raw.day, 1)),
    pace,
    return: webAddress(raw.return),
  };
}

/** The id of the contract puzzle for a tile that builds `builds`. */
export function contractFor(builds) {
  const kind = builds.find((b) => CONTRACTS[b]);
  return kind ? CONTRACTS[kind] : DEFAULT_CONTRACT;
}

/** Where this browser keeps the factory of a holding: one per OpenTactics game and tile. */
export const saveKey = (holding) => SAVE_PREFIX + holding.game + '.' + holding.tile.join(',');

/** Ticks of factory time the holding may have run by its current day. */
export const allowance = (holding) => Math.round(holding.day * holding.pace * TICKS_PER_MINUTE);

/** A saved holding brought up to date by a newer link for the same tile: it keeps the later day. */
export function merge(saved, link) {
  if (!saved) return link;
  return { ...link, day: Math.max(link.day, whole(saved.day, 1)) };
}

/**
 * The report on how the contract went. `goals` are the puzzle's goals
 * (["deliver", item, amount]), `progress` the puzzle state's [done, needed]
 * pairs, `resources` the pack's resources, `ticks` the factory's time.
 */
export function report(holding, goals, progress, resources, ticks) {
  let worth = 0, earned = 0;
  const parts = [], stats = {};
  goals.forEach(([, item, amount], i) => {
    const resource = resources[item] || { id: '?', name: '?' };
    const done = Math.min(progress[i] ? progress[i][0] : 0, amount);
    const value = VALUES[resource.id] || 1;
    worth += value * amount;
    earned += value * done;
    parts.push(done.toLocaleString('en') + ' of ' + amount.toLocaleString('en') + ' × ' + resource.name);
    stats[resource.name] = done;
  });
  const share = worth > 0 ? earned / worth : 0;
  stats['Factory minutes'] = Math.floor(ticks / TICKS_PER_MINUTE);
  return {
    v: 1,
    kind: 'factory',
    game: holding.game,
    tile: holding.tile,
    owner: holding.owner,
    day: holding.day,
    income_pct: Math.min(200, Math.floor(50 + 150 * share)),
    discount_pct: Math.floor(25 * share),
    name: holding.name,
    summary: 'Delivered ' + parts.join(', '),
    stats,
  };
}

/** The link that hands `report` back to OpenTactics. */
export async function reportLink(holding, value) {
  return holding.return + '#' + (await pack('report', value));
}

/** The link that opens `holding` here, for trying it out and for tests. */
export async function holdingLink(page, holding) {
  return page.replace(/#.*$/, '') + '#' + (await pack('holding', holding));
}
