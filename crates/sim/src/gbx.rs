//! Parser for rule packs: the GlassBox rule language plus one-line declarations.
//!
//! Numbers are whole, because the simulation must stay deterministic. Names
//! must be declared before a rule can use them, so a typo is an error rather
//! than a resource nothing produces.

use crate::content::{
    Amount, Condition, Content, LayerDef, MapRule, MetricDef, Pack, Patch, ResourceDef, Rule,
    Scope, Test, UnitDef, WorldGen,
};
use crate::world::{Building, Command};
use crate::{Dir, ItemId, LayerId, MetricId, Pos, UnitId};
use std::fmt;

const BELT: &str = "belt";
const DEFAULT_BELT_SPEED: u32 = 16;

/// What went wrong in a pack, and on which line (counting from 1).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PackError {
    pub line: usize,
    pub message: String,
}

impl fmt::Display for PackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line {}: {}", self.line, self.message)
    }
}

impl std::error::Error for PackError {}

type Parsed<T> = Result<T, PackError>;

struct Line {
    number: usize,
    tokens: Vec<String>,
}

impl Line {
    fn fail<T>(&self, message: impl Into<String>) -> Parsed<T> {
        Err(PackError { line: self.number, message: message.into() })
    }

    fn head(&self) -> &str {
        &self.tokens[0]
    }

    fn strs(&self) -> Vec<&str> {
        self.tokens.iter().map(String::as_str).collect()
    }

    fn whole<T: std::str::FromStr>(&self, token: &str) -> Parsed<T> {
        match token.parse() {
            Ok(value) => Ok(value),
            Err(_) => self.fail(format!("expected a whole number, found '{token}'")),
        }
    }

    fn positive(&self, token: &str) -> Parsed<u32> {
        match self.whole::<u32>(token)? {
            0 => self.fail("this value must be at least 1"),
            value => Ok(value),
        }
    }
}

struct Block {
    header: Line,
    body: Vec<Line>,
}

/// Splits a line into tokens, keeping "quoted text" whole and dropping comments.
fn tokenize(text: &str, number: usize) -> Parsed<Vec<String>> {
    let mut tokens = Vec::new();
    let mut chars = text.chars().peekable();
    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
        } else if c == '#' {
            break;
        } else if c == '"' {
            chars.next();
            let mut token = String::new();
            loop {
                match chars.next() {
                    Some('"') => break,
                    Some(inner) => token.push(inner),
                    None => {
                        let message = "this quoted text is never closed".to_string();
                        return Err(PackError { line: number, message });
                    }
                }
            }
            tokens.push(token);
        } else {
            let mut token = String::new();
            while let Some(&inner) = chars.peek() {
                if inner.is_whitespace() {
                    break;
                }
                token.push(inner);
                chars.next();
            }
            if token.starts_with("//") {
                break;
            }
            tokens.push(token);
        }
    }
    Ok(tokens)
}

fn is_block_start(head: &str) -> bool {
    matches!(head, "unitRule" | "rule" | "mapRule" | "zoneRule" | "agentRule")
}

fn split_lines(text: &str) -> Parsed<(Vec<Line>, Vec<Block>)> {
    let mut top = Vec::new();
    let mut blocks = Vec::new();
    let mut open: Option<Block> = None;
    for (index, raw) in text.lines().enumerate() {
        let number = index + 1;
        let tokens = tokenize(raw, number)?;
        if tokens.is_empty() {
            continue;
        }
        let line = Line { number, tokens };
        match open.take() {
            Some(mut block) if line.head() == "end" => {
                block.body.shrink_to_fit();
                blocks.push(block);
            }
            Some(block) if is_block_start(line.head()) => {
                let name = block.header.tokens.get(1).cloned().unwrap_or_default();
                return line.fail(format!("rule '{name}' needs an 'end' before this one starts"));
            }
            Some(mut block) => {
                block.body.push(line);
                open = Some(block);
            }
            None if is_block_start(line.head()) => {
                open = Some(Block { header: line, body: Vec::new() });
            }
            None if line.head() == "end" => return line.fail("'end' without a rule to close"),
            None => top.push(line),
        }
    }
    if let Some(block) = open {
        return block.header.fail("this rule is never closed with 'end'");
    }
    Ok((top, blocks))
}

