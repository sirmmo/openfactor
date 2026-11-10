//! Saving a world to bytes and loading it back.
//!
//! A save holds only the state that can't be worked out again: the map,
//! stock, research, buildings, what they hold and what is on the belts. Belt
//! lines and networks are rebuilt from the layout on load, which gives exactly
//! the lines the world had, because they were last rebuilt after its last
//! change. The loaded world has the same [`World::state_hash`] as the saved one
//! and continues identically.
//!
//! Numbers are written as LEB128 variable-length integers. Every value read
//! back is checked, so a damaged or hostile file is an error, never a panic.

use super::{Entity, Job, Kind, Machine, MetricValue, Run, Shared, World};
use crate::belt::BeltItem;
use crate::content::{Pack, Route};
use crate::hash::Hasher;
use crate::{BeltId, Dir, EntityId, ItemId, ItemStats, MetricId, Pos, TechId, UnitId};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt;

const MAGIC: &[u8; 4] = b"OFSV";
/// Version 2 added deliveries, sandboxes and puzzles. Version 1 still loads.
const VERSION: u64 = 2;

/// Why a save couldn't be loaded.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LoadError {
    /// Not a save, or cut short.
    Damaged(&'static str),
    /// Written by a newer version of the format.
    Version(u64),
    /// Saved with a rule pack whose rules differ from this one.
    OtherPack,
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoadError::Damaged(what) => write!(f, "the save is damaged ({what})"),
            LoadError::Version(version) => write!(f, "the save uses format {version}, newer than this game"),
            LoadError::OtherPack => write!(f, "the save was made with different rules"),
        }
    }
}

impl std::error::Error for LoadError {}

type Loaded<T> = Result<T, LoadError>;

struct Writer(Vec<u8>);

impl Writer {
    fn u(&mut self, mut value: u64) {
        loop {
            let byte = (value & 0x7f) as u8;
            value >>= 7;
            if value == 0 {
                self.0.push(byte);
                return;
            }
            self.0.push(byte | 0x80);
        }
    }

    fn i(&mut self, value: i32) {
        self.u(u64::from(((value << 1) ^ (value >> 31)) as u32));
    }

    fn pos(&mut self, pos: Pos) {
        self.i(pos.x);
        self.i(pos.y);
        self.i(pos.z);
    }

    fn option(&mut self, value: Option<u64>) {
        match value {
            None => self.u(0),
            Some(value) => {
                self.u(1);
                self.u(value);
            }
        }
    }
}

struct Reader<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl Reader<'_> {
    fn u(&mut self) -> Loaded<u64> {
        let mut value = 0u64;
        for shift in (0..64).step_by(7) {
            let byte = *self.bytes.get(self.at).ok_or(LoadError::Damaged("it ends early"))?;
            self.at += 1;
            value |= u64::from(byte & 0x7f) << shift;
            if byte & 0x80 == 0 {
                return Ok(value);
            }
        }
        Err(LoadError::Damaged("a number is too long"))
    }

    fn u32(&mut self) -> Loaded<u32> {
        u32::try_from(self.u()?).map_err(|_| LoadError::Damaged("a number is out of range"))
    }

    /// A count of things that follow, each taking at least one byte.
    fn len(&mut self) -> Loaded<usize> {
        let len = self.u()?;
        if len > (self.bytes.len() - self.at) as u64 {
            return Err(LoadError::Damaged("a count is larger than the file"));
        }
        Ok(len as usize)
    }

    /// An index below `limit`.
    fn index(&mut self, limit: usize, what: &'static str) -> Loaded<u16> {
        match self.u()? {
            value if value < limit as u64 => Ok(value as u16),
            _ => Err(LoadError::Damaged(what)),
        }
    }

    fn i(&mut self) -> Loaded<i32> {
        let raw = self.u32()?;
        Ok(((raw >> 1) as i32) ^ -((raw & 1) as i32))
    }

    fn pos(&mut self) -> Loaded<Pos> {
        Ok(Pos::at(self.i()?, self.i()?, self.i()?))
    }

    fn flag(&mut self) -> Loaded<bool> {
        match self.u()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(LoadError::Damaged("a flag is neither 0 nor 1")),
        }
    }

    fn option(&mut self) -> Loaded<Option<u64>> {
        match self.u()? {
            0 => Ok(None),
            1 => Ok(Some(self.u()?)),
            _ => Err(LoadError::Damaged("a flag is neither 0 nor 1")),
        }
    }
}

