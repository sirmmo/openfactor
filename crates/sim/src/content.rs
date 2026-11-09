use crate::{Command, ItemId, LayerId, MetricId, Pos, UnitId};

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
    /// Distance a belt item moves per tick, in 1/`TILE` of a tile.
    pub(crate) belt_speed: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResourceDef {
    pub id: String,
    pub name: String,
    pub sprite: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LayerDef {
    pub id: String,
    /// Tiles drawn for this layer; empty means it is not drawn.
    pub sprites: Vec<u32>,
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
    pub sprite: u32,
    /// If set, the unit can only be placed where this layer is above zero.
    pub place_on: Option<LayerId>,
    /// Accepts any resource without limit.
    pub stores_any: bool,
    pub rules: Vec<Rule>,
    /// Resources it accepts and how many it holds, derived from `local .. in`.
    pub inputs: Vec<(ItemId, u64)>,
    /// Resources it sends through its front, derived from `local .. out`.
    pub outputs: Vec<ItemId>,
}

impl UnitDef {
    pub fn accepts_items(&self) -> bool {
        self.stores_any || !self.inputs.is_empty()
    }
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
