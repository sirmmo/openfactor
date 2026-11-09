//! Deterministic factory simulation.
//!
//! `World::step` is a pure function of the current state and the commands for
//! that tick. To keep it that way this crate must not use floats, clocks, I/O
//! or `HashMap`/`HashSet` (their iteration order is randomised).
//!
//! Apart from belts, everything in the world is defined by a rule [`Pack`].

#![forbid(unsafe_code)]

mod belt;
mod content;
mod gbx;
mod hash;
mod world;

pub use belt::{ITEM_SPACING, TILE};
pub use content::{
    Amount, BeltDef, Condition, Content, LayerDef, Link, MapRule, MetricDef, Overlay, Pack, Port,
    ResourceDef, Route, Rule, Scope, TechDef, Test, UnitDef, WorldGen,
};
pub use gbx::PackError;
pub use world::{
    Building, Command, Inspection, ItemStats, MetricValue, NetworkView, Reject, Rejected, Status,
    World,
};

pub type Tick = u64;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ItemId(pub u16);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnitId(pub u16);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BeltId(pub u16);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TechId(pub u16);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct LayerId(pub u16);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MetricId(pub u16);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct EntityId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
    /// The level: 0 is the ground.
    pub z: i32,
}

impl Pos {
    /// A position on the ground.
    pub const fn new(x: i32, y: i32) -> Self {
        Pos { x, y, z: 0 }
    }

    pub const fn at(x: i32, y: i32, z: i32) -> Self {
        Pos { x, y, z }
    }

    /// The ground tile under this position. Map layers only exist down there.
    pub const fn ground(self) -> Pos {
        Pos::new(self.x, self.y)
    }

    /// The position `levels` above this one, or below if negative.
    pub const fn shifted(self, levels: i32) -> Pos {
        Pos::at(self.x, self.y, self.z + levels)
    }

    pub fn step(self, dir: Dir) -> Pos {
        let (dx, dy) = match dir {
            Dir::North => (0, -1),
            Dir::East => (1, 0),
            Dir::South => (0, 1),
            Dir::West => (-1, 0),
        };
        Pos::at(self.x + dx, self.y + dy, self.z)
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

    pub fn left(self) -> Dir {
        match self {
            Dir::North => Dir::West,
            Dir::East => Dir::North,
            Dir::South => Dir::East,
            Dir::West => Dir::South,
        }
    }

    pub fn right(self) -> Dir {
        self.left().opposite()
    }
}
