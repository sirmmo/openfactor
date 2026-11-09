use crate::belt::{BeltItem, Line, Target, TILE};
use crate::content::{Amount, Content, Pack, Rule, Scope};
use crate::hash::Hasher;
use crate::{Dir, EntityId, ItemId, LayerId, Pos, Tick, UnitId};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Building {
    Belt,
    Unit(UnitId),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Command {
    Place { pos: Pos, dir: Dir, building: Building },
    Remove { pos: Pos },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reject {
    Occupied,
    NoResource,
    UnknownUnit,
    NothingToRemove,
}

/// A command that had no effect. Rejection is deterministic like everything else.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rejected {
    pub index: usize,
    pub reason: Reject,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ItemStats {
    pub produced: u64,
    pub consumed: u64,
    pub destroyed: u64,
}

#[derive(Clone, Debug)]
struct Entity {
    pos: Pos,
    dir: Dir,
    kind: Kind,
    /// Where a unit sends its output. Derived from the layout.
    output_target: Option<Target>,
}

#[derive(Clone, Debug)]
enum Kind {
    Belt,
    Machine(Machine),
}

#[derive(Clone, Debug)]
struct Machine {
    unit: UnitId,
    bins: BTreeMap<ItemId, u64>,
    job: Option<Job>,
}

#[derive(Clone, Copy, Debug)]
struct Job {
    rule: usize,
    remaining: u32,
}

#[derive(Clone, Copy)]
enum Source {
    Line(u32),
    Machine(EntityId),
}

#[derive(Clone, Copy)]
struct Offer {
    key: EntityId,
    item: ItemId,
    source: Source,
    target: Target,
}

type Layer = BTreeMap<Pos, u64>;
type Stats = BTreeMap<ItemId, ItemStats>;

#[derive(Clone, Debug)]
pub struct World {
    content: Content,
    tick: Tick,
    next_id: u32,
    layers: Vec<Layer>,
    globals: BTreeMap<ItemId, u64>,
    tiles: BTreeMap<Pos, EntityId>,
    /// Everything that isn't a belt. Kept apart so a tick never visits belts.
    machines: BTreeMap<EntityId, Entity>,
    belts: BTreeMap<EntityId, Entity>,
    lines: Vec<Line>,
    stats: Stats,
    topology_dirty: bool,
}

fn stat(stats: &mut Stats, item: ItemId) -> &mut ItemStats {
    stats.entry(item).or_default()
}

fn layer_at(layers: &[Layer], layer: LayerId, pos: Pos) -> u64 {
    layers[usize::from(layer.0)].get(&pos).copied().unwrap_or(0)
}

fn can_start(rule: &Rule, machine: &Machine, layers: &[Layer], globals: &BTreeMap<ItemId, u64>, pos: Pos) -> bool {
    let conditions_hold = rule
        .conditions
        .iter()
        .all(|c| c.test.holds(layer_at(layers, c.layer, pos), c.value));
    let has_inputs = rule.takes.iter().all(|take| {
        let needed = u64::from(take.amount);
        match take.scope {
            Scope::Local(item) => machine.bins.get(&item).copied().unwrap_or(0) >= needed,
            Scope::Global(item) => globals.get(&item).copied().unwrap_or(0) >= needed,
            Scope::Map { layer, .. } => layer_at(layers, layer, pos) >= needed,
        }
    });
    // A unit doesn't start again until what it last made has left.
    let output_clear = rule.gives.iter().all(|give| match give.scope {
        Scope::Local(item) => machine.bins.get(&item).copied().unwrap_or(0) == 0,
        _ => true,
    });
    conditions_hold && has_inputs && output_clear
}

fn subtract(map: &mut BTreeMap<ItemId, u64>, item: ItemId, amount: u64) {
    let held = map.get_mut(&item).expect("checked by can_start");
    *held -= amount;
    if *held == 0 {
        map.remove(&item);
    }
}

fn apply_takes(rule: &Rule, machine: &mut Machine, layers: &mut [Layer], globals: &mut BTreeMap<ItemId, u64>, stats: &mut Stats, pos: Pos) {
    for &Amount { scope, amount } in &rule.takes {
        let amount_wide = u64::from(amount);
        match scope {
            Scope::Local(item) => {
                subtract(&mut machine.bins, item, amount_wide);
                stat(stats, item).consumed += amount_wide;
            }
            Scope::Global(item) => subtract(globals, item, amount_wide),
            Scope::Map { layer, .. } => {
                let tiles = &mut layers[usize::from(layer.0)];
                let value = tiles.get_mut(&pos).expect("checked by can_start");
                *value -= amount_wide;
                if *value == 0 {
                    tiles.remove(&pos);
                }
            }
        }
    }
}

fn apply_gives(rule: &Rule, machine: &mut Machine, layers: &mut [Layer], globals: &mut BTreeMap<ItemId, u64>, stats: &mut Stats, pos: Pos) {
    for &Amount { scope, amount } in &rule.gives {
        let amount_wide = u64::from(amount);
        match scope {
            Scope::Local(item) => {
                *machine.bins.entry(item).or_default() += amount_wide;
                stat(stats, item).produced += amount_wide;
            }
            Scope::Global(item) => *globals.entry(item).or_default() += amount_wide,
            Scope::Map { layer, radius } => {
                let tiles = &mut layers[usize::from(layer.0)];
                let reach = radius as i32;
                for dx in -reach..=reach {
                    let spread = reach - dx.abs();
                    for dy in -spread..=spread {
                        *tiles.entry(Pos::new(pos.x + dx, pos.y + dy)).or_default() += amount_wide;
                    }
                }
            }
        }
    }
}

impl World {
    pub fn new(pack: &Pack) -> World {
        let content = pack.content.clone();
        let mut layers = vec![Layer::new(); content.layers.len()];
        for patch in &pack.worldgen.patches {
            let tiles = &mut layers[usize::from(patch.layer.0)];
            for dx in 0..patch.width as i32 {
                for dy in 0..patch.height as i32 {
                    let pos = Pos::new(patch.origin.x + dx, patch.origin.y + dy);
                    tiles.insert(pos, u64::from(patch.amount_per_tile));
                }
            }
        }
        World {
            content,
            tick: 0,
            next_id: 0,
            layers,
            globals: BTreeMap::new(),
            tiles: BTreeMap::new(),
            machines: BTreeMap::new(),
            belts: BTreeMap::new(),
            lines: Vec::new(),
            stats: BTreeMap::new(),
            topology_dirty: false,
        }
    }

    pub fn content(&self) -> &Content {
        &self.content
    }

    pub fn tick(&self) -> Tick {
        self.tick
    }

    /// Advances one tick. Commands apply in slice order, so the server's
    /// ordering is what makes them agree across clients.
    pub fn step(&mut self, commands: &[Command]) -> Vec<Rejected> {
        let mut rejected = Vec::new();
        for (index, command) in commands.iter().enumerate() {
            if let Err(reason) = self.apply(command) {
                rejected.push(Rejected { index, reason });
            }
        }
        if self.topology_dirty {
            self.rebuild_lines();
        }
        let speed = self.content.belt_speed;
        for line in &mut self.lines {
            line.advance(speed);
        }
        self.transfer_items();
        self.run_machines();
        self.tick += 1;
        rejected
    }

    fn apply(&mut self, command: &Command) -> Result<(), Reject> {
        match *command {
            Command::Place { pos, dir, building } => self.place(pos, dir, building),
            Command::Remove { pos } => self.remove(pos),
        }
    }

    fn place(&mut self, pos: Pos, dir: Dir, building: Building) -> Result<(), Reject> {
        if self.tiles.contains_key(&pos) {
            return Err(Reject::Occupied);
        }
        let kind = match building {
            Building::Belt => Kind::Belt,
            Building::Unit(unit) => {
                let definition =
                    self.content.units.get(usize::from(unit.0)).ok_or(Reject::UnknownUnit)?;
                if let Some(layer) = definition.place_on {
                    if layer_at(&self.layers, layer, pos) == 0 {
                        return Err(Reject::NoResource);
                    }
                }
                Kind::Machine(Machine { unit, bins: BTreeMap::new(), job: None })
            }
        };
        let id = EntityId(self.next_id);
        self.next_id += 1;
        self.tiles.insert(pos, id);
        let map = if matches!(kind, Kind::Belt) { &mut self.belts } else { &mut self.machines };
        map.insert(id, Entity { pos, dir, kind, output_target: None });
        self.topology_dirty = true;
        Ok(())
    }

    fn remove(&mut self, pos: Pos) -> Result<(), Reject> {
        let id = self.tiles.remove(&pos).ok_or(Reject::NothingToRemove)?;
        let entity = self
            .machines
            .remove(&id)
            .or_else(|| self.belts.remove(&id))
            .expect("tile points at a live entity");
        // Items on a removed belt are accounted for in `rebuild_lines`.
        if let Kind::Machine(machine) = &entity.kind {
            for (&item, &count) in &machine.bins {
                stat(&mut self.stats, item).destroyed += count;
            }
        }
        self.topology_dirty = true;
        Ok(())
    }

    fn belt_dir(&self, pos: Pos) -> Option<Dir> {
        Some(self.belts.get(self.tiles.get(&pos)?)?.dir)
    }

    /// The belt tile an entity at `pos` facing `dir` would feed, if any.
    fn fed_belt(&self, pos: Pos, dir: Dir) -> Option<Pos> {
        let front = pos.step(dir);
        let front_dir = self.belt_dir(front)?;
        (front_dir != dir.opposite()).then_some(front)
    }

    fn sends_items(&self, entity: &Entity) -> bool {
        match &entity.kind {
            Kind::Belt => true,
            Kind::Machine(machine) => !self.content.unit(machine.unit).outputs.is_empty(),
        }
    }

    /// Regroups belt tiles into lines, keeping every item where it physically is.
    fn rebuild_lines(&mut self) {
        let mut carried: BTreeMap<Pos, Vec<(u32, ItemId)>> = BTreeMap::new();
        for line in &self.lines {
            for (pos, offset, item) in line.placed_items() {
                carried.entry(pos).or_default().push((offset, item));
            }
        }

        // (belt feeders, machine feeders) for each belt tile.
        let mut feeders: BTreeMap<Pos, (u32, u32)> = BTreeMap::new();
        for entity in self.machines.values().chain(self.belts.values()) {
            if !self.sends_items(entity) {
                continue;
            }
            if let Some(front) = self.fed_belt(entity.pos, entity.dir) {
                let count = feeders.entry(front).or_default();
                match entity.kind {
                    Kind::Belt => count.0 += 1,
                    _ => count.1 += 1,
                }
            }
        }

        let belts: Vec<Pos> = self
            .tiles
            .keys()
            .copied()
            .filter(|&pos| self.belt_dir(pos).is_some())
            .collect();
        // A tile continues its feeder's line only when that belt is its sole input.
        let continues = |pos: Pos| feeders.get(&pos).copied() == Some((1, 0));

        let mut visited = BTreeSet::new();
        let mut chains: Vec<Vec<Pos>> = Vec::new();
        let starts = belts.iter().copied().filter(|&pos| !continues(pos));
        // Tiles not reached from a start can only be closed rings of belts.
        for start in starts.chain(belts.iter().copied()) {
            if !visited.insert(start) {
                continue;
            }
            let mut chain = vec![start];
            let mut current = start;
            while let Some(next) = self.fed_belt(current, self.belt_dir(current).unwrap()) {
                if !continues(next) || !visited.insert(next) {
                    break;
                }
                chain.push(next);
                current = next;
            }
            chains.push(chain);
        }

        let mut line_at = BTreeMap::new();
        for (index, chain) in chains.iter().enumerate() {
            for &pos in chain {
                line_at.insert(pos, index as u32);
            }
        }

        let target_of = |world: &World, pos: Pos, dir: Dir| -> Option<Target> {
            if let Some(front) = world.fed_belt(pos, dir) {
                return Some(Target::Line(line_at[&front]));
            }
            let id = *world.tiles.get(&pos.step(dir))?;
            match &world.machines.get(&id)?.kind {
                Kind::Machine(machine) if world.content.unit(machine.unit).accepts_items() => {
                    Some(Target::Entity(id))
                }
                _ => None,
            }
        };

        let mut lines = Vec::with_capacity(chains.len());
        for tiles in chains {
            let head = *tiles.last().expect("chains are never empty");
            let length = tiles.len() as u32 * TILE;
            let mut positioned: Vec<(u32, ItemId)> = Vec::new();
            for (index, pos) in tiles.iter().enumerate() {
                for (offset, item) in carried.remove(pos).unwrap_or_default() {
                    positioned.push((index as u32 * TILE + offset, item));
                }
            }
            // Stable, so items that share a position keep a deterministic order.
            positioned.sort_by_key(|&(from_start, _)| std::cmp::Reverse(from_start));

            let mut items = VecDeque::with_capacity(positioned.len());
            let mut previous = length;
            for (from_start, item) in positioned {
                items.push_back(BeltItem { item, gap: previous - from_start });
                previous = from_start;
            }
            lines.push(Line {
                head_entity: self.tiles[&head],
                target: target_of(self, head, self.belt_dir(head).unwrap()),
                tiles,
                items,
                tail_free: previous,
                compressed: 0,
                last_feeder: None,
            });
        }

        for items in carried.into_values() {
            for (_, item) in items {
                stat(&mut self.stats, item).destroyed += 1;
            }
        }

        let output_targets: Vec<Option<Target>> = self
            .machines
            .values()
            .map(|entity| target_of(self, entity.pos, entity.dir))
            .collect();
        for (entity, target) in self.machines.values_mut().zip(output_targets) {
            entity.output_target = target;
        }

        self.lines = lines;
        self.topology_dirty = false;
    }

    /// The next item a machine wants to send, if it has one.
    fn output_item(&self, machine: &Machine) -> Option<ItemId> {
        let outputs = &self.content.unit(machine.unit).outputs;
        outputs.iter().copied().find(|item| machine.bins.contains_key(item))
    }

    fn collect_offers(&self) -> Vec<Offer> {
        let mut offers = Vec::new();
        for (index, line) in self.lines.iter().enumerate() {
            if let (Some(item), Some(target)) = (line.head_item(), line.target) {
                let source = Source::Line(index as u32);
                offers.push(Offer { key: line.head_entity, item, source, target });
            }
        }
        for (&id, entity) in &self.machines {
            let item = match &entity.kind {
                Kind::Machine(machine) => self.output_item(machine),
                Kind::Belt => None,
            };
            if let (Some(item), Some(target)) = (item, entity.output_target) {
                offers.push(Offer { key: id, item, source: Source::Machine(id), target });
            }
        }
        offers.sort_by_key(|offer| offer.key);
        offers
    }

    fn transfer_items(&mut self) {
        // A line takes at most one item per tick; feeders take turns in id order.
        let mut winners: BTreeMap<u32, ((bool, EntityId), Offer)> = BTreeMap::new();
        for offer in self.collect_offers() {
            match offer.target {
                Target::Entity(id) => {
                    if self.accepts(id, offer.item) {
                        self.take(offer.source, offer.item);
                        self.store(id, offer.item);
                    }
                }
                Target::Line(index) => {
                    let line = &self.lines[index as usize];
                    if !line.can_accept() {
                        continue;
                    }
                    let had_turn = line.last_feeder.map_or(false, |last| offer.key <= last);
                    let rank = (had_turn, offer.key);
                    let entry = winners.entry(index).or_insert((rank, offer));
                    if rank < entry.0 {
                        *entry = (rank, offer);
                    }
                }
            }
        }
        for (index, (_, offer)) in winners {
            self.take(offer.source, offer.item);
            let line = &mut self.lines[index as usize];
            line.push_tail(offer.item);
            line.last_feeder = Some(offer.key);
        }
    }

    fn accepts(&self, id: EntityId, item: ItemId) -> bool {
        let machine = match &self.machines[&id].kind {
            Kind::Machine(machine) => machine,
            Kind::Belt => return false,
        };
        let unit = self.content.unit(machine.unit);
        if unit.stores_any {
            return true;
        }
        let held = machine.bins.get(&item).copied().unwrap_or(0);
        unit.inputs.iter().any(|&(input, limit)| input == item && held < limit)
    }

    fn machine_mut(&mut self, id: EntityId) -> &mut Machine {
        match &mut self.machines.get_mut(&id).expect("id is live").kind {
            Kind::Machine(machine) => machine,
            Kind::Belt => unreachable!("belts are stored separately"),
        }
    }

    fn store(&mut self, id: EntityId, item: ItemId) {
        *self.machine_mut(id).bins.entry(item).or_default() += 1;
    }

    fn take(&mut self, source: Source, item: ItemId) {
        match source {
            Source::Line(index) => self.lines[index as usize].pop_head(),
            Source::Machine(id) => subtract(&mut self.machine_mut(id).bins, item, 1),
        }
    }

    fn run_machines(&mut self) {
        let content = &self.content;
        let layers = &mut self.layers;
        let globals = &mut self.globals;
        let stats = &mut self.stats;
        for entity in self.machines.values_mut() {
            let pos = entity.pos;
            let machine = match &mut entity.kind {
                Kind::Machine(machine) => machine,
                Kind::Belt => continue,
            };
            let rules = &content.unit(machine.unit).rules;
            if let Some(job) = &mut machine.job {
                job.remaining -= 1;
                if job.remaining == 0 {
                    let rule = &rules[job.rule];
                    machine.job = None;
                    apply_gives(rule, machine, layers, globals, stats, pos);
                }
                continue;
            }
            // Rules are tried in the order the pack lists them.
            let ready = rules.iter().position(|r| can_start(r, machine, layers, globals, pos));
            if let Some(index) = ready {
                apply_takes(&rules[index], machine, layers, globals, stats, pos);
                machine.job = Some(Job { rule: index, remaining: rules[index].rate });
            }
        }
    }

    /// Every building, for rendering. Machines come first, then belts.
    pub fn buildings(&self) -> impl Iterator<Item = (Pos, Dir, Building)> + '_ {
        self.machines.values().chain(self.belts.values()).map(|entity| {
            let building = match &entity.kind {
                Kind::Belt => Building::Belt,
                Kind::Machine(machine) => Building::Unit(machine.unit),
            };
            (entity.pos, entity.dir, building)
        })
    }

    /// Every non-zero map tile as (position, layer, value).
    pub fn layer_tiles(&self) -> impl Iterator<Item = (Pos, LayerId, u64)> + '_ {
        self.layers.iter().enumerate().flat_map(|(index, tiles)| {
            tiles.iter().map(move |(pos, value)| (*pos, LayerId(index as u16), *value))
        })
    }

    pub fn layer_value(&self, layer: LayerId, pos: Pos) -> u64 {
        layer_at(&self.layers, layer, pos)
    }

    /// The world-wide stock of a resource moved by `global` rules.
    pub fn global(&self, item: ItemId) -> u64 {
        self.globals.get(&item).copied().unwrap_or(0)
    }

    /// Every belt item as (tile, tile direction, offset along it in 0..=`TILE`, item).
    pub fn belt_items(&self) -> impl Iterator<Item = (Pos, Dir, u32, ItemId)> + '_ {
        self.lines.iter().flat_map(move |line| {
            line.placed_items().map(move |(pos, offset, item)| {
                (pos, self.belt_dir(pos).expect("line tiles are belts"), offset, item)
            })
        })
    }

    pub fn stats(&self, item: ItemId) -> ItemStats {
        self.stats.get(&item).copied().unwrap_or_default()
    }

    /// Number of items currently on belts.
    pub fn belt_item_count(&self) -> usize {
        self.lines.iter().map(|line| line.items.len()).sum()
    }

    /// Number of belt lines after the most recent step.
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    /// How many of `item` the unit at `pos` holds.
    pub fn stock(&self, pos: Pos, item: ItemId) -> u64 {
        let entity = self.tiles.get(&pos).and_then(|id| self.machines.get(id));
        match entity.map(|entity| &entity.kind) {
            Some(Kind::Machine(machine)) => machine.bins.get(&item).copied().unwrap_or(0),
            _ => 0,
        }
    }

    /// Count of every item that physically exists in the world.
    pub fn item_totals(&self) -> BTreeMap<ItemId, u64> {
        let mut totals: BTreeMap<ItemId, u64> = BTreeMap::new();
        for line in &self.lines {
            for belt_item in &line.items {
                *totals.entry(belt_item.item).or_default() += 1;
            }
        }
        for entity in self.machines.values() {
            if let Kind::Machine(machine) = &entity.kind {
                for (&item, &count) in &machine.bins {
                    *totals.entry(item).or_default() += count;
                }
            }
        }
        totals
    }

    /// Hash of the full simulation state, for comparing peers and replays.
    pub fn state_hash(&self) -> u64 {
        let mut h = Hasher::new();
        h.write(self.tick);
        h.write(u64::from(self.next_id));
        self.hash_content(&mut h);
        for tiles in &self.layers {
            h.write(tiles.len() as u64);
            for (pos, value) in tiles {
                hash_pos(&mut h, *pos);
                h.write(*value);
            }
        }
        h.write(self.globals.len() as u64);
        for (item, value) in &self.globals {
            h.write(u64::from(item.0));
            h.write(*value);
        }
        for entities in [&self.machines, &self.belts] {
            h.write(entities.len() as u64);
            for (id, entity) in entities {
                h.write(u64::from(id.0));
                hash_pos(&mut h, entity.pos);
                h.write(entity.dir as u64);
                hash_kind(&mut h, &entity.kind);
            }
        }
        h.write(self.lines.len() as u64);
        for line in &self.lines {
            hash_line(&mut h, line);
        }
        h.write(self.stats.len() as u64);
        for (item, stats) in &self.stats {
            h.write(u64::from(item.0));
            h.write(stats.produced);
            h.write(stats.consumed);
            h.write(stats.destroyed);
        }
        h.finish()
    }

    /// Covers everything that affects the simulation. Names and sprites are
    /// left out, so a cosmetic difference between packs is not a desync.
    fn hash_content(&self, h: &mut Hasher) {
        h.write(u64::from(self.content.belt_speed));
        h.write(self.content.resources.len() as u64);
        for resource in &self.content.resources {
            h.write_str(&resource.id);
        }
        h.write(self.content.layers.len() as u64);
        for layer in &self.content.layers {
            h.write_str(&layer.id);
        }
        h.write(self.content.units.len() as u64);
        for unit in &self.content.units {
            h.write_str(&unit.id);
            h.write(unit.place_on.map_or(u64::MAX, |layer| u64::from(layer.0)));
            h.write(u64::from(unit.stores_any));
            h.write(unit.rules.len() as u64);
            for rule in &unit.rules {
                hash_rule(h, rule);
            }
        }
    }
}

