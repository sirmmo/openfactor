//! Browser sandbox bindings: a plain numeric ABI, so no JS glue generator is needed.
//!
//! Edits are queued and applied on the next `of_step`. A returned pointer is
//! only valid until the next call, because wasm memory can move when it grows.

use openfactor_sim::{Building, Command, Dir, ItemId, MetricId, Pack, Pos, UnitId, World};
use std::cell::RefCell;

const BASE_PACK: &str = include_str!("../../../packs/base.gbx");
const DIRS: [Dir; 4] = [Dir::North, Dir::East, Dir::South, Dir::West];
/// Building codes: 0 is a belt, and n is the pack's unit n - 1.
const KIND_BELT: i32 = 0;

struct Game {
    world: World,
    queued: Vec<Command>,
    snapshot: Vec<i32>,
    pack_json: String,
}

impl Game {
    fn new() -> Game {
        let pack = Pack::parse(BASE_PACK).expect("the base pack is checked by a test");
        Game {
            world: World::new(&pack),
            queued: pack.worldgen.starter.clone(),
            snapshot: Vec::new(),
            pack_json: pack_json(&pack),
        }
    }

    fn write_snapshot(&mut self) {
        let layers = self.world.content().layers();
        let out = &mut self.snapshot;
        out.clear();

        let mark = reserve_count(out);
        for (pos, layer, _) in self.world.layer_tiles() {
            if !layers[usize::from(layer.0)].sprites.is_empty() {
                out.extend([pos.x, pos.y, i32::from(layer.0)]);
            }
        }
        finish_count(out, mark, 3);

        let mark = reserve_count(out);
        for (pos, dir, building) in self.world.buildings() {
            out.extend([pos.x, pos.y, dir_code(dir), building_code(building)]);
        }
        finish_count(out, mark, 4);

        let mark = reserve_count(out);
        for (pos, dir, offset, item) in self.world.belt_items() {
            out.extend([pos.x, pos.y, dir_code(dir), offset as i32, i32::from(item.0)]);
        }
        finish_count(out, mark, 5);
    }
}

fn reserve_count(out: &mut Vec<i32>) -> usize {
    out.push(0);
    out.len()
}

fn finish_count(out: &mut [i32], mark: usize, record_size: usize) {
    out[mark - 1] = ((out.len() - mark) / record_size) as i32;
}

fn dir_code(dir: Dir) -> i32 {
    DIRS.iter().position(|&d| d == dir).expect("all directions are listed") as i32
}

fn building_code(building: Building) -> i32 {
    match building {
        Building::Belt => KIND_BELT,
        Building::Unit(unit) => i32::from(unit.0) + 1,
    }
}

fn building_from_code(kind: i32) -> Building {
    if kind == KIND_BELT {
        Building::Belt
    } else {
        // An out-of-range code is refused by the simulation as an unknown unit.
        Building::Unit(UnitId((kind - 1) as u16))
    }
}

