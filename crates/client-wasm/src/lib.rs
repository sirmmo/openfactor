//! Browser sandbox bindings: a plain numeric ABI, so no JS glue generator is needed.
//!
//! Edits are queued and applied on the next `of_step`. A snapshot pointer is
//! only valid until the next call, because wasm memory can move when it grows.

use openfactor_sim::{Building, Command, Content, Dir, ItemId, Patch, Pos, RecipeId, World, WorldGen};
use std::cell::RefCell;

const DIRS: [Dir; 4] = [Dir::North, Dir::East, Dir::South, Dir::West];
const KIND_BELT: i32 = 0;
const KIND_MINER: i32 = 1;
const KIND_SMELTER: i32 = 2;
const KIND_ASSEMBLER: i32 = 3;
const KIND_DEPOT: i32 = 4;

struct Game {
    world: World,
    queued: Vec<Command>,
    snapshot: Vec<i32>,
}

impl Game {
    fn new() -> Game {
        let patch = |x, y, width, height| Patch {
            origin: Pos::new(x, y),
            width,
            height,
            item: ItemId(0),
            amount_per_tile: 100_000,
        };
        let worldgen = WorldGen { patches: vec![patch(0, 0, 6, 6), patch(4, 12, 8, 4)] };
        let world = World::new(Content::example(), &worldgen).expect("example content is valid");
        Game { world, queued: starter_factory(), snapshot: Vec::new() }
    }

    fn write_snapshot(&mut self) {
        let out = &mut self.snapshot;
        out.clear();

        let mark = reserve_count(out);
        for (pos, item, _) in self.world.resources() {
            out.extend([pos.x, pos.y, i32::from(item.0)]);
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
        Building::Miner => KIND_MINER,
        Building::Assembler { recipe } => KIND_SMELTER + i32::from(recipe.0),
        Building::Depot => KIND_DEPOT,
    }
}

fn building_from_code(kind: i32) -> Option<Building> {
    match kind {
        KIND_BELT => Some(Building::Belt),
        KIND_MINER => Some(Building::Miner),
        KIND_SMELTER => Some(Building::Assembler { recipe: RecipeId(0) }),
        KIND_ASSEMBLER => Some(Building::Assembler { recipe: RecipeId(1) }),
        KIND_DEPOT => Some(Building::Depot),
        _ => None,
    }
}

/// A working ore -> plate -> widget line, so the page opens with something moving.
fn starter_factory() -> Vec<Command> {
    let at = |x, building| Command::Place { pos: Pos::new(x, 2), dir: Dir::East, building };
    let mut commands = vec![at(2, Building::Miner)];
    commands.extend((3..=8).map(|x| at(x, Building::Belt)));
    commands.push(at(9, Building::Assembler { recipe: RecipeId(0) }));
    commands.extend((10..=14).map(|x| at(x, Building::Belt)));
    commands.push(at(15, Building::Assembler { recipe: RecipeId(1) }));
    commands.extend((16..=19).map(|x| at(x, Building::Belt)));
    commands.push(at(20, Building::Depot));
    commands
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

/// Queues a placement. `kind` is one of the KIND_ codes; unknown kinds are ignored.
#[no_mangle]
pub extern "C" fn of_place(x: i32, y: i32, dir: i32, kind: i32) {
    if let Some(building) = building_from_code(kind) {
        let dir = DIRS[dir.rem_euclid(4) as usize];
        let pos = Pos::new(x, y);
        with_game(|game| game.queued.push(Command::Place { pos, dir, building }));
    }
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

/// Writes the view as three counted lists of i32 records: resources (x, y, item),
/// buildings (x, y, dir, kind) and belt items (x, y, dir, offset, item).
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
    fn starter_factory_builds_and_makes_widgets() {
        of_reset();
        assert_eq!(of_step(3600), 0);
        assert!(of_produced(2) > 0);
        assert!(!of_snapshot().is_null() && of_snapshot_len() > 0);
        println!("starter hash {:08x}{:08x}", of_hash_hi(), of_hash_lo());
    }
}
