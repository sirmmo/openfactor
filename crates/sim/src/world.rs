use crate::belt::{BeltItem, Line, Target, TILE};
use crate::content::{Amount, Content, Cost, Pack, Port, Route, Rule, Scope, UnitDef};
use crate::hash::Hasher;
use crate::{BeltId, Dir, EntityId, ItemId, LayerId, MetricId, Pos, TechId, Tick, UnitId};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

/// How many items a routing unit holds while passing them on.
const ROUTE_BUFFER: u64 = 2;
const PORTS: [Port; 3] = [Port::Front, Port::Left, Port::Right];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Building {
    Belt(BeltId),
    Unit(UnitId),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Command {
    Place { pos: Pos, dir: Dir, building: Building },
    Remove { pos: Pos },
    /// Chooses a unit's rule (by its index in the unit's rules) or a filter's
    /// resource (by its id). `None` goes back to the default.
    Configure { pos: Pos, setting: Option<u16> },
    /// Pays for a research and starts it.
    Research { tech: TechId },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reject {
    Occupied,
    NoResource,
    UnknownUnit,
    UnknownBelt,
    NothingToRemove,
    NothingToConfigure,
    InvalidSetting,
    /// Needs research that hasn't finished.
    Locked,
    /// The shared stock doesn't cover the cost.
    CantAfford,
    UnknownTech,
    AlreadyResearched,
    /// Another research is running.
    ResearchBusy,
    /// A research it depends on hasn't finished.
    MissingRequirement,
    /// Below the ground or above the top level.
    NoSuchLevel,
    /// Nothing on the level below is close enough to hold it up.
    Unsupported,
    /// Something on the level above would lose its only support.
    CarriesLoad,
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
    /// In the shared stock from the start.
    pub granted: u64,
    /// Currently tied up in buildings and finished research.
    pub spent: u64,
}

/// A metric's running total and the tick of its latest change.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MetricValue {
    pub total: u64,
    pub last_tick: Tick,
}

/// What a unit is doing, for display.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    /// Waiting for inputs, or a routing or storage unit.
    Idle,
    Working,
    /// Has its other inputs but its network can't supply this resource.
    NoNetwork(ItemId),
    /// Has its inputs but a map condition doesn't hold.
    Conditions,
    /// Its output has nowhere to go.
    Full,
    /// Its network output hasn't been used yet.
    Standby,
    /// A lift with no other end.
    Unlinked,
}

/// One network a unit belongs to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NetworkView {
    pub resource: ItemId,
    pub members: u32,
    pub producers: u32,
    pub stock: u64,
}

/// Everything the world knows about one building.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Inspection {
    pub building: Building,
    pub dir: Dir,
    pub status: Status,
    pub setting: Option<u16>,
    /// The running rule's index and the ticks it has left.
    pub job: Option<(usize, u32)>,
    pub held: Vec<(ItemId, u64)>,
    pub networks: Vec<NetworkView>,
    /// For a lift: the level it sends to, and the level it receives from.
    pub sends_to: Option<i32>,
    pub receives_from: Option<i32>,
}

#[derive(Clone, Debug)]
struct Entity {
    pos: Pos,
    dir: Dir,
    kind: Kind,
    /// Where each port sends to, in `PORTS` order. Derived from the layout.
    targets: [Option<Target>; 3],
    /// The network index for each resource it links to. Derived from the layout.
    nets: Vec<(ItemId, u32)>,
}

#[derive(Clone, Debug)]
enum Kind {
    Belt(BeltId),
    Machine(Machine),
}

#[derive(Clone, Debug)]
struct Machine {
    unit: UnitId,
    bins: BTreeMap<ItemId, u64>,
    job: Option<Job>,
    /// See [`Command::Configure`].
    setting: Option<u16>,
    /// The port a routing unit tries first.
    next_port: u8,
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
    port: u8,
}

/// Connected members of one resource's network. Each producer holds its own
/// stock in `Shared::charge`, so joining or cutting a network loses nothing.
#[derive(Clone, Debug)]
struct Network {
    resource: ItemId,
    members: u32,
    producers: Vec<EntityId>,
    stock: u64,
}

type Layer = BTreeMap<Pos, u64>;
type Stats = BTreeMap<ItemId, ItemStats>;

/// State that any unit's rule can read or change. Kept apart from the
/// entities so a rule can touch it while they are being iterated.
#[derive(Clone, Debug)]
struct Shared {
    layers: Vec<Layer>,
    globals: BTreeMap<ItemId, u64>,
    metrics: BTreeMap<MetricId, MetricValue>,
    stats: Stats,
    networks: Vec<Network>,
    charge: BTreeMap<(ItemId, EntityId), u64>,
    researched: BTreeSet<TechId>,
    /// The running research and the ticks it has left.
    research: Option<(TechId, u32)>,
}

#[derive(Clone, Debug)]
pub struct World {
    content: Content,
    tick: Tick,
    next_id: u32,
    shared: Shared,
    tiles: BTreeMap<Pos, EntityId>,
    /// Everything that isn't a belt. Kept apart so a tick never visits belts.
    machines: BTreeMap<EntityId, Entity>,
    belts: BTreeMap<EntityId, Entity>,
    lines: Vec<Line>,
    topology_dirty: bool,
}

/// One unit, as a rule sees it.
struct Site<'a> {
    id: EntityId,
    pos: Pos,
    nets: &'a [(ItemId, u32)],
}

impl Site<'_> {
    fn network(&self, resource: ItemId) -> usize {
        let found = self.nets.iter().find(|(linked, _)| *linked == resource);
        found.expect("rules that use a network add a link to it").1 as usize
    }
}

/// Which of a rule's needs are met right now.
struct Readiness {
    conditions: bool,
    inputs: bool,
    network: Option<ItemId>,
    outputs_clear: bool,
    network_clear: bool,
}

impl Readiness {
    fn can_start(&self) -> bool {
        self.conditions && self.inputs && self.network.is_none() && self.outputs_clear && self.network_clear
    }
}

fn stat(stats: &mut Stats, item: ItemId) -> &mut ItemStats {
    stats.entry(item).or_default()
}