fn json_string(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 2);
    out.push('"');
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c if u32::from(c) < 0x20 => out.push(' '),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// What the page needs to build its toolbar and draw the pack's content.
fn pack_json(pack: &Pack) -> String {
    let content = &pack.content;
    let units: Vec<String> = content
        .units()
        .iter()
        .map(|unit| {
            format!(
                r#"{{"id":{},"name":{},"sprite":{},"sends":{},"accepts":{}}}"#,
                json_string(&unit.id),
                json_string(&unit.name),
                unit.sprite,
                !unit.outputs.is_empty(),
                unit.accepts_items()
            )
        })
        .collect();
    let resources: Vec<String> = content
        .resources()
        .iter()
        .map(|r| format!(r#"{{"name":{},"sprite":{}}}"#, json_string(&r.name), r.sprite))
        .collect();
    let layers: Vec<String> = content
        .layers()
        .iter()
        .map(|layer| format!(r#"{{"sprites":{:?}}}"#, layer.sprites))
        .collect();
    let metrics: Vec<String> =
        content.metrics().iter().map(|m| format!(r#"{{"name":{}}}"#, json_string(&m.name))).collect();
    format!(
        r#"{{"units":[{}],"resources":[{}],"layers":[{}],"metrics":[{}]}}"#,
        units.join(","),
        resources.join(","),
        layers.join(","),
        metrics.join(",")
    )
}

thread_local! {
    static GAME: RefCell<Game> = RefCell::new(Game::new());
}

fn with_game<T>(f: impl FnOnce(&mut Game) -> T) -> T {
    GAME.with(|game| f(&mut game.borrow_mut()))
}

#[no_mangle]
pub extern "C" fn of_reset() {
    with_game(|game| *game = Game::new());
}

/// The pack's definitions as JSON text. Read `of_pack_json_len` bytes from here.
#[no_mangle]
pub extern "C" fn of_pack_json() -> *const u8 {
    with_game(|game| game.pack_json.as_ptr())
}

#[no_mangle]
pub extern "C" fn of_pack_json_len() -> u32 {
    with_game(|game| game.pack_json.len() as u32)
}

/// Queues a placement. `kind` is 0 for a belt, or 1 + the pack's unit index.
#[no_mangle]
pub extern "C" fn of_place(x: i32, y: i32, dir: i32, kind: i32) {
    let building = building_from_code(kind);
    let dir = DIRS[dir.rem_euclid(4) as usize];
    let pos = Pos::new(x, y);
    with_game(|game| game.queued.push(Command::Place { pos, dir, building }));
}

#[no_mangle]
pub extern "C" fn of_remove(x: i32, y: i32) {
    with_game(|game| game.queued.push(Command::Remove { pos: Pos::new(x, y) }));
}

/// Applies queued edits and advances `ticks` ticks. Returns how many edits were rejected.
#[no_mangle]
pub extern "C" fn of_step(ticks: u32) -> u32 {
    with_game(|game| {
        let mut rejected = 0;
        if ticks > 0 {
            let commands = std::mem::take(&mut game.queued);
            rejected = game.world.step(&commands).len() as u32;
            for _ in 1..ticks {
                game.world.step(&[]);
            }
        }
        rejected
    })
}

/// Writes the view as three counted lists of i32 records: drawn map tiles
/// (x, y, layer), buildings (x, y, dir, kind) and belt items (x, y, dir, offset, item).
#[no_mangle]
pub extern "C" fn of_snapshot() -> *const i32 {
    with_game(|game| {
        game.write_snapshot();
        game.snapshot.as_ptr()
    })
}

#[no_mangle]
pub extern "C" fn of_snapshot_len() -> u32 {
    with_game(|game| game.snapshot.len() as u32)
}

#[no_mangle]
pub extern "C" fn of_tick() -> u32 {
    with_game(|game| game.world.tick() as u32)
}

#[no_mangle]
pub extern "C" fn of_produced(item: u32) -> u32 {
    with_game(|game| game.world.stats(ItemId(item as u16)).produced as u32)
}

#[no_mangle]
pub extern "C" fn of_metric(metric: u32) -> u32 {
    with_game(|game| game.world.metric(MetricId(metric as u16)).total as u32)
}

#[no_mangle]
pub extern "C" fn of_hash_hi() -> u32 {
    with_game(|game| (game.world.state_hash() >> 32) as u32)
}

#[no_mangle]
pub extern "C" fn of_hash_lo() -> u32 {
    with_game(|game| game.world.state_hash() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_pack_builds_and_makes_widgets() {
        of_reset();
        assert_eq!(of_step(3600), 0);
        assert!(of_produced(2) > 0);
        assert!(of_metric(0) > 0);
        assert!(!of_snapshot().is_null() && of_snapshot_len() > 0);
        println!("starter hash {:08x}{:08x}", of_hash_hi(), of_hash_lo());
    }

    #[test]
    fn pack_json_lists_the_units() {
        let json = with_game(|game| game.pack_json.clone());
        assert!(json.contains(r#""id":"depot","name":"Depot","sprite":58,"sends":false,"accepts":true"#));
        assert!(json.contains(r#"{"sprites":[3, 32]}"#));
    }
}
