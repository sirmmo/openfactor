use crate::belt::{BeltItem, Line, Target, TILE};
use crate::content::{Content, ContentError, WorldGen};
use crate::hash::Hasher;
use crate::{Dir, EntityId, ItemId, Pos, RecipeId, Tick};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Building {
    Belt,
    Miner,
    Assembler { recipe: RecipeId },
    Depot,
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
    UnknownRecipe,
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
struct Resource {
    item: ItemId,
    amount: u32,
}

#[derive(Clone, Debug)]
struct Entity {
    pos: Pos,
    dir: Dir,
    kind: Kind,
    /// Where a miner or assembler sends its output. Derived from the layout.
    output_target: Option<Target>,
}

#[derive(Clone, Debug)]
enum Kind {
    Belt,
    Miner {
        progress: u32,
        output: Option<ItemId>,
    },
    Assembler {
        recipe: RecipeId,
        /// Parallel to the recipe's inputs.
        stock: Vec<u32>,
        remaining: u32,
        pending: VecDeque<ItemId>,
    },
    Depot {
        stock: BTreeMap<ItemId, u64>,
    },
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

#[derive(Clone, Debug)]
pub struct World {
    content: Content,
    tick: Tick,
    next_id: u32,
    resources: BTreeMap<Pos, Resource>,
    tiles: BTreeMap<Pos, EntityId>,
    /// Everything that isn't a belt. Kept apart so a tick never visits belts.
    machines: BTreeMap<EntityId, Entity>,
    belts: BTreeMap<EntityId, Entity>,
    lines: Vec<Line>,
    stats: BTreeMap<ItemId, ItemStats>,
    topology_dirty: bool,
}

fn stat(stats: &mut BTreeMap<ItemId, ItemStats>, item: ItemId) -> &mut ItemStats {
    stats.entry(item).or_default()
}

impl World {
    pub fn new(content: Content, worldgen: &WorldGen) -> Result<World, ContentError> {
        content.validate(worldgen)?;
        let mut resources = BTreeMap::new();
        for patch in &worldgen.patches {
            for dx in 0..patch.width as i32 {
                for dy in 0..patch.height as i32 {
                    let pos = Pos::new(patch.origin.x + dx, patch.origin.y + dy);
                    let amount = patch.amount_per_tile;
                    resources.insert(pos, Resource { item: patch.item, amount });
                }
            }
        }
        Ok(World {
            content,
            tick: 0,
            next_id: 0,
            resources,
            tiles: BTreeMap::new(),
            machines: BTreeMap::new(),
            belts: BTreeMap::new(),
            lines: Vec::new(),
            stats: BTreeMap::new(),
            topology_dirty: false,
        })
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
            Building::Miner => {
                if !self.resources.contains_key(&pos) {
                    return Err(Reject::NoResource);
                }
                Kind::Miner { progress: 0, output: None }
            }
            Building::Assembler { recipe } => {
                let definition = self
                    .content
                    .recipes
                    .get(usize::from(recipe.0))
                    .ok_or(Reject::UnknownRecipe)?;
                Kind::Assembler {
                    recipe,
                    stock: vec![0; definition.inputs.len()],
                    remaining: 0,
                    pending: VecDeque::new(),
                }
            }
            Building::Depot => Kind::Depot { stock: BTreeMap::new() },
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
        let content = &self.content;
        let stats = &mut self.stats;
        for_each_held_item(&entity.kind, content, |item, count| {
            stat(stats, item).destroyed += count;
        });
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
            if matches!(entity.kind, Kind::Depot { .. }) {
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
            match world.machines.get(&id)?.kind {
                Kind::Assembler { .. } | Kind::Depot { .. } => Some(Target::Entity(id)),
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
                Kind::Miner { output, .. } => *output,
                Kind::Assembler { pending, .. } => pending.front().copied(),
                _ => None,
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
                        self.take(offer.source);
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
            self.take(offer.source);
            let line = &mut self.lines[index as usize];
            line.push_tail(offer.item);
            line.last_feeder = Some(offer.key);
        }
    }

    fn accepts(&self, id: EntityId, item: ItemId) -> bool {
        match &self.machines[&id].kind {
            Kind::Depot { .. } => true,
            Kind::Assembler { recipe, stock, .. } => {
                let inputs = &self.content.recipes[usize::from(recipe.0)].inputs;
                inputs
                    .iter()
                    .zip(stock)
                    .any(|(&(input, amount), &held)| input == item && held < amount * 2)
            }
            _ => false,
        }
    }

    fn store(&mut self, id: EntityId, item: ItemId) {
        let content = &self.content;
        match &mut self.machines.get_mut(&id).expect("target is live").kind {
            Kind::Depot { stock } => *stock.entry(item).or_default() += 1,
            Kind::Assembler { recipe, stock, .. } => {
                let inputs = &content.recipes[usize::from(recipe.0)].inputs;
                let slot = inputs.iter().position(|&(input, _)| input == item);
                stock[slot.expect("store follows accepts")] += 1;
            }
            _ => unreachable!("only depots and assemblers accept items"),
        }
    }

    fn take(&mut self, source: Source) {
        match source {
            Source::Line(index) => self.lines[index as usize].pop_head(),
            Source::Machine(id) => {
                match &mut self.machines.get_mut(&id).expect("source is live").kind {
                    Kind::Miner { output, .. } => *output = None,
                    Kind::Assembler { pending, .. } => {
                        pending.pop_front();
                    }
                    _ => unreachable!("only miners and assemblers output items"),
                }
            }
        }
    }

    fn run_machines(&mut self) {
        let content = &self.content;
        let resources = &mut self.resources;
        let stats = &mut self.stats;
        for entity in self.machines.values_mut() {
            match &mut entity.kind {
                Kind::Miner { progress, output } if output.is_none() => {
                    let resource = match resources.get_mut(&entity.pos) {
                        Some(resource) => resource,
                        None => continue,
                    };
                    *progress += 1;
                    if *progress >= content.miner_ticks {
                        *progress = 0;
                        *output = Some(resource.item);
                        stat(stats, resource.item).produced += 1;
                        resource.amount -= 1;
                        if resource.amount == 0 {
                            resources.remove(&entity.pos);
                        }
                    }
                }
                Kind::Assembler { recipe, stock, remaining, pending } => {
                    let definition = &content.recipes[usize::from(recipe.0)];
                    if *remaining > 0 {
                        *remaining -= 1;
                        if *remaining == 0 {
                            for &(item, count) in &definition.outputs {
                                pending.extend((0..count).map(|_| item));
                                stat(stats, item).produced += u64::from(count);
                            }
                        }
                    } else if pending.is_empty()
                        && definition.inputs.iter().zip(stock.iter()).all(|(i, s)| *s >= i.1)
                    {
                        for (&(item, count), held) in definition.inputs.iter().zip(stock.iter_mut())
                        {
                            *held -= count;
                            stat(stats, item).consumed += u64::from(count);
                        }
                        *remaining = definition.ticks;
                    }
                }
                _ => {}
            }
        }
    }

    /// Every building, for rendering. Machines come first, then belts.
    pub fn buildings(&self) -> impl Iterator<Item = (Pos, Dir, Building)> + '_ {
        self.machines.values().chain(self.belts.values()).map(|entity| {
            let building = match entity.kind {
                Kind::Belt => Building::Belt,
                Kind::Miner { .. } => Building::Miner,
                Kind::Assembler { recipe, .. } => Building::Assembler { recipe },
                Kind::Depot { .. } => Building::Depot,
            };
            (entity.pos, entity.dir, building)
        })
    }

    /// Remaining minable tiles as (position, item, amount left).
    pub fn resources(&self) -> impl Iterator<Item = (Pos, ItemId, u32)> + '_ {
        self.resources.iter().map(|(pos, resource)| (*pos, resource.item, resource.amount))
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

    pub fn depot_stock(&self, pos: Pos, item: ItemId) -> u64 {
        let entity = self.tiles.get(&pos).and_then(|id| self.machines.get(id));
        match entity.map(|entity| &entity.kind) {
            Some(Kind::Depot { stock }) => stock.get(&item).copied().unwrap_or(0),
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
            for_each_held_item(&entity.kind, &self.content, |item, count| {
                *totals.entry(item).or_default() += count;
            });
        }
        totals
    }

    /// Hash of the full simulation state, for comparing peers and replays.
    pub fn state_hash(&self) -> u64 {
        let mut h = Hasher::new();
        h.write(self.tick);
        h.write(u64::from(self.next_id));
        self.hash_content(&mut h);
        h.write(self.resources.len() as u64);
        for (pos, resource) in &self.resources {
            hash_pos(&mut h, *pos);
            h.write(u64::from(resource.item.0));
            h.write(u64::from(resource.amount));
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

    fn hash_content(&self, h: &mut Hasher) {
        h.write(u64::from(self.content.miner_ticks));
        h.write(u64::from(self.content.belt_speed));
        h.write(self.content.items.len() as u64);
        for name in &self.content.items {
            h.write_str(name);
        }
        h.write(self.content.recipes.len() as u64);
        for recipe in &self.content.recipes {
            h.write(u64::from(recipe.ticks));
            for list in [&recipe.inputs, &recipe.outputs] {
                h.write(list.len() as u64);
                for &(item, count) in list {
                    h.write(u64::from(item.0));
                    h.write(u64::from(count));
                }
            }
        }
    }
}

fn for_each_held_item(kind: &Kind, content: &Content, mut visit: impl FnMut(ItemId, u64)) {
    match kind {
        Kind::Belt => {}
        Kind::Miner { output, .. } => {
            if let Some(item) = output {
                visit(*item, 1);
            }
        }
        Kind::Assembler { recipe, stock, pending, .. } => {
            let inputs = &content.recipes[usize::from(recipe.0)].inputs;
            for (&(item, _), &held) in inputs.iter().zip(stock) {
                visit(item, u64::from(held));
            }
            for &item in pending {
                visit(item, 1);
            }
        }
        Kind::Depot { stock } => {
            for (&item, &count) in stock {
                visit(item, count);
            }
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
        Kind::Miner { progress, output } => {
            h.write(1);
            h.write(u64::from(*progress));
            h.write(output.map_or(u64::MAX, |item| u64::from(item.0)));
        }
        Kind::Assembler { recipe, stock, remaining, pending } => {
            h.write(2);
            h.write(u64::from(recipe.0));
            h.write(u64::from(*remaining));
            h.write(stock.len() as u64);
            stock.iter().for_each(|&held| h.write(u64::from(held)));
            h.write(pending.len() as u64);
            pending.iter().for_each(|item| h.write(u64::from(item.0)));
        }
        Kind::Depot { stock } => {
            h.write(3);
            h.write(stock.len() as u64);
            for (item, count) in stock {
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