fn parse_dir(line: &Line, token: &str) -> Parsed<Dir> {
    match token {
        "north" => Ok(Dir::North),
        "east" => Ok(Dir::East),
        "south" => Ok(Dir::South),
        "west" => Ok(Dir::West),
        _ => line.fail(format!("expected north, east, south or west, found '{token}'")),
    }
}

fn parse_test(token: &str) -> Option<Test> {
    match token {
        "atLeast" => Some(Test::AtLeast),
        "atMost" => Some(Test::AtMost),
        "greater" => Some(Test::Greater),
        "less" => Some(Test::Less),
        "is" => Some(Test::Is),
        _ => None,
    }
}

#[derive(Default)]
struct Builder {
    resources: Vec<ResourceDef>,
    layers: Vec<LayerDef>,
    units: Vec<UnitDef>,
    metrics: Vec<MetricDef>,
    map_rules: Vec<MapRule>,
    patches: Vec<Patch>,
    starter: Vec<Command>,
    belt_speed: Option<u32>,
}

impl Builder {
    fn resource(&self, line: &Line, id: &str) -> Parsed<ItemId> {
        match self.resources.iter().position(|r| r.id == id) {
            Some(index) => Ok(ItemId(index as u16)),
            None => line.fail(format!("unknown resource '{id}'; declare it with 'resource {id}'")),
        }
    }

    fn layer(&self, line: &Line, id: &str) -> Parsed<LayerId> {
        match self.layers.iter().position(|l| l.id == id) {
            Some(index) => Ok(LayerId(index as u16)),
            None => line.fail(format!("unknown map layer '{id}'; declare it with 'layer {id}'")),
        }
    }

    fn metric(&self, line: &Line, id: &str) -> Parsed<MetricId> {
        match self.metrics.iter().position(|m| m.id == id) {
            Some(index) => Ok(MetricId(index as u16)),
            None => line.fail(format!("unknown metric '{id}'; declare it with 'metric {id}'")),
        }
    }

    fn unit(&self, line: &Line, id: &str) -> Parsed<UnitId> {
        match self.units.iter().position(|u| u.id == id) {
            Some(index) => Ok(UnitId(index as u16)),
            None => line.fail(format!("unknown unit '{id}'; declare it with 'unit {id}'")),
        }
    }