/// The map under a position, whatever level it is on.
fn layer_at(layers: &[Layer], layer: LayerId, pos: Pos) -> u64 {
    layers[usize::from(layer.0)].get(&pos.ground()).copied().unwrap_or(0)
}

fn subtract(map: &mut BTreeMap<ItemId, u64>, item: ItemId, amount: u64) {
    let held = map.get_mut(&item).expect("checked before taking");
    *held -= amount;
    if *held == 0 {
        map.remove(&item);
    }
}

fn take_from_tile(layers: &mut [Layer], layer: LayerId, pos: Pos, amount: u64) {
    let pos = pos.ground();
    let tiles = &mut layers[usize::from(layer.0)];
    let value = tiles.get_mut(&pos).expect("checked before taking");
    *value -= amount;
    if *value == 0 {
        tiles.remove(&pos);
    }
}

fn map_rule_applies(rule: &Rule, layers: &[Layer], pos: Pos) -> bool {
    let conditions_hold = rule
        .conditions
        .iter()
        .all(|c| c.test.holds(layer_at(layers, c.layer, pos), c.value));
    conditions_hold
        && rule.takes.iter().all(|take| match take.scope {
            Scope::Map { layer, .. } => layer_at(layers, layer, pos) >= u64::from(take.amount),
            _ => unreachable!("the parser only allows map statements in a map rule"),
        })
}

fn apply_map_rule(rule: &Rule, layers: &mut [Layer], pos: Pos) {
    for take in &rule.takes {
        if let Scope::Map { layer, .. } = take.scope {
            take_from_tile(layers, layer, pos, u64::from(take.amount));
        }
    }
    for give in &rule.gives {
        if let Scope::Map { layer, .. } = give.scope {
            *layers[usize::from(layer.0)].entry(pos).or_default() += u64::from(give.amount);
        }
    }
}

impl Shared {
    fn is_open(&self, locked_by: Option<TechId>) -> bool {
        locked_by.map_or(true, |tech| self.researched.contains(&tech))
    }

    fn can_afford(&self, cost: &Cost) -> bool {
        cost.iter().all(|&(item, amount)| self.globals.get(&item).copied().unwrap_or(0) >= u64::from(amount))
    }

    fn pay(&mut self, cost: &Cost) {
        for &(item, amount) in cost {
            subtract(&mut self.globals, item, u64::from(amount));
            stat(&mut self.stats, item).spent += u64::from(amount);
        }
    }

    fn refund(&mut self, cost: &Cost) {
        for &(item, amount) in cost {
            *self.globals.entry(item).or_default() += u64::from(amount);
            stat(&mut self.stats, item).spent -= u64::from(amount);
        }
    }

    fn charge_of(&self, resource: ItemId, id: EntityId) -> u64 {
        self.charge.get(&(resource, id)).copied().unwrap_or(0)
    }

    fn check(&self, rule: &Rule, machine: &Machine, site: &Site) -> Readiness {
        let conditions = rule
            .conditions
            .iter()
            .all(|c| c.test.holds(layer_at(&self.layers, c.layer, site.pos), c.value));
        let mut inputs = true;
        let mut network = None;
        for take in &rule.takes {
            let needed = u64::from(take.amount);
            match take.scope {
                Scope::Local(item) => inputs &= machine.bins.get(&item).copied().unwrap_or(0) >= needed,
                Scope::Global(item) => inputs &= self.globals.get(&item).copied().unwrap_or(0) >= needed,
                Scope::Map { layer, .. } => inputs &= layer_at(&self.layers, layer, site.pos) >= needed,
                Scope::Network(resource) => {
                    if self.networks[site.network(resource)].stock < needed {
                        network = Some(resource);
                    }
                }
            }
        }
        // A unit doesn't start again until what it last made has left. A
        // network producer keeps one batch in hand so supply doesn't stutter.
        let mut outputs_clear = true;
        let mut network_clear = true;
        for give in &rule.gives {
            match give.scope {
                Scope::Local(item) => outputs_clear &= !machine.bins.contains_key(&item),
                Scope::Network(resource) => network_clear &= self.charge_of(resource, site.id) <= u64::from(give.amount),
                _ => {}
            }
        }
        Readiness { conditions, inputs, network, outputs_clear, network_clear }
    }

    /// Takes from the network's producers, oldest first.
    fn draw(&mut self, resource: ItemId, network: usize, amount: u64) {
        let network = &mut self.networks[network];
        network.stock -= amount;
        let mut remaining = amount;
        for &producer in &network.producers {
            if remaining == 0 {
                break;
            }
            if let Some(held) = self.charge.get_mut(&(resource, producer)) {
                let taken = remaining.min(*held);
                *held -= taken;
                remaining -= taken;
                if *held == 0 {
                    self.charge.remove(&(resource, producer));
                }
            }
        }
        debug_assert_eq!(remaining, 0, "a network's stock is the sum of its producers' charge");
    }

    fn apply_takes(&mut self, rule: &Rule, machine: &mut Machine, site: &Site) {
        for &Amount { scope, amount } in &rule.takes {
            let amount_wide = u64::from(amount);
            match scope {
                Scope::Local(item) => {
                    subtract(&mut machine.bins, item, amount_wide);
                    stat(&mut self.stats, item).consumed += amount_wide;
                }
                Scope::Global(item) => {
                    subtract(&mut self.globals, item, amount_wide);
                    stat(&mut self.stats, item).consumed += amount_wide;
                }
                Scope::Network(resource) => {
                    self.draw(resource, site.network(resource), amount_wide);
                    stat(&mut self.stats, resource).consumed += amount_wide;
                }
                Scope::Map { layer, .. } => take_from_tile(&mut self.layers, layer, site.pos, amount_wide),
            }
        }
    }