fn dir_code(dir: Dir) -> u64 {
    match dir {
        Dir::North => 0,
        Dir::East => 1,
        Dir::South => 2,
        Dir::West => 3,
    }
}

impl World {
    /// Fingerprint of the rules, so a save can't be loaded into other rules.
    fn content_fingerprint(&self) -> u64 {
        let mut h = Hasher::new();
        self.hash_content(&mut h);
        h.finish()
    }

    /// The whole state of the world as bytes. See [`World::load`].
    pub fn save(&self) -> Vec<u8> {
        debug_assert!(!self.topology_dirty, "commands only apply inside a step");
        let mut w = Writer(MAGIC.to_vec());
        w.u(VERSION);
        w.u(self.content_fingerprint());
        w.u(self.tick);
        w.u(u64::from(self.next_id));

        let shared = &self.shared;
        for tiles in &shared.layers {
            w.u(tiles.len() as u64);
            for (&pos, &value) in tiles {
                w.pos(pos);
                w.u(value);
            }
        }
        w.u(shared.globals.len() as u64);
        for (item, &value) in &shared.globals {
            w.u(u64::from(item.0));
            w.u(value);
        }
        w.u(shared.metrics.len() as u64);
        for (metric, value) in &shared.metrics {
            w.u(u64::from(metric.0));
            w.u(value.total);
            w.u(value.last_tick);
        }
        w.u(shared.stats.len() as u64);
        for (item, stats) in &shared.stats {
            w.u(u64::from(item.0));
            for value in [stats.produced, stats.consumed, stats.destroyed, stats.granted, stats.spent, stats.delivered] {
                w.u(value);
            }
        }
        w.u(u64::from(shared.free));
        match &self.puzzle {
            None => w.u(0),
            Some(run) => {
                w.u(1);
                w.u(u64::from(run.puzzle));
                w.u(run.started);
                run.base.iter().for_each(|&base| w.u(base));
                w.option(run.solved);
                w.u(u64::from(run.failed));
            }
        }
        w.u(shared.charge.len() as u64);
        for (&(resource, id), &held) in &shared.charge {
            w.u(u64::from(resource.0));
            w.u(u64::from(id.0));
            w.u(held);
        }
        w.u(shared.researched.len() as u64);
        shared.researched.iter().for_each(|tech| w.u(u64::from(tech.0)));
        match shared.research {
            None => w.u(0),
            Some((tech, remaining)) => {
                w.u(1);
                w.u(u64::from(tech.0));
                w.u(u64::from(remaining));
            }
        }

        w.u((self.machines.len() + self.belts.len()) as u64);
        for (id, entity) in self.machines.iter().chain(&self.belts) {
            w.u(u64::from(id.0));
            w.pos(entity.pos);
            w.u(dir_code(entity.dir));
            match &entity.kind {
                Kind::Belt(belt) => {
                    w.u(0);
                    w.u(u64::from(belt.0));
                }
                Kind::Machine(machine) => {
                    w.u(1);
                    w.u(u64::from(machine.unit.0));
                    w.option(machine.setting.map(u64::from));
                    w.u(u64::from(machine.next_port));
                    w.option(machine.job.map(|job| job.rule as u64));
                    if let Some(job) = machine.job {
                        w.u(u64::from(job.remaining));
                    }
                    w.u(machine.bins.len() as u64);
                    for (item, &count) in &machine.bins {
                        w.u(u64::from(item.0));
                        w.u(count);
                    }
                }
            }
        }

        // Lines are rebuilt from the layout, so only what moves on them is kept.
        w.u(self.lines.len() as u64);
        for line in &self.lines {
            w.u(line.tiles.len() as u64);
            w.u(line.compressed as u64);
            w.option(line.last_feeder.map(|id| u64::from(id.0)));
            w.u(line.items.len() as u64);
            for item in &line.items {
                w.u(u64::from(item.item.0));
                w.u(u64::from(item.gap));
            }
        }
        w.0
    }