    fn declared_name<'a>(line: &'a Line, kind: &str) -> Parsed<&'a str> {
        match line.tokens.get(1) {
            Some(id) => Ok(id),
            None => line.fail(format!("'{kind}' needs a name")),
        }
    }

    fn declare_resource(&mut self, line: &Line) -> Parsed<()> {
        let id = Self::declared_name(line, "resource")?;
        if self.resources.iter().any(|r| r.id == id) {
            return line.fail(format!("resource '{id}' is declared twice"));
        }
        let (mut name, mut sprite) = (id.to_string(), 0);
        for pair in line.strs()[2..].chunks(2) {
            match pair {
                ["name", value] => name = value.to_string(),
                ["sprite", value] => sprite = line.whole(value)?,
                _ => return line.fail(format!("resource does not understand '{}'", pair[0])),
            }
        }
        self.resources.push(ResourceDef { id: id.to_string(), name, sprite });
        Ok(())
    }

    fn declare_metric(&mut self, line: &Line) -> Parsed<()> {
        let id = Self::declared_name(line, "metric")?;
        if self.metrics.iter().any(|m| m.id == id) {
            return line.fail(format!("metric '{id}' is declared twice"));
        }
        let name = match &line.strs()[2..] {
            [] => id.to_string(),
            ["name", value] => value.to_string(),
            other => return line.fail(format!("metric does not understand '{}'", other[0])),
        };
        self.metrics.push(MetricDef { id: id.to_string(), name });
        Ok(())
    }

    fn declare_layer(&mut self, line: &Line) -> Parsed<()> {
        let id = Self::declared_name(line, "layer")?;
        if self.layers.iter().any(|l| l.id == id) {
            return line.fail(format!("layer '{id}' is declared twice"));
        }
        let mut sprites = Vec::new();
        match line.strs().get(2..) {
            Some([]) | None => {}
            Some(["sprites", values @ ..]) if !values.is_empty() => {
                for value in values {
                    sprites.push(line.whole(value)?);
                }
            }
            Some(other) => return line.fail(format!("layer does not understand '{}'", other[0])),
        }
        self.layers.push(LayerDef { id: id.to_string(), sprites });
        Ok(())
    }

    fn declare_unit(&mut self, line: &Line) -> Parsed<()> {
        let id = Self::declared_name(line, "unit")?;
        if id == BELT {
            return line.fail("'belt' is built into the engine and can't be redefined");
        }
        if self.units.iter().any(|u| u.id == id) {
            return line.fail(format!("unit '{id}' is declared twice"));
        }
        let mut unit = UnitDef {
            id: id.to_string(),
            name: id.to_string(),
            sprite: 0,
            place_on: None,
            stores_any: false,
            rules: Vec::new(),
            inputs: Vec::new(),
            outputs: Vec::new(),
        };
        for pair in line.strs()[2..].chunks(2) {
            match pair {
                ["name", value] => unit.name = value.to_string(),
                ["sprite", value] => unit.sprite = line.whole(value)?,
                ["placeOn", value] => unit.place_on = Some(self.layer(line, value)?),
                ["stores", "any"] => unit.stores_any = true,
                _ => return line.fail(format!("unit does not understand '{}'", pair[0])),
            }
        }
        self.units.push(unit);
        Ok(())
    }

    fn map_statement(&mut self, line: &Line) -> Parsed<()> {
        match line.strs().as_slice() {
            ["beltSpeed", value] => self.belt_speed = Some(line.positive(value)?),
            ["patch", layer, "at", x, y, "size", width, height, "amount", amount] => {
                self.patches.push(Patch {
                    layer: self.layer(line, layer)?,
                    origin: Pos::new(line.whole(x)?, line.whole(y)?),
                    width: line.positive(width)?,
                    height: line.positive(height)?,
                    amount_per_tile: line.positive(amount)?,
                });
            }
            ["place", what, x, y, dir] => self.place(line, what, x, y, dir, 1)?,
            ["place", what, x, y, dir, "count", count] => {
                let count = line.positive(count)?;
                self.place(line, what, x, y, dir, count)?;
            }
            ["resource", ..] | ["layer", ..] | ["unit", ..] | ["metric", ..] => {}
            _ => return line.fail(format!("this statement isn't understood: '{}'", line.head())),
        }
        Ok(())
    }

    fn place(&mut self, line: &Line, what: &str, x: &str, y: &str, dir: &str, count: u32) -> Parsed<()> {
        let building =
            if what == BELT { Building::Belt } else { Building::Unit(self.unit(line, what)?) };
        let dir = parse_dir(line, dir)?;
        let mut pos = Pos::new(line.whole(x)?, line.whole(y)?);
        for _ in 0..count {
            self.starter.push(Command::Place { pos, dir, building });
            pos = pos.step(dir);
        }
        Ok(())
    }

    fn scoped(&self, line: &Line, scope: Scope, direction: &str, amount: &str, rule: &mut Rule) -> Parsed<()> {
        let entry = Amount { scope, amount: line.positive(amount)? };
        match direction {
            "in" if rule.takes.iter().any(|take| take.scope == scope) => {
                return line.fail("this rule already takes that; combine them into one amount");
            }
            "in" => rule.takes.push(entry),
            "out" => rule.gives.push(entry),
            _ => return line.fail(format!("expected 'in' or 'out', found '{direction}'")),
        }
        Ok(())
    }

    fn condition(&self, line: &Line, layer: &str, test: &str, value: &str, rule: &mut Rule) -> Parsed<()> {
        let test = match parse_test(test) {
            Some(test) => test,
            None => return line.fail(format!("'{test}' is not a condition or a direction")),
        };
        let layer = self.layer(line, layer)?;
        rule.conditions.push(Condition { layer, test, value: line.whole(value)? });
        Ok(())
    }

    fn rule_statement(&self, line: &Line, target: &mut String, rule: &mut Rule) -> Parsed<()> {
        match line.strs().as_slice() {
            ["appliesTo", unit] => *target = unit.to_string(),
            ["rate", rate] => rule.rate = line.positive(rate)?,
            ["local", resource, direction, amount] | [resource, direction @ ("in" | "out"), amount] => {
                let scope = Scope::Local(self.resource(line, resource)?);
                self.scoped(line, scope, direction, amount, rule)?;
            }
            ["global", resource, direction, amount] => {
                let scope = Scope::Global(self.resource(line, resource)?);
                self.scoped(line, scope, direction, amount, rule)?;
            }
            ["map", layer, "out", amount, "radius", radius] => {
                let scope = Scope::Map { layer: self.layer(line, layer)?, radius: line.whole(radius)? };
                self.scoped(line, scope, "out", amount, rule)?;
            }
            ["map", layer, direction @ ("in" | "out"), amount] => {
                let scope = Scope::Map { layer: self.layer(line, layer)?, radius: 0 };
                self.scoped(line, scope, direction, amount, rule)?;
            }
            ["map", layer, test, value] | ["test", "map", layer, test, value] => {
                self.condition(line, layer, test, value, rule)?;
            }
            ["log", metric, amount] => rule.logs.push((self.metric(line, metric)?, line.positive(amount)?)),
            ["log", flag, metric, amount] if flag.starts_with("-timestamp=") => {
                rule.logs.push((self.metric(line, metric)?, line.positive(amount)?));
            }
            ["successEvent", ..] | ["onFail", ..] => {}
            [keyword @ ("agent" | "options"), ..] => {
                return line.fail(format!("'{keyword}' is GlassBox syntax OpenFactor doesn't support yet"));
            }
            _ => return line.fail(format!("this rule statement isn't understood: '{}'", line.head())),
        }
        Ok(())
    }

    fn map_rule_statement(&self, line: &Line, rule: &mut Rule) -> Parsed<()> {
        match line.strs().as_slice() {
            ["rate", rate] => rule.rate = line.positive(rate)?,
            ["map", _, "out", _, "radius", _] => {
                return line.fail("a mapRule changes one tile at a time, so 'radius' isn't allowed");
            }
            ["map", layer, direction @ ("in" | "out"), amount] => {
                let scope = Scope::Map { layer: self.layer(line, layer)?, radius: 0 };
                self.scoped(line, scope, direction, amount, rule)?;
            }
            ["map", layer, test, value] | ["test", "map", layer, test, value] => {
                self.condition(line, layer, test, value, rule)?;
            }
            _ => return line.fail("a mapRule can only use 'rate' and 'map' statements"),
        }
        Ok(())
    }

    /// The layer whose tiles a map rule can possibly apply to.
    fn map_rule_anchor(rule: &Rule) -> Option<LayerId> {
        let from_take = rule.takes.iter().find_map(|take| match take.scope {
            Scope::Map { layer, .. } => Some(layer),
            _ => None,
        });
        from_take.or_else(|| {
            let only_above_zero = |c: &&Condition| match c.test {
                Test::AtLeast | Test::Is => c.value > 0,
                Test::Greater => true,
                Test::AtMost | Test::Less => false,
            };
            rule.conditions.iter().find(only_above_zero).map(|c| c.layer)
        })
    }

    fn new_rule(name: &str) -> Rule {
        Rule {
            name: name.to_string(),
            rate: 1,
            conditions: Vec::new(),
            takes: Vec::new(),
            gives: Vec::new(),
            logs: Vec::new(),
        }
    }

    fn add_map_rule(&mut self, block: &Block, name: &str) -> Parsed<()> {
        let mut rule = Self::new_rule(name);
        for line in &block.body {
            self.map_rule_statement(line, &mut rule)?;
        }
        if rule.takes.is_empty() && rule.gives.is_empty() {
            return block.header.fail(format!("rule '{name}' doesn't take or give anything"));
        }
        let anchor = match Self::map_rule_anchor(&rule) {
            Some(anchor) => anchor,
            None => {
                return block.header.fail(format!(
                    "mapRule '{name}' could apply to endless empty tiles; add 'map <layer> in <n>' or 'map <layer> atLeast <n>'"
                ));
            }
        };
        self.map_rules.push(MapRule { rule, anchor });
        Ok(())
    }

    fn add_rule(&mut self, block: &Block) -> Parsed<()> {
        let header = &block.header;
        if !matches!(header.head(), "unitRule" | "rule" | "mapRule") {
            return header.fail(format!("'{}' is GlassBox syntax OpenFactor doesn't support yet", header.head()));
        }
        let name = match header.strs().as_slice() {
            [_, name] => name.to_string(),
            _ => return header.fail(format!("'{}' needs exactly one name", header.head())),
        };
        if header.head() == "mapRule" {
            return self.add_map_rule(block, &name);
        }
        let mut rule = Self::new_rule(&name);
        let mut target = name.clone();
        for line in &block.body {
            self.rule_statement(line, &mut target, &mut rule)?;
        }
        if rule.takes.is_empty() && rule.gives.is_empty() && rule.logs.is_empty() {
            return header.fail(format!("rule '{name}' doesn't take or give anything"));
        }
        let unit = self.unit(header, &target)?;
        self.units[usize::from(unit.0)].rules.push(rule);
        Ok(())
    }

    fn finish(mut self) -> Pack {
        for unit in &mut self.units {
            for rule in &unit.rules {
                for take in &rule.takes {
                    if let Scope::Local(item) = take.scope {
                        let limit = u64::from(take.amount) * 2;
                        match unit.inputs.iter_mut().find(|(input, _)| *input == item) {
                            Some(entry) => entry.1 = entry.1.max(limit),
                            None => unit.inputs.push((item, limit)),
                        }
                    }
                }
                for give in &rule.gives {
                    if let Scope::Local(item) = give.scope {
                        if !unit.outputs.contains(&item) {
                            unit.outputs.push(item);
                        }
                    }
                }
            }
        }
        Pack {
            content: Content {
                resources: self.resources,
                layers: self.layers,
                units: self.units,
                metrics: self.metrics,
                map_rules: self.map_rules,
                belt_speed: self.belt_speed.unwrap_or(DEFAULT_BELT_SPEED),
            },
            worldgen: WorldGen { patches: self.patches, starter: self.starter },
        }
    }
}

impl Pack {
    /// Reads a pack, or names the first problem and the line it is on.
    pub fn parse(text: &str) -> Result<Pack, PackError> {
        let (top, blocks) = split_lines(text)?;
        let mut builder = Builder::default();
        // Declarations come first so a pack can be written in any order.
        for line in top.iter().filter(|l| l.head() == "resource") {
            builder.declare_resource(line)?;
        }
        for line in top.iter().filter(|l| l.head() == "metric") {
            builder.declare_metric(line)?;
        }
        for line in top.iter().filter(|l| l.head() == "layer") {
            builder.declare_layer(line)?;
        }
        for line in top.iter().filter(|l| l.head() == "unit") {
            builder.declare_unit(line)?;
        }
        for line in &top {
            builder.map_statement(line)?;
        }
        for block in &blocks {
            builder.add_rule(block)?;
        }
        Ok(builder.finish())
    }
}