    fn apply_gives(&mut self, rule: &Rule, machine: &mut Machine, site: &Site) {
        for &Amount { scope, amount } in &rule.gives {
            let amount_wide = u64::from(amount);
            match scope {
                Scope::Local(item) => {
                    *machine.bins.entry(item).or_default() += amount_wide;
                    stat(&mut self.stats, item).produced += amount_wide;
                }
                Scope::Global(item) => {
                    *self.globals.entry(item).or_default() += amount_wide;
                    stat(&mut self.stats, item).produced += amount_wide;
                }
                Scope::Network(resource) => {
                    *self.charge.entry((resource, site.id)).or_default() += amount_wide;
                    let network = site.network(resource);
                    self.networks[network].stock += amount_wide;
                    stat(&mut self.stats, resource).produced += amount_wide;
                }
                Scope::Map { layer, radius } => {
                    let tiles = &mut self.layers[usize::from(layer.0)];
                    let reach = radius as i32;
                    for dx in -reach..=reach {
                        let spread = reach - dx.abs();
                        for dy in -spread..=spread {
                            *tiles.entry(Pos::new(site.pos.x + dx, site.pos.y + dy)).or_default() += amount_wide;
                        }
                    }
                }
            }
        }
    }
}

fn port_dir(dir: Dir, port: Port) -> Dir {
    match port {
        Port::Front => dir,
        Port::Left => dir.left(),
        Port::Right => dir.right(),
    }
}

fn port_index(port: Port) -> usize {
    PORTS.iter().position(|&p| p == port).expect("all ports are listed")
}

