// Sound for the game: short effects from Kenney's Interface Sounds (CC0), and a
// quiet background made on the spot with Web Audio. It only listens: the page
// tells it what happened, and nothing here touches the simulation.

const SOUND_ROOT = 'assets/sounds/';
const SOUND_KEY = 'openfactor.sound';
// The same sound played again sooner than this is skipped, so a dragged belt
// line doesn't turn into a buzz.
const REPEAT_MS = 70;
const EFFECT_VOLUME = 0.5, BACKGROUND_VOLUME = 0.06;

// Which recordings each theme uses for each event; one is picked at random.
const SETS = {
  factory: {
    build: ['drop_002', 'drop_003'], erase: ['minimize_003'], refuse: ['error_004'], click: ['click_002'],
    done: ['confirmation_002'], solved: ['maximize_006'], failed: ['bong_001'],
  },
  terminal: {
    build: ['tick_002', 'select_003'], erase: ['glitch_002'], refuse: ['glitch_004'], click: ['tick_001'],
    done: ['confirmation_004'], solved: ['maximize_008'], failed: ['error_008'],
  },
  farm: {
    build: ['pluck_001', 'pluck_002'], erase: ['scratch_002'], refuse: ['error_002'], click: ['click_004'],
    done: ['confirmation_001'], solved: ['maximize_004'], failed: ['bong_001'],
  },
};

/** Background recipes: filtered noise plus a few soft tones. */
const BACKGROUNDS = {
  // A low machine hum with a slow throb.
  factory: { noise: { type: 'lowpass', frequency: 260, q: 0.7, gain: 0.9 }, tones: [[55, 0.35], [110, 0.12]], throb: 0.6 },
  // A server room: a steady drone and a thin fan hiss.
  terminal: { noise: { type: 'highpass', frequency: 2400, q: 0.5, gain: 0.25 }, tones: [[98, 0.25], [147, 0.12], [196, 0.06]], throb: 0.15 },
  // Wind over the fields, rising and falling.
  farm: { noise: { type: 'bandpass', frequency: 500, q: 0.6, gain: 1.2 }, tones: [], throb: 0.08, gust: true },
};

export function createSound(theme = 'factory') {
  const set = SETS[theme] || SETS.factory;
  const recipe = BACKGROUNDS[theme] || BACKGROUNDS.factory;
  let on = true;
  try { on = localStorage.getItem(SOUND_KEY) !== 'off'; } catch { /* default on */ }

  let context = null, master = null, background = null;
  const buffers = new Map(), lastPlayed = new Map();

  // Browsers only allow sound after the player has done something on the page.
  function start() {
    if (context || !on) return;
    const AudioContext = window.AudioContext || window.webkitAudioContext;
    if (!AudioContext) return;
    context = new AudioContext();
    master = context.createGain();
    master.connect(context.destination);
    for (const names of Object.values(set)) for (const name of names) load(name);
    background = makeBackground();
  }
  for (const type of ['pointerdown', 'keydown']) window.addEventListener(type, start, { capture: true });

  function load(name) {
    if (buffers.has(name)) return;
    buffers.set(name, null);
    fetch(SOUND_ROOT + name + '.ogg')
      .then((response) => response.arrayBuffer())
      .then((data) => context.decodeAudioData(data))
      .then((buffer) => buffers.set(name, buffer))
      .catch(() => { /* this sound just stays silent */ });
  }

  function play(event, volume = 1) {
    if (!on || !context) return;
    const names = set[event];
    if (!names) return;
    const now = performance.now();
    if (now - (lastPlayed.get(event) || 0) < REPEAT_MS) return;
    lastPlayed.set(event, now);
    const buffer = buffers.get(names[Math.floor(Math.random() * names.length)]);
    if (!buffer) return;
    const source = context.createBufferSource();
    source.buffer = buffer;
    source.playbackRate.value = 0.95 + Math.random() * 0.1;
    const gain = context.createGain();
    gain.gain.value = EFFECT_VOLUME * volume;
    source.connect(gain).connect(master);
    source.start();
  }

  function makeBackground() {
    const out = context.createGain();
    out.gain.value = 0;
    out.connect(master);
    // Two seconds of noise, looped.
    const length = context.sampleRate * 2;
    const noiseBuffer = context.createBuffer(1, length, context.sampleRate);
    const data = noiseBuffer.getChannelData(0);
    let brown = 0;
    for (let i = 0; i < length; i++) {
      brown = (brown + 0.02 * (Math.random() * 2 - 1)) / 1.02;
      data[i] = brown * 3.5;
    }
    const noise = context.createBufferSource();
    noise.buffer = noiseBuffer;
    noise.loop = true;
    const filter = context.createBiquadFilter();
    filter.type = recipe.noise.type;
    filter.frequency.value = recipe.noise.frequency;
    filter.Q.value = recipe.noise.q;
    const noiseGain = context.createGain();
    noiseGain.gain.value = recipe.noise.gain;
    noise.connect(filter).connect(noiseGain).connect(out);
    noise.start();
    for (const [frequency, level] of recipe.tones) {
      const tone = context.createOscillator();
      tone.frequency.value = frequency;
      const toneGain = context.createGain();
      toneGain.gain.value = level;
      tone.connect(toneGain).connect(out);
      tone.start();
    }
    // A slow swell on top: a machine's throb, or gusts of wind.
    const swell = context.createOscillator();
    swell.frequency.value = recipe.gust ? 0.07 : 0.9;
    const depth = context.createGain();
    depth.gain.value = recipe.throb;
    const level = context.createGain();
    level.gain.value = 1;
    swell.connect(depth).connect(level.gain);
    swell.start();
    out.disconnect();
    out.connect(level).connect(master);
    return out;
  }

  return {
    play,
    get on() { return on; },
    /** Turns sound on or off, and remembers the choice. */
    toggle() {
      on = !on;
      try { localStorage.setItem(SOUND_KEY, on ? 'on' : 'off'); } catch { /* not remembered */ }
      if (on) start();
      if (master) master.gain.setTargetAtTime(on ? 1 : 0, context.currentTime, 0.05);
      return on;
    },
    /** How busy the factory is, from 0 to 1: the background grows with it. */
    activity(amount) {
      if (!background) return;
      const target = BACKGROUND_VOLUME * (0.35 + 0.65 * Math.min(1, Math.max(0, amount)));
      background.gain.setTargetAtTime(target, context.currentTime, 1.5);
    },
  };
}