    /// Rebuilds a world saved with [`World::save`] under the same rules.
    pub fn load(pack: &Pack, bytes: &[u8]) -> Result<World, LoadError> {
        if bytes.len() < MAGIC.len() || &bytes[..MAGIC.len()] != MAGIC {
            return Err(LoadError::Damaged("it isn't an OpenFactor save"));
        }
        let mut r = Reader { bytes, at: MAGIC.len() };
        let version = r.u()?;
        if version == 0 || version > VERSION {
            return Err(LoadError::Version(version));
        }
        let mut world = World::new(pack);
        if r.u()? != world.content_fingerprint() {
            return Err(LoadError::OtherPack);
        }
        let content = world.content.clone();
        let (items, metrics, techs) = (content.resources.len(), content.metrics.len(), content.techs.len());
        world.tick = r.u()?;
        world.next_id = r.u32()?;

        let mut shared = Shared {
            layers: Vec::new(),
            globals: BTreeMap::new(),
            metrics: BTreeMap::new(),
            stats: BTreeMap::new(),
            networks: Vec::new(),
            charge: BTreeMap::new(),
            researched: BTreeSet::new(),
            research: None,
            free: false,
        };
        for _ in 0..content.layers.len() {
            let mut tiles = BTreeMap::new();
            for _ in 0..r.len()? {
                let pos = r.pos()?;
                match r.u()? {
                    0 => return Err(LoadError::Damaged("a map tile is empty")),
                    value => tiles.insert(pos, value),
                };
            }
            shared.layers.push(tiles);
        }
        for _ in 0..r.len()? {
            let item = ItemId(r.index(items, "unknown resource in the stock")?);
            shared.globals.insert(item, r.u()?);
        }
        for _ in 0..r.len()? {
            let metric = MetricId(r.index(metrics, "unknown metric")?);
            shared.metrics.insert(metric, MetricValue { total: r.u()?, last_tick: r.u()? });
        }
        for _ in 0..r.len()? {
            let item = ItemId(r.index(items, "unknown resource in the totals")?);
            let mut stats = ItemStats { produced: r.u()?, consumed: r.u()?, destroyed: r.u()?, granted: r.u()?, spent: r.u()?, delivered: 0 };
            if version >= 2 {
                stats.delivered = r.u()?;
            }
            shared.stats.insert(item, stats);
        }
        if version >= 2 {
            shared.free = r.flag()?;
            world.puzzle = match r.u()? {
                0 => None,
                1 => {
                    let puzzle = r.index(content.puzzles.len(), "unknown puzzle")?;
                    let started = r.u()?;
                    let goals = content.puzzles[usize::from(puzzle)].goals.len();
                    let base = (0..goals).map(|_| r.u()).collect::<Loaded<Vec<u64>>>()?;
                    let solved = r.option()?;
                    let failed = r.flag()?;
                    if started > world.tick || solved.map_or(false, |tick| tick < started || tick > world.tick) {
                        return Err(LoadError::Damaged("the puzzle's times are impossible"));
                    }
                    Some(Run { puzzle, started, base, solved, failed })
                }
                _ => return Err(LoadError::Damaged("the puzzle is unreadable")),
            };
        }
        for _ in 0..r.len()? {
            let resource = ItemId(r.index(items, "unknown network resource")?);
            let id = EntityId(r.u32()?);
            shared.charge.insert((resource, id), r.u()?);
        }
        for _ in 0..r.len()? {
            shared.researched.insert(TechId(r.index(techs, "unknown research")?));
        }
        shared.research = match r.u()? {
            0 => None,
            1 => Some((TechId(r.index(techs, "unknown research")?), r.u32()?)),
            _ => return Err(LoadError::Damaged("the running research is unreadable")),
        };
        world.shared = shared;

        for _ in 0..r.len()? {
            let id = EntityId(r.u32()?);
            if id.0 >= world.next_id {
                return Err(LoadError::Damaged("a building has an id from the future"));
            }
            let pos = r.pos()?;
            if pos.z < 0 || pos.z >= content.levels as i32 {
                return Err(LoadError::Damaged("a building is on a level that doesn't exist"));
            }
            let dir = match r.u()? {
                0 => Dir::North,
                1 => Dir::East,
                2 => Dir::South,
                3 => Dir::West,
                _ => return Err(LoadError::Damaged("a building faces no direction")),
            };
            let kind = match r.u()? {
                0 => Kind::Belt(BeltId(r.index(content.belts.len(), "unknown belt")?)),
                1 => {
                    let unit = UnitId(r.index(content.units.len(), "unknown unit")?);
                    let def = content.unit(unit);
                    let choices = match def.route {
                        Some(Route::Filter) => items,
                        _ => def.rules.len(),
                    };
                    let setting = match r.option()? {
                        Some(value) if value < choices as u64 => Some(value as u16),
                        Some(_) => return Err(LoadError::Damaged("a building has an impossible setting")),
                        None => None,
                    };
                    let next_port = match r.u()? {
                        value @ 0..=2 => value as u8,
                        _ => return Err(LoadError::Damaged("a building's next side is unreadable")),
                    };
                    let job = match r.option()? {
                        Some(rule) if rule < def.rules.len() as u64 => Some(Job { rule: rule as usize, remaining: r.u32()? }),
                        Some(_) => return Err(LoadError::Damaged("a building runs a recipe it doesn't have")),
                        None => None,
                    };
                    let mut bins = BTreeMap::new();
                    for _ in 0..r.len()? {
                        let item = ItemId(r.index(items, "unknown resource in a building")?);
                        match r.u()? {
                            0 => return Err(LoadError::Damaged("a building holds an empty stack")),
                            count => bins.insert(item, count),
                        };
                    }
                    Kind::Machine(Machine { unit, bins, job, setting, next_port })
                }
                _ => return Err(LoadError::Damaged("a building is neither a belt nor a unit")),
            };
            if world.tiles.insert(pos, id).is_some() {
                return Err(LoadError::Damaged("two buildings share a tile"));
            }
            if world.machines.contains_key(&id) || world.belts.contains_key(&id) {
                return Err(LoadError::Damaged("two buildings share an id"));
            }
            let entity = Entity { pos, dir, kind, targets: [None; 3], nets: Vec::new() };
            match entity.kind {
                Kind::Belt(_) => world.belts.insert(id, entity),
                Kind::Machine(_) => world.machines.insert(id, entity),
            };
        }
        if world.shared.charge.keys().any(|(_, id)| !world.machines.contains_key(id)) {
            return Err(LoadError::Damaged("a missing building holds network stock"));
        }

        world.rebuild_lines();
        world.rebuild_networks();
        if r.len()? != world.lines.len() {
            return Err(LoadError::Damaged("the belts don't match the buildings"));
        }
        for line in &mut world.lines {
            if r.u()? != line.tiles.len() as u64 {
                return Err(LoadError::Damaged("a belt line has the wrong length"));
            }
            let compressed = r.u()?;
            let last_feeder = r.option()?.map(|id| u32::try_from(id).map(EntityId));
            line.last_feeder = match last_feeder {
                Some(Ok(id)) => Some(id),
                Some(Err(_)) => return Err(LoadError::Damaged("a belt's last feeder is unreadable")),
                None => None,
            };
            let mut used = 0u64;
            let mut queue = VecDeque::new();
            for _ in 0..r.len()? {
                let item = ItemId(r.index(items, "unknown resource on a belt")?);
                let gap = r.u32()?;
                used += u64::from(gap);
                queue.push_back(BeltItem { item, gap });
            }
            let length = u64::from(line.length());
            if used > length || compressed > queue.len() as u64 {
                return Err(LoadError::Damaged("a belt holds more than fits on it"));
            }
            line.tail_free = (length - used) as u32;
            line.compressed = compressed as usize;
            line.items = queue;
        }
        if r.at != bytes.len() {
            return Err(LoadError::Damaged("there is more after the end"));
        }
        Ok(world)
    }
}
