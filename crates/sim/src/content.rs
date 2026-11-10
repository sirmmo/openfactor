use crate::{BeltId, Command, ItemId, LayerId, MetricId, Pos, TechId, UnitId};

/// What something costs from the shared stock.
pub type Cost = Vec<(ItemId, u32)>;

/// A parsed rule pack: the definitions plus the map they start on.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pack {
    pub content: Content,
    pub worldgen: WorldGen,
}

/// Everything that defines "the game". Only belts are built into the engine.
/// Built by [`Pack::parse`], which guarantees every reference is valid.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Content {
    pub(crate) resources: Vec<ResourceDef>,
    pub(crate) layers: Vec<LayerDef>,
    pub(crate) units: Vec<UnitDef>,
    pub(crate) metrics: Vec<MetricDef>,
    pub(crate) map_rules: Vec<MapRule>,
    /// Belt tiers. The first is always the built-in `belt`.
    pub(crate) belts: Vec<BeltDef>,
    pub(crate) techs: Vec<TechDef>,
    /// How many levels can be built on. 1 is a flat world.
    pub(crate) levels: u32,
    /// How far, counted along the grid, something above the ground can be
    /// from the nearest building on the level below it.
    pub(crate) span: u32,
}

/// One entry in the research tree. Paid for from the shared stock when it starts.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TechDef {
    pub id: String,
    pub name: String,
    /// Player-facing explanation, from `describe`. Cosmetic.
    pub description: String,
    pub requires: Vec<TechId>,
    pub cost: Cost,
    /// Ticks from starting to finishing.
    pub time: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BeltDef {
    pub id: String,
    pub name: String,
    /// Player-facing explanation, from `describe`. Cosmetic.
    pub description: String,
    /// Distance an item moves per tick, in 1/`TILE` of a tile.
    pub speed: u32,
    /// Which set of belt art to draw.
    pub style: u32,
    /// 3D models for a straight piece and a corner. Cosmetic.
    pub model: Option<String>,
    pub corner_model: Option<String>,
    pub cost: Cost,
    /// The research that makes it available, if any.
    pub locked_by: Option<TechId>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResourceDef {
    pub id: String,
    pub name: String,
    /// Player-facing explanation, from `describe`. Cosmetic.
    pub description: String,
    pub sprite: u32,
    /// Colour blended over the sprite, as 0xRRGGBB.
    pub tint: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LayerDef {
    pub id: String,
    pub name: String,
    /// Tiles drawn for this layer; empty means it has no ground art.
    pub sprites: Vec<u32>,
    /// Colour laid over those tiles, as 0xRRGGBB.
    pub tint: Option<u32>,
    /// Draws the layer as a haze that thickens with its value.
    pub overlay: Option<Overlay>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Overlay {
    /// Haze colour, as 0xRRGGBB.
    pub color: u32,
    /// The value at which the haze is at its thickest.
    pub full: u32,
}

/// A running total that rules add to with `log`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MetricDef {
    pub id: String,
    pub name: String,
}

/// One `mapRule`: applied every `rate` ticks to each tile that qualifies.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MapRule {
    pub rule: Rule,
    /// Only tiles with a value in this layer can qualify, which keeps the
    /// rule to a finite set of tiles on an unbounded map.
    pub anchor: LayerId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnitDef {
    pub id: String,
    pub name: String,
    /// Player-facing explanation, from `describe`. Cosmetic.
    pub description: String,
    pub sprite: u32,
    /// Colour blended over the sprite, as 0xRRGGBB.
    pub tint: Option<u32>,
    /// Toolbar section. Cosmetic.
    pub group: String,
    /// 3D model. Cosmetic.
    pub model: Option<String>,
    /// If not empty, the unit can only be placed where one of these layers is above zero.
    pub place_on: Vec<LayerId>,
    /// Accepts any resource without limit.
    pub stores_any: bool,
    /// Adds whatever it receives to the shared stock.
    pub delivers: bool,
    pub cost: Cost,
    /// The research that makes it available, if any.
    pub locked_by: Option<TechId>,
    /// If set, the unit passes items along instead of running rules.
    pub route: Option<Route>,
    /// Networks it belongs to. Rules that use a network add it here automatically.
    pub links: Vec<Link>,
    pub rules: Vec<Rule>,
    /// Resources it accepts and how many it holds, derived from `local .. in`.
    pub inputs: Vec<(ItemId, u64)>,
    /// Resources it sends and from which side, derived from `local .. out`.
    pub outputs: Vec<(ItemId, Port)>,
}

impl UnitDef {
    pub fn accepts_items(&self) -> bool {
        self.stores_any || self.delivers || !self.inputs.is_empty() || self.route.is_some()
    }

    pub fn sends_items(&self) -> bool {
        !self.outputs.is_empty() || self.route.is_some()
    }

    /// Whether its rules can add to `resource`'s network.
    pub(crate) fn produces_on(&self, resource: ItemId) -> bool {
        let gives = |rule: &Rule| rule.gives.iter().any(|g| g.scope == Scope::Network(resource));
        self.rules.iter().any(gives)
    }

    /// How many of `item` it holds when running rule `index`, or any open rule if `None`.
    pub(crate) fn input_limit(&self, item: ItemId, index: Option<usize>, is_open: impl Fn(&Rule) -> bool) -> Option<u64> {
        let limit_of = |rule: &Rule| {
            rule.takes.iter().find(|take| take.scope == Scope::Local(item)).map(|take| u64::from(take.amount) * 2)
        };
        match index {
            Some(index) => limit_of(&self.rules[index]),
            None => self.rules.iter().filter(|rule| is_open(rule)).filter_map(limit_of).max(),
        }
    }
}

/// How a routing unit passes along what it receives.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Route {
    /// Takes turns between its front, left and right.
    Split,
    /// The chosen resource goes straight on; the rest leaves by the sides.
    Filter,
    /// Sends to the next matching tunnel ahead, within `range` tiles, over anything between.
    Tunnel { range: u32 },
    /// Sends to the next lift of the same kind straight up (or down), within
    /// `range` levels. The shaft between them must be empty.
    Lift { range: u32, down: bool },
}

/// Membership of a network. Two members connect when either one's `reach`
/// covers the other, counted in steps along the grid.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Link {
    pub resource: ItemId,
    pub reach: u32,
}

