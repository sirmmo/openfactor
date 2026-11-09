# OpenFactor

An open source, browser-based factory-building **platform**. The simulation is a
deterministic Rust library; the game is a content pack that runs on it.

## The one rule

    state(t+1) = sim(state(t), commands(t))

The simulation has no clock, no I/O, no floats and no randomised containers.
Anything unpredictable (players, bots, outside data) enters as a command. Replays,
saves, co-op sync and integrations all depend on this, so don't break it.

## Decisions

- **View:** top-down 2D, with stacked layers later.
- **Multiplayer:** small co-op (2-8 players). The server orders commands and every
  client runs the same simulation. Federation between worlds comes later.
- **Priority:** programmable platform first (data packs, external API, WASM mods),
  polish as a game second.

## Milestones

1. **Headless sim with replay tests (current).**
2. Browser client with a build and render loop.
3. Two browsers in one world, with late join and desync detection.
4. Data packs and the external API.
5. WASM mods, then federation.

## Running

    cargo test
    cargo run --release -p openfactor-sim --example demo [rows] [ticks]
