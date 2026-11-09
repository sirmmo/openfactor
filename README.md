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

1. Headless sim with replay tests (done).
2. **Browser client with a build and render loop (first version in `site/play`).**
3. Two browsers in one world, with late join and desync detection.
4. GlassBox-style rule packs (first version in `packs/`), then an in-game
   inspector and editor, and the external API.
5. WASM mods, then federation.

## Rule packs

Resources, machines, recipes, belt tiers, routing, power and pipe networks are all
data: see [`packs/base.gbx`](packs/base.gbx) and the "Rules" section of the docs.
The language follows citysim's GlassBox scripts, with additions for logistics.

## Running

    cargo test --workspace
    cargo run --release -p openfactor-sim --example demo [rows] [ticks]

The docs and play pages live in `site/`. Every push to `main` tests the code,
builds the WebAssembly client and publishes both to GitHub Pages.

## Licence and credits

The code is licensed under the GNU General Public License, version 2 or later.
See [COPYING](COPYING).

Sandbox art is [Tiny Factory](https://kenney.nl/assets/tiny-factory) by Kenney (CC0).