/// The side of a unit something leaves from, relative to the way it faces.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Port {
    Front,
    Left,
    Right,
}

/// One `unitRule`. Inputs are taken when it starts, outputs given when it ends.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rule {
    pub name: String,
    pub rate: u32,
    pub conditions: Vec<Condition>,
    pub takes: Vec<Amount>,
    pub gives: Vec<Amount>,
    /// Added to metrics each time the rule finishes.
    pub logs: Vec<(MetricId, u32)>,
    /// Outputs that leave somewhere other than the front.
    pub sides: Vec<(ItemId, Port)>,
    /// The research that makes it available, if any.
    pub locked_by: Option<TechId>,
}

impl Rule {
    pub(crate) fn port_of(&self, item: ItemId) -> Port {
        self.sides.iter().find(|(side, _)| *side == item).map_or(Port::Front, |&(_, port)| port)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Amount {
    pub scope: Scope,
    pub amount: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scope {
    Local(ItemId),
    Global(ItemId),
    /// Shared between the buildings connected in that resource's network.
    Network(ItemId),
    Map { layer: LayerId, radius: u32 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Condition {
    pub layer: LayerId,
    pub test: Test,
    pub value: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Test {
    AtLeast,
    AtMost,
    Greater,
    Less,
    Is,
}

impl Test {
    pub(crate) fn holds(self, actual: u64, value: u32) -> bool {
        let value = u64::from(value);
        match self {
            Test::AtLeast => actual >= value,
            Test::AtMost => actual <= value,
            Test::Greater => actual > value,
            Test::Less => actual < value,
            Test::Is => actual == value,
        }
    }
}

/// A rectangle of a map layer set to a starting value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Patch {
    pub origin: Pos,
    pub width: u32,
    pub height: u32,
    pub layer: LayerId,
    pub amount_per_tile: u32,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct WorldGen {
    pub(crate) patches: Vec<Patch>,
    /// Buildings the pack places at the start, to apply on the first tick.
    pub starter: Vec<Command>,
    /// What the shared stock starts with.
    pub(crate) stock: Vec<(ItemId, u32)>,
}

impl Content {
    pub fn resources(&self) -> &[ResourceDef] {
        &self.resources
    }

    pub fn layers(&self) -> &[LayerDef] {
        &self.layers
    }

    pub fn units(&self) -> &[UnitDef] {
        &self.units
    }

    pub fn metrics(&self) -> &[MetricDef] {
        &self.metrics
    }

    pub fn techs(&self) -> &[TechDef] {
        &self.techs
    }

    pub fn tech_id(&self, id: &str) -> Option<TechId> {
        self.techs.iter().position(|t| t.id == id).map(|i| TechId(i as u16))
    }

    pub fn levels(&self) -> u32 {
        self.levels
    }

    pub fn span(&self) -> u32 {
        self.span
    }

    pub fn belts(&self) -> &[BeltDef] {
        &self.belts
    }

    pub fn belt_id(&self, id: &str) -> Option<BeltId> {
        self.belts.iter().position(|b| b.id == id).map(|i| BeltId(i as u16))
    }

    pub fn metric_id(&self, id: &str) -> Option<MetricId> {
        self.metrics.iter().position(|m| m.id == id).map(|i| MetricId(i as u16))
    }

    pub fn resource_id(&self, id: &str) -> Option<ItemId> {
        self.resources.iter().position(|r| r.id == id).map(|i| ItemId(i as u16))
    }

    pub fn layer_id(&self, id: &str) -> Option<LayerId> {
        self.layers.iter().position(|l| l.id == id).map(|i| LayerId(i as u16))
    }

    pub fn unit_id(&self, id: &str) -> Option<UnitId> {
        self.units.iter().position(|u| u.id == id).map(|i| UnitId(i as u16))
    }

    pub(crate) fn unit(&self, unit: UnitId) -> &UnitDef {
        &self.units[usize::from(unit.0)]
    }
}
