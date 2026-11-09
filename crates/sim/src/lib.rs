//! Deterministic factory simulation.
//!
//! `World::step` is a pure function of the current state and the commands for
//! that tick. To keep it that way this crate must not use floats, clocks, I/O
//! or `HashMap`/`HashSet` (their iteration order is randomised).

#![forbid(unsafe_code)]

mod belt;
mod content;
mod hash;
mod world;

pub use belt::{ITEM_SPACING, TILE};
pub use content::{Content, ContentError, Patch, Recipe, WorldGen};
pub use world::{Building, Command, ItemStats, Reject, Rejected, World};

pub type Tick = u64;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ItemId(pub u16);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RecipeId(pub u16);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct EntityId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    pub const fn new(x: i32, y: i32) -> Self {
        Pos { x, y }
    }

    pub fn step(self, dir: Dir) -> Pos {
        let (dx, dy) = match dir {
            Dir::North => (0, -1),
            Dir::East => (1, 0),
            Dir::South => (0, 1),
            Dir::West => (-1, 0),
        };
        Pos::new(self.x + dx, self.y + dy)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dir {
    North,
    East,
    South,
    West,
}

impl Dir {
    pub fn opposite(self) -> Dir {
        match self {
            Dir::North => Dir::South,
            Dir::East => Dir::West,
            Dir::South => Dir::North,
            Dir::West => Dir::East,
        }
    }
}