/// Index of the disjoint-set root of `index`.
fn find(parents: &mut [usize], mut index: usize) -> usize {
    while parents[index] != index {
        parents[index] = parents[parents[index]];
        index = parents[index];
    }
    index
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
        let mut globals = BTreeMap::new();
        let mut stats: Stats = BTreeMap::new();
        for &(item, amount) in &pack.worldgen.stock {
            *globals.entry(item).or_default() += u64::from(amount);
            stat(&mut stats, item).granted += u64::from(amount);
        }
        World {
            content,
            tick: 0,
            next_id: 0,
            shared: Shared {
                layers,
                globals,
                metrics: BTreeMap::new(),
                stats,
                networks: Vec::new(),
                charge: BTreeMap::new(),
                researched: BTreeSet::new(),
                research: None,
            },
            tiles: BTreeMap::new(),
            machines: BTreeMap::new(),
            belts: BTreeMap::new(),
            lines: Vec::new(),
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
            self.rebuild_networks();
            self.topology_dirty = false;
        }
        for line in &mut self.lines {
            line.advance();
        }
        self.transfer_items();
        self.run_machines();
        self.run_map_rules();
        self.advance_research();
        self.tick += 1;
        rejected
    }

    fn apply(&mut self, command: &Command) -> Result<(), Reject> {
        match *command {
            Command::Place { pos, dir, building } => self.place(pos, dir, building),
            Command::Remove { pos } => self.remove(pos),
            Command::Configure { pos, setting } => self.configure(pos, setting),
            Command::Research { tech } => self.start_research(tech),
        }
    }

    fn place(&mut self, pos: Pos, dir: Dir, building: Building) -> Result<(), Reject> {
        if pos.z < 0 || pos.z >= self.content.levels as i32 {
            return Err(Reject::NoSuchLevel);
        }
        if self.tiles.contains_key(&pos) {
            return Err(Reject::Occupied);
        }
        let kind = match building {
            Building::Belt(belt) => {
                let definition =
                    self.content.belts.get(usize::from(belt.0)).ok_or(Reject::UnknownBelt)?;
                if !self.shared.is_open(definition.locked_by) {
                    return Err(Reject::Locked);
                }
                Kind::Belt(belt)
            }
            Building::Unit(unit) => {
                let definition =
                    self.content.units.get(usize::from(unit.0)).ok_or(Reject::UnknownUnit)?;
                if !self.shared.is_open(definition.locked_by) {
                    return Err(Reject::Locked);
                }
                let layers = &self.shared.layers;
                let allowed = definition.place_on.is_empty()
                    || (pos.z == 0 && definition.place_on.iter().any(|&layer| layer_at(layers, layer, pos) > 0));
                if !allowed {
                    return Err(Reject::NoResource);
                }
                Kind::Machine(Machine {
                    unit,
                    bins: BTreeMap::new(),
                    job: None,
                    setting: None,
                    next_port: 0,
                })
            }
        };
        if !self.is_supported(pos, None) {
            return Err(Reject::Unsupported);
        }
        let cost = self.cost_of(building);
        if !self.shared.can_afford(cost) {
            return Err(Reject::CantAfford);
        }
        let cost = cost.clone();
        self.shared.pay(&cost);
        let id = EntityId(self.next_id);
        self.next_id += 1;
        self.tiles.insert(pos, id);
        let map = if matches!(kind, Kind::Belt(_)) { &mut self.belts } else { &mut self.machines };
        map.insert(id, Entity { pos, dir, kind, targets: [None; 3], nets: Vec::new() });
        self.topology_dirty = true;
        Ok(())
    }

    /// Whether something at `pos` would be held up, not counting a building
    /// at `ignoring`. Anything on the level below within the span will do.
    fn is_supported(&self, pos: Pos, ignoring: Option<Pos>) -> bool {
        pos.z == 0
            || self.within_span(pos.shifted(-1)).any(|below| Some(below) != ignoring && self.tiles.contains_key(&below))
    }

    /// Every position on `center`'s level within the span of it.
    fn within_span(&self, center: Pos) -> impl Iterator<Item = Pos> {
        let span = self.content.span as i32;
        (-span..=span).flat_map(move |dx| {
            let spread = span - dx.abs();
            (-spread..=spread).map(move |dy| Pos::at(center.x + dx, center.y + dy, center.z))
        })
    }

    /// Whether removing the building at `pos` would leave something above it unsupported.
    fn carries_load(&self, pos: Pos) -> bool {
        self.within_span(pos.shifted(1))
            .any(|above| self.tiles.contains_key(&above) && !self.is_supported(above, Some(pos)))
    }

    fn cost_of(&self, building: Building) -> &Cost {
        match building {
            Building::Belt(belt) => &self.content.belts[usize::from(belt.0)].cost,
            Building::Unit(unit) => &self.content.unit(unit).cost,
        }
    }

    fn start_research(&mut self, tech: TechId) -> Result<(), Reject> {
        let definition = self.content.techs.get(usize::from(tech.0)).ok_or(Reject::UnknownTech)?;
        if self.shared.researched.contains(&tech) {
            return Err(Reject::AlreadyResearched);
        }
        if self.shared.research.is_some() {
            return Err(Reject::ResearchBusy);
        }
        if !definition.requires.iter().all(|t| self.shared.researched.contains(t)) {
            return Err(Reject::MissingRequirement);
        }
        if !self.shared.can_afford(&definition.cost) {
            return Err(Reject::CantAfford);
        }
        self.shared.pay(&definition.cost);
        self.shared.research = Some((tech, definition.time));
        Ok(())
    }

    fn advance_research(&mut self) {
        if let Some((tech, remaining)) = &mut self.shared.research {
            *remaining -= 1;
            if *remaining == 0 {
                let finished = *tech;
                self.shared.researched.insert(finished);
                self.shared.research = None;
            }
        }
    }

    fn remove(&mut self, pos: Pos) -> Result<(), Reject> {
        if !self.tiles.contains_key(&pos) {
            return Err(Reject::NothingToRemove);
        }
        if self.carries_load(pos) {
            return Err(Reject::CarriesLoad);
        }
        let id = self.tiles.remove(&pos).expect("checked above");
        let entity = self
            .machines
            .remove(&id)
            .or_else(|| self.belts.remove(&id))
            .expect("tile points at a live entity");
        let building = match &entity.kind {
            Kind::Belt(belt) => Building::Belt(*belt),
            Kind::Machine(machine) => Building::Unit(machine.unit),
        };
        // Buildings give back everything they cost.
        let cost = self.cost_of(building).clone();
        self.shared.refund(&cost);
        // Items on a removed belt are accounted for in `rebuild_lines`.
        if let Kind::Machine(machine) = &entity.kind {
            for (&item, &count) in &machine.bins {
                stat(&mut self.shared.stats, item).destroyed += count;
            }
            for link in &self.content.unit(machine.unit).links {
                if let Some(held) = self.shared.charge.remove(&(link.resource, id)) {
                    stat(&mut self.shared.stats, link.resource).destroyed += held;
                }
            }
        }
        self.topology_dirty = true;
        Ok(())
    }

    fn configure(&mut self, pos: Pos, setting: Option<u16>) -> Result<(), Reject> {
        let id = self.tiles.get(&pos).ok_or(Reject::NothingToConfigure)?;
        let entity = self.machines.get_mut(id).ok_or(Reject::NothingToConfigure)?;
        let machine = match &mut entity.kind {
            Kind::Machine(machine) => machine,
            Kind::Belt(_) => return Err(Reject::NothingToConfigure),
        };
        let unit = self.content.unit(machine.unit);
        let choices = match unit.route {
            Some(Route::Filter) => self.content.resources.len(),
            Some(_) => return Err(Reject::NothingToConfigure),
            None if unit.rules.len() > 1 => unit.rules.len(),
            None => return Err(Reject::NothingToConfigure),
        };
        if setting.map_or(false, |value| usize::from(value) >= choices) {
            return Err(Reject::InvalidSetting);
        }
        if unit.route.is_none() {
            let locked_by = setting.and_then(|value| unit.rules[usize::from(value)].locked_by);
            if !self.shared.is_open(locked_by) {
                return Err(Reject::Locked);
            }
        }
        machine.setting = setting;
        Ok(())
    }

    fn belt_at(&self, pos: Pos) -> Option<(Dir, BeltId)> {
        let entity = self.belts.get(self.tiles.get(&pos)?)?;
        match entity.kind {
            Kind::Belt(belt) => Some((entity.dir, belt)),
            Kind::Machine(_) => None,
        }
    }

    /// The belt tile an entity at `pos` facing `dir` would feed, if any.
    fn fed_belt(&self, pos: Pos, dir: Dir) -> Option<Pos> {
        let front = pos.step(dir);
        let (front_dir, _) = self.belt_at(front)?;
        (front_dir != dir.opposite()).then_some(front)
    }

    /// The next tunnel of the same kind ahead, facing the same way.
    fn tunnel_partner(&self, entity: &Entity, unit: UnitId, range: u32) -> Option<EntityId> {
        let mut pos = entity.pos;
        for _ in 0..range {
            pos = pos.step(entity.dir);
            let found = self.tiles.get(&pos).and_then(|id| Some((*id, self.machines.get(id)?)));
            if let Some((id, other)) = found {
                let same_unit = matches!(&other.kind, Kind::Machine(m) if m.unit == unit);
                if same_unit && other.dir == entity.dir {
                    return Some(id);
                }
            }
        }
        None
    }

    /// The next lift of the same kind up (or down) the shaft, if the way is clear.
    fn lift_partner(&self, start: Pos, unit: UnitId, range: u32, down: bool) -> Option<EntityId> {
        let step = if down { -1 } else { 1 };
        for height in 1..=range.min(self.content.levels) as i32 {
            if let Some(&id) = self.tiles.get(&start.shifted(step * height)) {
                let same = matches!(self.machines.get(&id), Some(Entity { kind: Kind::Machine(m), .. }) if m.unit == unit);
                return same.then_some(id);
            }
        }
        None
    }

    /// Where a tunnel or lift hands its items on to, if there's a next piece.
    fn passage_partner(&self, entity: &Entity, unit: UnitId, route: Route) -> Option<EntityId> {
        match route {
            Route::Tunnel { range } => self.tunnel_partner(entity, unit, range),
            Route::Lift { range, down } => self.lift_partner(entity.pos, unit, range, down),
            Route::Split | Route::Filter => None,
        }
    }

    /// Which ports can send to the tile next to them, in `PORTS` order.
    fn adjacent_ports(&self, entity: &Entity, machine: &Machine) -> [bool; 3] {
        let unit = self.content.unit(machine.unit);
        match unit.route {
            Some(route @ (Route::Tunnel { .. } | Route::Lift { .. })) => {
                [self.passage_partner(entity, machine.unit, route).is_none(), false, false]
            }
            Some(_) => [true; 3],
            None => {
                let mut used = [false; 3];
                for &(_, port) in &unit.outputs {
                    used[port_index(port)] = true;
                }
                used
            }
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

        #[derive(Clone, Copy, Default)]
        struct Feeds {
            belts: u32,
            machines: u32,
            other_tier: bool,
        }
        let mut feeders: BTreeMap<Pos, Feeds> = BTreeMap::new();
        for entity in self.machines.values() {
            let machine = match &entity.kind {
                Kind::Machine(machine) => machine,
                Kind::Belt(_) => continue,
            };
            let used = self.adjacent_ports(entity, machine);
            for (port, _) in PORTS.iter().zip(used).filter(|(_, used)| *used) {
                if let Some(front) = self.fed_belt(entity.pos, port_dir(entity.dir, *port)) {
                    feeders.entry(front).or_default().machines += 1;
                }
            }
        }
        for entity in self.belts.values() {
            if let (Some(front), Kind::Belt(tier)) = (self.fed_belt(entity.pos, entity.dir), &entity.kind) {
                let feeds = feeders.entry(front).or_default();
                feeds.belts += 1;
                feeds.other_tier |= self.belt_at(front).map(|(_, belt)| belt) != Some(*tier);
            }
        }

        let belts: Vec<Pos> = self
            .tiles
            .keys()
            .copied()
            .filter(|&pos| self.belt_at(pos).is_some())
            .collect();
        // A tile continues its feeder's line only when that belt is its sole
        // input and they run at the same speed.
        let continues = |pos: Pos| {
            feeders.get(&pos).map_or(false, |f| f.belts == 1 && f.machines == 0 && !f.other_tier)
        };

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
            while let Some(next) = self.fed_belt(current, self.belt_at(current).unwrap().0) {
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
            let (head_dir, tier) = self.belt_at(head).expect("chains hold belts");
            lines.push(Line {
                head_entity: self.tiles[&head],
                target: target_of(self, head, head_dir),
                tiles,
                items,
                tail_free: previous,
                speed: self.content.belts[usize::from(tier.0)].speed,
                compressed: 0,
                last_feeder: None,
            });
        }

        for items in carried.into_values() {
            for (_, item) in items {
                stat(&mut self.shared.stats, item).destroyed += 1;
            }
        }

        let targets: Vec<[Option<Target>; 3]> = self
            .machines
            .values()
            .map(|entity| {
                let mut found = [None; 3];
                let machine = match &entity.kind {
                    Kind::Machine(machine) => machine,
                    Kind::Belt(_) => return found,
                };
                let unit = self.content.unit(machine.unit);
                if let Some(partner) = unit.route.and_then(|route| self.passage_partner(entity, machine.unit, route)) {
                    found[0] = Some(Target::Entity(partner));
                    return found;
                }
                let used = self.adjacent_ports(entity, machine);
                for (index, port) in PORTS.iter().enumerate() {
                    if used[index] {
                        found[index] = target_of(self, entity.pos, port_dir(entity.dir, *port));
                    }
                }
                found
            })
            .collect();
        for (entity, found) in self.machines.values_mut().zip(targets) {
            entity.targets = found;
        }

        self.lines = lines;
    }

    /// Groups linked units into connected networks, one resource at a time.
    fn rebuild_networks(&mut self) {
        let mut by_resource: BTreeMap<ItemId, Vec<(EntityId, Pos, u32)>> = BTreeMap::new();
        for (&id, entity) in &self.machines {
            if let Kind::Machine(machine) = &entity.kind {
                for link in &self.content.unit(machine.unit).links {
                    by_resource.entry(link.resource).or_default().push((id, entity.pos, link.reach));
                }
            }
        }

        let mut networks: Vec<Network> = Vec::new();
        let mut memberships: BTreeMap<EntityId, Vec<(ItemId, u32)>> = BTreeMap::new();
        for (resource, members) in by_resource {
            let index_at: BTreeMap<Pos, usize> =
                members.iter().enumerate().map(|(index, member)| (member.1, index)).collect();
            let mut parents: Vec<usize> = (0..members.len()).collect();
            let top = self.content.levels as i32 - 1;
            for (index, &(_, pos, reach)) in members.iter().enumerate() {
                let reach = reach as i32;
                // A step up or down counts the same as a step along the ground.
                for dz in -reach.min(top)..=reach.min(top) {
                    let flat = reach - dz.abs();
                    for dx in -flat..=flat {
                        let spread = flat - dx.abs();
                        for dy in -spread..=spread {
                            if let Some(&other) = index_at.get(&Pos::at(pos.x + dx, pos.y + dy, pos.z + dz)) {
                                let (a, b) = (find(&mut parents, index), find(&mut parents, other));
                                // The older member stays the root, so numbering is stable.
                                parents[a.max(b)] = a.min(b);
                            }
                        }
                    }
                }
            }

            let mut network_of_root: BTreeMap<usize, u32> = BTreeMap::new();
            for (index, &(id, _, _)) in members.iter().enumerate() {
                let root = find(&mut parents, index);
                let network = *network_of_root.entry(root).or_insert_with(|| {
                    networks.push(Network { resource, members: 0, producers: Vec::new(), stock: 0 });
                    networks.len() as u32 - 1
                });
                memberships.entry(id).or_default().push((resource, network));

                let entry = &mut networks[network as usize];
                entry.members += 1;
                let unit = match &self.machines[&id].kind {
                    Kind::Machine(machine) => self.content.unit(machine.unit),
                    Kind::Belt(_) => unreachable!("only machines link to networks"),
                };
                if unit.produces_on(resource) {
                    entry.producers.push(id);
                    entry.stock += self.shared.charge_of(resource, id);
                }
            }
        }

        for (id, entity) in &mut self.machines {
            entity.nets = memberships.remove(id).unwrap_or_default();
        }
        self.shared.networks = networks;
    }

    fn can_receive(&self, target: Target, item: ItemId) -> bool {
        match target {
            Target::Line(index) => self.lines[index as usize].can_accept(),
            Target::Entity(id) => self.accepts(id, item),
        }
    }

    /// Where a routing unit should send `item` next, if anywhere can take it.
    fn route_port(&self, entity: &Entity, machine: &Machine, route: Route, item: ItemId) -> Option<(usize, Target)> {
        let allowed = |port: usize| match route {
            Route::Split | Route::Tunnel { .. } | Route::Lift { .. } => true,
            Route::Filter => match machine.setting {
                Some(chosen) if chosen == item.0 => port == 0,
                Some(_) => port != 0,
                None => port == 0,
            },
        };
        (0..PORTS.len()).map(|step| (usize::from(machine.next_port) + step) % PORTS.len()).find_map(|port| {
            let target = entity.targets[port]?;
            (allowed(port) && self.can_receive(target, item)).then_some((port, target))
        })
    }

    fn collect_offers(&self) -> Vec<Offer> {
        let mut offers = Vec::new();
        for (index, line) in self.lines.iter().enumerate() {
            if let (Some(item), Some(target)) = (line.head_item(), line.target) {
                let source = Source::Line(index as u32);
                offers.push(Offer { key: line.head_entity, item, source, target, port: 0 });
            }
        }
        for (&id, entity) in &self.machines {
            let machine = match &entity.kind {
                Kind::Machine(machine) => machine,
                Kind::Belt(_) => continue,
            };
            let unit = self.content.unit(machine.unit);
            let source = Source::Machine(id);
            if let Some(route) = unit.route {
                let found = machine.bins.keys().find_map(|&item| {
                    self.route_port(entity, machine, route, item).map(|choice| (item, choice))
                });
                if let Some((item, (port, target))) = found {
                    offers.push(Offer { key: id, item, source, target, port: port as u8 });
                }
                continue;
            }
            // One item per side per tick.
            let mut sent = [false; 3];
            for &(item, port) in &unit.outputs {
                let port = port_index(port);
                if sent[port] || !machine.bins.contains_key(&item) {
                    continue;
                }
                if let Some(target) = entity.targets[port].filter(|&t| self.can_receive(t, item)) {
                    sent[port] = true;
                    offers.push(Offer { key: id, item, source, target, port: port as u8 });
                }
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
                        self.take(offer);
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
            self.take(offer);
            let line = &mut self.lines[index as usize];
            line.push_tail(offer.item);
            line.last_feeder = Some(offer.key);
        }
    }

    fn accepts(&self, id: EntityId, item: ItemId) -> bool {
        let machine = match &self.machines[&id].kind {
            Kind::Machine(machine) => machine,
            Kind::Belt(_) => return false,
        };
        let unit = self.content.unit(machine.unit);
        if unit.stores_any || unit.delivers {
            return true;
        }
        if unit.route.is_some() {
            return machine.bins.values().sum::<u64>() < ROUTE_BUFFER;
        }
        let held = machine.bins.get(&item).copied().unwrap_or(0);
        let is_open = |rule: &Rule| self.shared.is_open(rule.locked_by);
        unit.input_limit(item, machine.setting.map(usize::from), is_open).map_or(false, |limit| held < limit)
    }

    fn machine_mut(&mut self, id: EntityId) -> &mut Machine {
        match &mut self.machines.get_mut(&id).expect("id is live").kind {
            Kind::Machine(machine) => machine,
            Kind::Belt(_) => unreachable!("belts are stored separately"),
        }
    }

    fn store(&mut self, id: EntityId, item: ItemId) {
        let machine = self.machine_mut(id);
        let unit = machine.unit;
        if self.content.unit(unit).delivers {
            *self.shared.globals.entry(item).or_default() += 1;
        } else {
            *self.machine_mut(id).bins.entry(item).or_default() += 1;
        }
    }

    fn take(&mut self, offer: Offer) {
        match offer.source {
            Source::Line(index) => self.lines[index as usize].pop_head(),
            Source::Machine(id) => {
                let machine = self.machine_mut(id);
                subtract(&mut machine.bins, offer.item, 1);
                machine.next_port = (offer.port + 1) % PORTS.len() as u8;
            }
        }
    }

    fn run_machines(&mut self) {
        let content = &self.content;
        let shared = &mut self.shared;
        let tick = self.tick;
        for (&id, entity) in &mut self.machines {
            let site = Site { id, pos: entity.pos, nets: &entity.nets };
            let machine = match &mut entity.kind {
                Kind::Machine(machine) => machine,
                Kind::Belt(_) => continue,
            };
            let unit = content.unit(machine.unit);
            if let Some(job) = &mut machine.job {
                job.remaining -= 1;
                if job.remaining == 0 {
                    let rule = &unit.rules[job.rule];
                    machine.job = None;
                    shared.apply_gives(rule, machine, &site);
                    for &(metric, amount) in &rule.logs {
                        let value = shared.metrics.entry(metric).or_default();
                        value.total += u64::from(amount);
                        value.last_tick = tick;
                    }
                }
                continue;
            }
            // Rules are tried in the order the pack lists them, unless one was chosen.
            let ready = match machine.setting.filter(|_| unit.route.is_none()) {
                Some(index) => Some(usize::from(index))
                    .filter(|&index| shared.check(&unit.rules[index], machine, &site).can_start()),
                None => unit
                    .rules
                    .iter()
                    .position(|r| shared.is_open(r.locked_by) && shared.check(r, machine, &site).can_start()),
            };
            if let Some(index) = ready {
                shared.apply_takes(&unit.rules[index], machine, &site);
                machine.job = Some(Job { rule: index, remaining: unit.rules[index].rate });
            }
        }
    }

    /// Applies each map rule that is due, tile by tile in position order.
    fn run_map_rules(&mut self) {
        let finished_ticks = self.tick + 1;
        let layers = &mut self.shared.layers;
        for map_rule in &self.content.map_rules {
            let rule = &map_rule.rule;
            if finished_ticks % u64::from(rule.rate) != 0 {
                continue;
            }
            let tiles: Vec<Pos> = layers[usize::from(map_rule.anchor.0)].keys().copied().collect();
            for pos in tiles {
                if map_rule_applies(rule, layers, pos) {
                    apply_map_rule(rule, layers, pos);
                }
            }
        }
    }

    pub fn metric(&self, metric: MetricId) -> MetricValue {
        self.shared.metrics.get(&metric).copied().unwrap_or_default()
    }

    /// Every building, for rendering. Machines come first, then belts.
    pub fn buildings(&self) -> impl Iterator<Item = (Pos, Dir, Building)> + '_ {
        self.machines.values().chain(self.belts.values()).map(|entity| {
            let building = match &entity.kind {
                Kind::Belt(belt) => Building::Belt(*belt),
                Kind::Machine(machine) => Building::Unit(machine.unit),
            };
            (entity.pos, entity.dir, building)
        })
    }

    fn output_stuck(&self, entity: &Entity, machine: &Machine, unit: &UnitDef) -> bool {
        unit.outputs.iter().any(|&(item, port)| {
            machine.bins.contains_key(&item)
                && entity.targets[port_index(port)].map_or(true, |target| !self.can_receive(target, item))
        })
    }

    fn status_of(&self, id: EntityId, entity: &Entity) -> Status {
        let machine = match &entity.kind {
            Kind::Machine(machine) => machine,
            Kind::Belt(_) => return Status::Idle,
        };
        let unit = self.content.unit(machine.unit);
        if machine.job.is_some() {
            return Status::Working;
        }
        if let Some(Route::Lift { range, down }) = unit.route {
            let linked = [down, !down].iter().any(|&way| self.lift_partner(entity.pos, machine.unit, range, way).is_some());
            if !linked {
                return Status::Unlinked;
            }
        }
        if unit.route.is_some() {
            return Status::Idle;
        }
        if self.output_stuck(entity, machine, unit) {
            return Status::Full;
        }
        let site = Site { id, pos: entity.pos, nets: &entity.nets };
        let chosen = machine.setting.map(usize::from);
        let candidates = unit
            .rules
            .iter()
            .enumerate()
            .filter(|(index, rule)| chosen.map_or(self.shared.is_open(rule.locked_by), |c| c == *index));
        let mut status = Status::Idle;
        for (_, rule) in candidates {
            let readiness = self.shared.check(rule, machine, &site);
            if !readiness.network_clear {
                return Status::Standby;
            }
            if readiness.inputs {
                match readiness.network {
                    Some(resource) => return Status::NoNetwork(resource),
                    None if !readiness.conditions => status = Status::Conditions,
                    None => {}
                }
            }
        }
        status
    }

    /// What the unit at `pos` is doing, if there is one.
    pub fn status(&self, pos: Pos) -> Option<Status> {
        let id = *self.tiles.get(&pos)?;
        Some(self.status_of(id, self.machines.get(&id)?))
    }

    /// A full description of the building at `pos`, for an inspector.
    pub fn inspect(&self, pos: Pos) -> Option<Inspection> {
        let id = *self.tiles.get(&pos)?;
        let entity = self.machines.get(&id).or_else(|| self.belts.get(&id))?;
        let view = |&(_, index): &(ItemId, u32)| {
            let network = &self.shared.networks[index as usize];
            NetworkView {
                resource: network.resource,
                members: network.members,
                producers: network.producers.len() as u32,
                stock: network.stock,
            }
        };
        let (building, setting, job, held) = match &entity.kind {
            Kind::Belt(belt) => (Building::Belt(*belt), None, None, Vec::new()),
            Kind::Machine(machine) => (
                Building::Unit(machine.unit),
                machine.setting,
                machine.job.map(|job| (job.rule, job.remaining)),
                machine.bins.iter().map(|(&item, &count)| (item, count)).collect(),
            ),
        };
        let (sends_to, receives_from) = self.lift_ends(entity);
        Some(Inspection {
            building,
            dir: entity.dir,
            status: self.status_of(id, entity),
            setting,
            job,
            held,
            networks: entity.nets.iter().map(view).collect(),
            sends_to,
            receives_from,
        })
    }

    /// The levels a lift sends to and receives from.
    fn lift_ends(&self, entity: &Entity) -> (Option<i32>, Option<i32>) {
        let machine = match &entity.kind {
            Kind::Machine(machine) => machine,
            Kind::Belt(_) => return (None, None),
        };
        match self.content.unit(machine.unit).route {
            Some(Route::Lift { range, down }) => {
                let level = |way| {
                    let partner = self.lift_partner(entity.pos, machine.unit, range, way)?;
                    Some(self.machines[&partner].pos.z)
                };
                (level(down), level(!down))
            }
            _ => (None, None),
        }
    }

    /// Every non-zero map tile as (position, layer, value).
    pub fn layer_tiles(&self) -> impl Iterator<Item = (Pos, LayerId, u64)> + '_ {
        self.shared.layers.iter().enumerate().flat_map(|(index, tiles)| {
            tiles.iter().map(move |(pos, value)| (*pos, LayerId(index as u16), *value))
        })
    }

    /// Whether a research has finished.
    pub fn researched(&self, tech: TechId) -> bool {
        self.shared.researched.contains(&tech)
    }

    /// The running research and the ticks it has left.
    pub fn active_research(&self) -> Option<(TechId, u32)> {
        self.shared.research
    }

    pub fn layer_value(&self, layer: LayerId, pos: Pos) -> u64 {
        layer_at(&self.shared.layers, layer, pos)
    }

    /// The shared stock of a resource: what depots have received, less what has
    /// been spent, plus anything moved by `global` rules.
    pub fn global(&self, item: ItemId) -> u64 {
        self.shared.globals.get(&item).copied().unwrap_or(0)
    }

    /// Every belt item as (tile, tile direction, offset along it in 0..=`TILE`, item).
    pub fn belt_items(&self) -> impl Iterator<Item = (Pos, Dir, u32, ItemId)> + '_ {
        self.lines.iter().flat_map(move |line| {
            line.placed_items().map(move |(pos, offset, item)| {
                (pos, self.belt_at(pos).expect("line tiles are belts").0, offset, item)
            })
        })
    }

    pub fn stats(&self, item: ItemId) -> ItemStats {
        self.shared.stats.get(&item).copied().unwrap_or_default()
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

    /// Count of every item that physically exists in the world, network stock included.
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
        for (&(resource, _), &held) in &self.shared.charge {
            *totals.entry(resource).or_default() += held;
        }
        for (&item, &held) in &self.shared.globals {
            *totals.entry(item).or_default() += held;
        }
        totals
    }

    /// Hash of the full simulation state, for comparing peers and replays.
    pub fn state_hash(&self) -> u64 {
        let mut h = Hasher::new();
        h.write(self.tick);
        h.write(u64::from(self.next_id));
        self.hash_content(&mut h);
        for tiles in &self.shared.layers {
            h.write(tiles.len() as u64);
            for (pos, value) in tiles {
                hash_pos(&mut h, *pos);
                h.write(*value);
            }
        }
        h.write(self.shared.globals.len() as u64);
        for (item, value) in &self.shared.globals {
            h.write(u64::from(item.0));
            h.write(*value);
        }
        h.write(self.shared.metrics.len() as u64);
        for (metric, value) in &self.shared.metrics {
            h.write(u64::from(metric.0));
            h.write(value.total);
            h.write(value.last_tick);
        }
        h.write(self.shared.charge.len() as u64);
        for ((resource, id), held) in &self.shared.charge {
            h.write(u64::from(resource.0));
            h.write(u64::from(id.0));
            h.write(*held);
        }
        h.write(self.shared.researched.len() as u64);
        self.shared.researched.iter().for_each(|tech| h.write(u64::from(tech.0)));
        match self.shared.research {
            None => h.write(u64::MAX),
            Some((tech, remaining)) => {
                h.write(u64::from(tech.0));
                h.write(u64::from(remaining));
            }
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
        h.write(self.shared.stats.len() as u64);
        for (item, stats) in &self.shared.stats {
            h.write(u64::from(item.0));
            h.write(stats.produced);
            h.write(stats.consumed);
            h.write(stats.destroyed);
            h.write(stats.granted);
            h.write(stats.spent);
        }
        h.finish()
    }

    /// Covers everything that affects the simulation. Names and sprites are
    /// left out, so a cosmetic difference between packs is not a desync.
    fn hash_content(&self, h: &mut Hasher) {
        h.write(u64::from(self.content.levels));
        h.write(u64::from(self.content.span));
        h.write(self.content.belts.len() as u64);
        for belt in &self.content.belts {
            h.write_str(&belt.id);
            h.write(u64::from(belt.speed));
            hash_cost(h, &belt.cost);
            h.write(hash_lock(belt.locked_by));
        }
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
            h.write(unit.place_on.len() as u64);
            unit.place_on.iter().for_each(|layer| h.write(u64::from(layer.0)));
            h.write(u64::from(unit.stores_any));
            h.write(u64::from(unit.delivers));
            hash_cost(h, &unit.cost);
            h.write(hash_lock(unit.locked_by));
            match unit.route {
                None => h.write(0),
                Some(Route::Split) => h.write(1),
                Some(Route::Filter) => h.write(2),
                Some(Route::Tunnel { range }) => {
                    h.write(3);
                    h.write(u64::from(range));
                }
                Some(Route::Lift { range, down }) => {
                    h.write(4);
                    h.write(u64::from(range));
                    h.write(u64::from(down));
                }
            }
            h.write(unit.links.len() as u64);
            for link in &unit.links {
                h.write(u64::from(link.resource.0));
                h.write(u64::from(link.reach));
            }
            h.write(unit.outputs.len() as u64);
            for (item, port) in &unit.outputs {
                h.write(u64::from(item.0));
                h.write(*port as u64);
            }
            h.write(unit.rules.len() as u64);
            for rule in &unit.rules {
                hash_rule(h, rule);
            }
        }
        h.write(self.content.techs.len() as u64);
        for tech in &self.content.techs {
            h.write_str(&tech.id);
            h.write(u64::from(tech.time));
            hash_cost(h, &tech.cost);
            h.write(tech.requires.len() as u64);
            tech.requires.iter().for_each(|required| h.write(u64::from(required.0)));
        }
        h.write(self.content.metrics.len() as u64);
        for metric in &self.content.metrics {
            h.write_str(&metric.id);
        }
        h.write(self.content.map_rules.len() as u64);
        for map_rule in &self.content.map_rules {
            h.write(u64::from(map_rule.anchor.0));
            hash_rule(h, &map_rule.rule);
        }
    }
}

