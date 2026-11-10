# OpenFactor

An open source, browser-based factory-building **platform**. The simulation is a
deterministic Rust library; the game is a content pack that runs on it.

## The one rule

    state(t+1) = sim(state(t), commands(t))

The simulation has no clock, no I/O, no floats and no randomised containers.
Anything unpredictable (players, bots, outside data) enters as a command. Replays,
saves, co-op sync and integrations all depend on this, so don't break it.

## Decisions

- **View:** top-down 2D and 3D views of the same world, which can stack up to eight levels.
- **Multiplayer:** small co-op (2-8 players). The server orders commands and every
  client runs the same simulation. Federation between worlds comes later.
- **Priority:** programmable platform first (data packs, external API, WASM mods),
  polish as a game second.
- **Saves:** `World::save` / `World::load` capture the whole state as bytes. The
  browser autosaves and can export and import `.ofsave` files; co-op late join
  will use the same snapshot.
- **Agents:** the play page registers WebMCP tools (`document.modelContext`) so an
  AI agent in the browser can look at and build the factory under the same rules
  as a player.

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

Art is [Tiny Factory](https://kenney.nl/assets/tiny-factory) and
[Factory Kit](https://kenney.nl/assets/factory-kit) by Kenney (CC0). The 3D view uses
[three.js](https://threejs.org) (MIT), vendored in `site/play/vendor`.