fn hash_rule(h: &mut Hasher, rule: &Rule) {
    h.write(u64::from(rule.rate));
    h.write(rule.conditions.len() as u64);
    for condition in &rule.conditions {
        h.write(u64::from(condition.layer.0));
        h.write(condition.test as u64);
        h.write(u64::from(condition.value));
    }
    for list in [&rule.takes, &rule.gives] {
        h.write(list.len() as u64);
        for entry in list {
            match entry.scope {
                Scope::Local(item) => {
                    h.write(0);
                    h.write(u64::from(item.0));
                }
                Scope::Global(item) => {
                    h.write(1);
                    h.write(u64::from(item.0));
                }
                Scope::Map { layer, radius } => {
                    h.write(2);
                    h.write(u64::from(layer.0));
                    h.write(u64::from(radius));
                }
            }
            h.write(u64::from(entry.amount));
        }
    }
}

fn hash_pos(h: &mut Hasher, pos: Pos) {
    h.write_i32(pos.x);
    h.write_i32(pos.y);
}

fn hash_kind(h: &mut Hasher, kind: &Kind) {
    match kind {
        Kind::Belt => h.write(0),
        Kind::Machine(machine) => {
            h.write(1);
            h.write(u64::from(machine.unit.0));
            match machine.job {
                None => h.write(u64::MAX),
                Some(job) => {
                    h.write(job.rule as u64);
                    h.write(u64::from(job.remaining));
                }
            }
            h.write(machine.bins.len() as u64);
            for (item, count) in &machine.bins {
                h.write(u64::from(item.0));
                h.write(*count);
            }
        }
    }
}

fn hash_line(h: &mut Hasher, line: &Line) {
    h.write(line.tiles.len() as u64);
    line.tiles.iter().for_each(|&pos| hash_pos(h, pos));
    h.write(u64::from(line.tail_free));
    h.write(line.compressed as u64);
    h.write(line.last_feeder.map_or(u64::MAX, |id| u64::from(id.0)));
    match line.target {
        None => h.write(0),
        Some(Target::Line(index)) => {
            h.write(1);
            h.write(u64::from(index));
        }
        Some(Target::Entity(id)) => {
            h.write(2);
            h.write(u64::from(id.0));
        }
    }
    h.write(line.items.len() as u64);
    for item in &line.items {
        h.write(u64::from(item.item.0));
        h.write(u64::from(item.gap));
    }
}