fn hash_cost(h: &mut Hasher, cost: &Cost) {
    h.write(cost.len() as u64);
    for (item, amount) in cost {
        h.write(u64::from(item.0));
        h.write(u64::from(*amount));
    }
}

fn hash_lock(locked_by: Option<TechId>) -> u64 {
    locked_by.map_or(u64::MAX, |tech| u64::from(tech.0))
}

fn hash_rule(h: &mut Hasher, rule: &Rule) {
    h.write(u64::from(rule.rate));
    h.write(hash_lock(rule.locked_by));
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
                Scope::Network(item) => {
                    h.write(3);
                    h.write(u64::from(item.0));
                }
            }
            h.write(u64::from(entry.amount));
        }
    }
    h.write(rule.logs.len() as u64);
    for (metric, amount) in &rule.logs {
        h.write(u64::from(metric.0));
        h.write(u64::from(*amount));
    }
}

fn hash_pos(h: &mut Hasher, pos: Pos) {
    h.write_i32(pos.x);
    h.write_i32(pos.y);
    h.write_i32(pos.z);
}

fn hash_kind(h: &mut Hasher, kind: &Kind) {
    match kind {
        Kind::Belt(belt) => {
            h.write(0);
            h.write(u64::from(belt.0));
        }
        Kind::Machine(machine) => {
            h.write(1);
            h.write(u64::from(machine.unit.0));
            h.write(machine.setting.map_or(u64::MAX, u64::from));
            h.write(u64::from(machine.next_port));
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
    h.write(u64::from(line.speed));
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
