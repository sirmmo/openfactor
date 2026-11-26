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
- **Multiplayer:** small co-op (2-8 players). The host's browser orders commands and
  sends each batch of ticks to the guests over WebRTC (PeerJS for introductions);
  every client runs the same simulation and compares state hashes. Late joiners
  start from a save. Federation between worlds comes later.
- **Priority:** programmable platform first (data packs, external API, WASM mods),
  polish as a game second.
- **Saves:** `World::save` / `World::load` capture the whole state as bytes. The
  browser autosaves and can export and import `.ofsave` files; co-op late join
  will use the same snapshot.
- **Rule sets and puzzles:** every game starts by choosing a pack (`packs/*.gbx`,
  or one loaded from a file) and free play, a sandbox or a puzzle. Puzzles are
  part of the pack and their goals are checked inside the simulation, so they are
  deterministic and shared in co-op. Campaign puzzles carry on in one factory;
  `fresh` puzzles are rounds on their own map, often timed. Switching packs
  reloads the page; saves record their pack.
- **Agents:** the play page registers WebMCP tools (`document.modelContext`) so an
  AI agent in the browser can look at and build the factory under the same rules
  as a player.

## OpenTactics holdings

OpenTactics can hand a factory, airport or port to a friend, who runs it here.
The play page reads `#holding=…` (deflate-raw JSON in base64url, or
`#holdingjson=…`), keeps a factory per OpenTactics game and tile in the browser,
and gives it a contract: one of the base pack's `Contracts` puzzles, picked by
what the tile builds (ground: gears and motors; air: circuits and controllers;
naval: steel and plates; otherwise plates). The factory may run for
`day × pace` minutes (pace 10 by default); then time waits for the next day.
**Report to OpenTactics** makes the `#report=…` link back, with
`income_pct = 50 + 150 × value delivered / contract value` (at most 200) and
`discount_pct = 25 × value delivered / contract value`, rounded down. The
contract puzzles are tested like any other; the links and the pace live in
[`site/play/holding.js`](site/play/holding.js), outside the simulation. The
format is OpenTactics' `HOLDINGS.md`, version 1.

## Milestones

1. Headless sim with replay tests (done).
2. **Browser client with a build and render loop (first version in `site/play`).**
3. **Two browsers in one world, with late join and desync detection (first version, browser to browser).**
4. **GlassBox-style rule packs: five rule sets with puzzles in `packs/`** (factory,
   tutorial, farming, chip fab, network), then an in-game editor and the external API.
5. WASM mods, then federation.

## Rule packs

Resources, machines, recipes, belt tiers, routing, power and pipe networks,
puzzles and their goals are all data: see [`packs/`](packs) and the "Rules"
section of the docs. Every pack is tested: each recipe must run when fed, and
each start must build its opening without a refusal.
The language follows citysim's GlassBox scripts, with additions for logistics.

## Running

    cargo test --workspace
    cargo run --release -p openfactor-sim --example demo [rows] [ticks]

The docs and play pages live in `site/`. Every push to `main` tests the code,
builds the WebAssembly client and publishes both to GitHub Pages.

## Licence and credits

The code is licensed under the GNU General Public License, version 2 or later.
See [COPYING](COPYING).

Art is [Tiny Factory](https://kenney.nl/assets/tiny-factory),
[Factory Kit](https://kenney.nl/assets/factory-kit), and for the other rule sets
[1-Bit Pack](https://kenney.nl/assets/1-bit-pack),
[Space Station Kit](https://kenney.nl/assets/space-station-kit),
[Tiny Farm](https://kenney.nl/assets/tiny-farm), [Nature Kit](https://kenney.nl/assets/nature-kit),
[Survival Kit](https://kenney.nl/assets/survival-kit), [Food Kit](https://kenney.nl/assets/food-kit),
[Cube Pets](https://kenney.nl/assets/cube-pets) and
[City Kit (Industrial)](https://kenney.nl/assets/city-kit-industrial),
[Mini Characters](https://kenney.nl/assets/mini-characters),
[Space Kit](https://kenney.nl/assets/space-kit), [Furniture Kit](https://kenney.nl/assets/furniture-kit),
[Toy Car Kit](https://kenney.nl/assets/toy-car-kit), [Brick Kit](https://kenney.nl/assets/brick-kit), Kenney's fonts and
[Interface Sounds](https://kenney.nl/assets/interface-sounds), all by Kenney (CC0); each
model folder in `site/play/assets/models` has its licence. The 3D view uses
[three.js](https://threejs.org) (MIT) and playing together uses [PeerJS](https://peerjs.com) (MIT),
both vendored in `site/play/vendor`.

## Publishing on Wavedash

OpenFactor is published on [Wavedash](https://wavedash.com) ([docs](https://docs.wavedash.com)) from
`wavedash.toml`, at the root.

1. **Create the game** in the Wavedash Developer Portal, or with the CLI (`curl -fsSL
   https://wavedash.com/cli/install.sh | sh`, `wavedash auth login`, then `wavedash project create
   --title "OpenFactor" --team-id <TEAM_ID>`).
2. **Set its id**: `game_id` in `wavedash.toml`, or, to keep it out of the repository,
   `WAVEDASH_GAME_ID_OPENFACTOR` (or `WAVEDASH_GAME_ID`) in the environment, and the repository
   variable `WAVEDASH_GAME_ID_OPENFACTOR` (Settings › Secrets and variables › Actions › Variables).
3. **Upload a build**: `sh tools/wavedash.sh` builds the WebAssembly client and copies the play page
   into `_wavedash/` the way the Pages workflow does (not the docs or the API reference) and, with
   the CLI on `PATH` and `WAVEDASH_TOKEN` set, runs `wavedash build push`. Without the CLI (or with
   `WAVEDASH_DRY_RUN=1`) it only says what it would push. On GitHub: Actions › Wavedash › Run
   workflow, with the secret `WAVEDASH_TOKEN`.
4. **Publish it**: `wavedash publish <BUILD_ID>` (the id `build push` printed), or in the Developer
   Portal.

On Wavedash the game stays behind Wavedash's loading screen until it calls `Wavedash.init()`:
`site/play/index.html` does it through `site/play/wavedash.js` once the client and the rules have
loaded and the first frame is asked for, or when loading fails, to show the error. Elsewhere (GitHub
Pages, a local server) there is no `Wavedash` object and the calls do nothing.

## Publishing on itch.io

The same builds go to [itch.io](https://itch.io) with
[butler](https://itch.io/docs/butler/), each to its own page as an HTML5 build: `openfactor` at https://sirmmo.itch.io/openfactor.
`sh tools/itch.sh [game]` builds them as for Wavedash and runs `butler push <build>
sirmmo/<page>:html5`; without butler, or with `ITCH_DRY_RUN=1`, it only says what it would push.
In CI, Actions > Itch > Run workflow does the same, with the repository secret `BUTLER_API_KEY`
(an API key from https://itch.io/user/settings/api-keys). Each page must exist on itch.io before
its first push (Dashboard > Create new project, kind HTML); after the first upload, tick "This
file will be played in the browser" on it once.

