//! Parser for rule packs: the GlassBox rule language plus one-line declarations.
//!
//! Numbers are whole, because the simulation must stay deterministic. Names
//! must be declared before a rule can use them, so a typo is an error rather
//! than a resource nothing produces.

use crate::content::{
    Amount, BeltDef, Condition, Content, Goal, LayerDef, Link, MapRule, MetricDef, Overlay, Pack, Patch,
    Port, PuzzleDef, ResourceDef, Route, Rule, Scope, TechDef, Test, UnitDef, WorldGen,
};
use crate::world::{Building, Command};
use crate::{BeltId, Dir, ItemId, LayerId, MetricId, Pos, TechId, UnitId};
use std::fmt;

const BELT: &str = "belt";
const DEFAULT_BELT_SPEED: u32 = 16;
/// Scanning a member's reach costs its area, so keep that bounded.
const MAX_REACH: u32 = 32;
const DEFAULT_TUNNEL_RANGE: u32 = 4;
const DEFAULT_LIFT_RANGE: u32 = 1;
/// Networks scan every level they can reach, so keep these bounded too.
const MAX_LEVELS: u32 = 8;
const MAX_SPAN: u32 = 16;
const DEFAULT_SPAN: u32 = 3;
const DEFAULT_RESEARCH_TIME: u32 = 600;

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

    fn color(&self, token: &str) -> Parsed<u32> {
        match u32::from_str_radix(token, 16) {
            Ok(value) if token.len() == 6 => Ok(value),
            _ => self.fail(format!("expected a colour like c8763c, found '{token}'")),
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
    matches!(head, "unitRule" | "rule" | "mapRule" | "zoneRule" | "agentRule" | "research" | "puzzle")
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
    belts: Vec<BeltDef>,
    techs: Vec<TechDef>,
    patches: Vec<Patch>,
    starter: Vec<Command>,
    stock: Vec<(ItemId, u32)>,
    sandbox: bool,
    title: Option<String>,
    summary: String,
    sheet: Option<String>,
    puzzles: Vec<PuzzleDef>,
    levels: Option<u32>,
    span: Option<u32>,
}

fn parse_port(token: &str) -> Option<Port> {
    match token {
        "front" => Some(Port::Front),
        "left" => Some(Port::Left),
        "right" => Some(Port::Right),
        _ => None,
    }
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
        let (mut name, mut sprite, mut tint, mut model) = (id.to_string(), 0, None, None);
        for pair in line.strs()[2..].chunks(2) {
            match pair {
                ["name", value] => name = value.to_string(),
                ["sprite", value] => sprite = line.whole(value)?,
                ["tint", value] => tint = Some(line.color(value)?),
                ["model", value] => model = Some(value.to_string()),
                _ => return line.fail(format!("resource does not understand '{}'", pair[0])),
            }
        }
        self.resources.push(ResourceDef { id: id.to_string(), name, description: String::new(), sprite, tint, model });
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
        let mut layer = LayerDef {
            id: id.to_string(),
            name: id.to_string(),
            sprites: Vec::new(),
            tint: None,
            overlay: None,
            decor: Vec::new(),
        };
        let tokens = line.strs();
        let mut rest = &tokens[2..];
        while !rest.is_empty() {
            rest = match rest {
                ["name", value, tail @ ..] => {
                    layer.name = value.to_string();
                    tail
                }
                ["tint", value, tail @ ..] => {
                    layer.tint = Some(line.color(value)?);
                    tail
                }
                ["haze", color, "full", full, tail @ ..] => {
                    layer.overlay = Some(Overlay { color: line.color(color)?, full: line.positive(full)? });
                    tail
                }
                ["sprites", tail @ ..] => {
                    let count = tail.iter().take_while(|t| t.parse::<u32>().is_ok()).count();
                    if count == 0 {
                        return line.fail("'sprites' needs at least one number");
                    }
                    for value in &tail[..count] {
                        layer.sprites.push(line.whole(value)?);
                    }
                    &tail[count..]
                }
                // Model names run to the end of the line, so `decor` comes last.
                ["decor", tail @ ..] => {
                    if tail.is_empty() {
                        return line.fail("'decor' needs at least one model");
                    }
                    layer.decor.extend(tail.iter().map(|t| t.to_string()));
                    &[]
                }
                other => return line.fail(format!("layer does not understand '{}'", other[0])),
            };
        }
        self.layers.push(layer);
        Ok(())
    }

    /// `belt` declares a tier. The built-in `belt` tier can be adjusted the same way.
    fn declare_belt(&mut self, line: &Line) -> Parsed<()> {
        let id = Self::declared_name(line, "belt")?;
        if id != BELT && self.belts.iter().any(|b| b.id == id) {
            return line.fail(format!("belt '{id}' is declared twice"));
        }
        let mut belt = BeltDef {
            id: id.to_string(),
            name: id.to_string(),
            description: String::new(),
            speed: DEFAULT_BELT_SPEED,
            style: 0,
            model: None,
            corner_model: None,
            cost: Vec::new(),
            locked_by: None,
        };
        if id == BELT {
            belt = self.belts[0].clone();
        }
        for pair in line.strs()[2..].chunks(2) {
            match pair {
                ["name", value] => belt.name = value.to_string(),
                ["speed", value] => belt.speed = line.positive(value)?,
                ["style", value] => belt.style = line.whole(value)?,
                ["model", value] => belt.model = Some(value.to_string()),
                ["corner", value] => belt.corner_model = Some(value.to_string()),
                _ => return line.fail(format!("belt does not understand '{}'", pair[0])),
            }
        }
        if id == BELT {
            self.belts[0] = belt;
        } else {
            self.belts.push(belt);
        }
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
        if self.belts.iter().any(|b| b.id == id) {
            return line.fail(format!("'{id}' is already the name of a belt"));
        }
        let mut unit = UnitDef {
            id: id.to_string(),
            name: id.to_string(),
            description: String::new(),
            sprite: 0,
            tint: None,
            group: "Buildings".to_string(),
            model: None,
            place_on: Vec::new(),
            stores_any: false,
            delivers: false,
            cost: Vec::new(),
            locked_by: None,
            route: None,
            links: Vec::new(),
            rules: Vec::new(),
            inputs: Vec::new(),
            outputs: Vec::new(),
        };
        for pair in line.strs()[2..].chunks(2) {
            match pair {
                ["name", value] => unit.name = value.to_string(),
                ["sprite", value] => unit.sprite = line.whole(value)?,
                ["tint", value] => unit.tint = Some(line.color(value)?),
                ["group", value] => unit.group = value.to_string(),
                ["model", value] => unit.model = Some(value.to_string()),
                ["placeOn", value] => unit.place_on.push(self.layer(line, value)?),
                ["stores", "any"] => unit.stores_any = true,
                ["stores", "shared"] => unit.delivers = true,
                ["routes", "split"] => unit.route = Some(Route::Split),
                ["routes", "filter"] => unit.route = Some(Route::Filter),
                ["routes", "tunnel"] => unit.route = Some(Route::Tunnel { range: DEFAULT_TUNNEL_RANGE }),
                ["routes", "up"] => unit.route = Some(Route::Lift { range: DEFAULT_LIFT_RANGE, down: false }),
                ["routes", "down"] => unit.route = Some(Route::Lift { range: DEFAULT_LIFT_RANGE, down: true }),
                ["routes", other] => {
                    return line.fail(format!("expected split, filter, tunnel, up or down, found '{other}'"));
                }
                ["range", value] => match unit.route {
                    Some(Route::Tunnel { .. }) => {
                        unit.route = Some(Route::Tunnel { range: line.positive(value)? });
                    }
                    Some(Route::Lift { down, .. }) => {
                        unit.route = Some(Route::Lift { range: line.positive(value)?, down });
                    }
                    _ => return line.fail("'range' only applies after 'routes tunnel', 'routes up' or 'routes down'"),
                },
                ["links", value] => {
                    let resource = self.resource(line, value)?;
                    unit.links.push(Link { resource, reach: 0 });
                }
                ["reach", value] => {
                    let reach = line.positive(value)?;
                    if reach > MAX_REACH {
                        return line.fail(format!("'reach' can be at most {MAX_REACH}"));
                    }
                    match unit.links.last_mut() {
                        Some(link) => link.reach = reach,
                        None => return line.fail("'reach' only applies after 'links <resource>'"),
                    }
                }
                _ => return line.fail(format!("unit does not understand '{}'", pair[0])),
            }
        }
        if unit.route.is_some() && (unit.stores_any || unit.delivers) {
            return line.fail("a unit can't both route items and store them");
        }
        self.units.push(unit);
        Ok(())
    }

    fn map_statement(&mut self, line: &Line) -> Parsed<()> {
        match line.strs().as_slice() {
            ["beltSpeed", value] => self.belts[0].speed = line.positive(value)?,
            ["patch", layer, "at", x, y, "size", width, height, "amount", amount] => {
                self.patches.push(Patch {
                    layer: self.layer(line, layer)?,
                    origin: Pos::new(line.whole(x)?, line.whole(y)?),
                    width: line.positive(width)?,
                    height: line.positive(height)?,
                    amount_per_tile: line.positive(amount)?,
                });
            }
            ["sandbox"] => self.sandbox = true,
            ["title", value] => self.title = Some(value.to_string()),
            ["summary", value] => self.summary = value.to_string(),
            ["sheet", value] => self.sheet = Some(value.to_string()),
            ["levels", value] => {
                let levels = line.positive(value)?;
                if levels > MAX_LEVELS {
                    return line.fail(format!("'levels' can be at most {MAX_LEVELS}"));
                }
                self.levels = Some(levels);
            }
            ["span", value] => {
                let span = line.whole(value)?;
                if span > MAX_SPAN {
                    return line.fail(format!("'span' can be at most {MAX_SPAN}"));
                }
                self.span = Some(span);
            }
            ["stock", pairs @ ..] => {
                let parsed = self.cost_pairs(line, pairs)?;
                self.stock.extend(parsed);
            }
            ["cost", what, pairs @ ..] => {
                let cost = self.cost_pairs(line, pairs)?;
                if let Some(belt) = self.belts.iter_mut().find(|b| b.id == *what) {
                    belt.cost = cost;
                } else {
                    let unit = self.unit(line, what)?;
                    self.units[usize::from(unit.0)].cost = cost;
                }
            }
            ["place", what, x, y, dir, options @ ..] => {
                let (mut count, mut level) = (1, 0);
                for pair in options.chunks(2) {
                    match pair {
                        ["count", value] => count = line.positive(value)?,
                        ["level", value] => level = line.whole::<u32>(value)? as i32,
                        _ => return line.fail(format!("place does not understand '{}'", pair[0])),
                    }
                }
                self.place(line, what, Pos::at(line.whole(x)?, line.whole(y)?, level), dir, count)?;
            }
            ["resource", ..] | ["layer", ..] | ["unit", ..] | ["metric", ..] | ["belt", ..] | ["describe", ..] => {}
            _ => return line.fail(format!("this statement isn't understood: '{}'", line.head())),
        }
        Ok(())
    }

    /// Reads `<resource> <n> <resource> <n> ...`.
    fn cost_pairs(&self, line: &Line, tokens: &[&str]) -> Parsed<Vec<(ItemId, u32)>> {
        if tokens.is_empty() || tokens.len() % 2 != 0 {
            return line.fail("expected pairs of a resource and an amount");
        }
        let mut pairs: Vec<(ItemId, u32)> = Vec::new();
        for pair in tokens.chunks(2) {
            let item = self.resource(line, pair[0])?;
            if pairs.iter().any(|(existing, _)| *existing == item) {
                return line.fail(format!("'{}' is listed twice", pair[0]));
            }
            pairs.push((item, line.positive(pair[1])?));
        }
        Ok(pairs)
    }

    fn place(&mut self, line: &Line, what: &str, start: Pos, dir: &str, count: u32) -> Parsed<()> {
        let building = match self.belts.iter().position(|b| b.id == what) {
            Some(index) => Building::Belt(BeltId(index as u16)),
            None => Building::Unit(self.unit(line, what)?),
        };
        let dir = parse_dir(line, dir)?;
        let mut pos = start;
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
            ["local", resource, "out", amount, side] | [resource, "out", amount, side] => {
                let item = self.resource(line, resource)?;
                let port = match parse_port(side) {
                    Some(port) => port,
                    None => return line.fail(format!("expected front, left or right, found '{side}'")),
                };
                self.scoped(line, Scope::Local(item), "out", amount, rule)?;
                if port != Port::Front {
                    rule.sides.push((item, port));
                }
            }
            ["global", resource, direction, amount] => {
                let scope = Scope::Global(self.resource(line, resource)?);
                self.scoped(line, scope, direction, amount, rule)?;
            }
            ["network", resource, direction, amount] => {
                let scope = Scope::Network(self.resource(line, resource)?);
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
            sides: Vec::new(),
            locked_by: None,
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
        let definition = &mut self.units[usize::from(unit.0)];
        if definition.route.is_some() {
            return header.fail(format!("unit '{target}' routes items, so it can't have rules"));
        }
        for give in &rule.gives {
            if let Scope::Local(item) = give.scope {
                let port = rule.port_of(item);
                let gives_item = |other: &&Rule| other.gives.iter().any(|g| g.scope == give.scope);
                if definition.rules.iter().filter(gives_item).any(|other| other.port_of(item) != port) {
                    return header.fail(format!(
                        "unit '{target}' would send the same resource from two different sides"
                    ));
                }
            }
        }
        definition.rules.push(rule);
        Ok(())
    }

    /// Marks whatever `name` refers to as needing `tech`.
    fn lock(&mut self, line: &Line, name: &str, tech: TechId) -> Parsed<()> {
        let mut found = false;
        let mut already = false;
        let mut mark = |slot: &mut Option<TechId>| {
            found = true;
            already |= slot.is_some();
            *slot = Some(tech);
        };
        self.belts.iter_mut().filter(|b| b.id == name).for_each(|b| mark(&mut b.locked_by));
        self.units.iter_mut().filter(|u| u.id == name).for_each(|u| mark(&mut u.locked_by));
        self.units
            .iter_mut()
            .flat_map(|u| &mut u.rules)
            .filter(|r| r.name == name)
            .for_each(|r| mark(&mut r.locked_by));
        if !found {
            return line.fail(format!("'{name}' isn't a belt, unit or rule, so it can't be unlocked"));
        }
        if already {
            return line.fail(format!("'{name}' is unlocked by more than one research"));
        }
        Ok(())
    }

    fn tech(&self, line: &Line, id: &str) -> Parsed<TechId> {
        match self.techs.iter().position(|t| t.id == id) {
            Some(index) => Ok(TechId(index as u16)),
            None => line.fail(format!("unknown research '{id}'")),
        }
    }

    fn add_research(&mut self, blocks: &[&Block]) -> Parsed<()> {
        for block in blocks {
            let id = match block.header.strs().as_slice() {
                [_, id] => id.to_string(),
                _ => return block.header.fail("'research' needs exactly one name"),
            };
            if self.techs.iter().any(|t| t.id == id) {
                return block.header.fail(format!("research '{id}' is declared twice"));
            }
            self.techs.push(TechDef {
                name: id.clone(),
                description: String::new(),
                id,
                requires: Vec::new(),
                cost: Vec::new(),
                time: DEFAULT_RESEARCH_TIME,
            });
        }
        for (index, block) in blocks.iter().enumerate() {
            let tech = TechId(index as u16);
            for line in &block.body {
                match line.strs().as_slice() {
                    ["name", value] => self.techs[index].name = value.to_string(),
                    ["time", value] => self.techs[index].time = line.positive(value)?,
                    ["cost", pairs @ ..] => self.techs[index].cost = self.cost_pairs(line, pairs)?,
                    ["requires", names @ ..] if !names.is_empty() => {
                        for name in names {
                            let required = self.tech(line, name)?;
                            self.techs[index].requires.push(required);
                        }
                    }
                    ["unlocks", names @ ..] if !names.is_empty() => {
                        for name in names {
                            self.lock(line, name, tech)?;
                        }
                    }
                    _ => return line.fail(format!("research does not understand '{}'", line.head())),
                }
            }
        }
        self.check_research_order(blocks)
    }

    /// Repeatedly finds research whose requirements are all met; anything left is a loop.
    fn check_research_order(&self, blocks: &[&Block]) -> Parsed<()> {
        let mut done = vec![false; self.techs.len()];
        loop {
            let ready: Vec<usize> = (0..self.techs.len())
                .filter(|&i| !done[i] && self.techs[i].requires.iter().all(|t| done[usize::from(t.0)]))
                .collect();
            if ready.is_empty() {
                break;
            }
            ready.into_iter().for_each(|i| done[i] = true);
        }
        match done.iter().position(|finished| !finished) {
            Some(index) => {
                let id = &self.techs[index].id;
                blocks[index].header.fail(format!("research '{id}' can never start: its requirements form a loop"))
            }
            None => Ok(()),
        }
    }

    /// `describe <id> "text"` gives a belt, unit, research or resource its explanation.
    fn describe(&mut self, line: &Line) -> Parsed<()> {
        let (id, text) = match line.strs().as_slice() {
            ["describe", id, text] => (id.to_string(), text.trim().to_string()),
            _ => return line.fail("expected 'describe <name> \"what it is\"'"),
        };
        if text.is_empty() {
            return line.fail(format!("the description of '{id}' is empty"));
        }
        let mut slots: Vec<&mut String> = Vec::new();
        slots.extend(self.belts.iter_mut().filter(|b| b.id == id).map(|b| &mut b.description));
        slots.extend(self.units.iter_mut().filter(|u| u.id == id).map(|u| &mut u.description));
        slots.extend(self.techs.iter_mut().filter(|t| t.id == id).map(|t| &mut t.description));
        slots.extend(self.resources.iter_mut().filter(|r| r.id == id).map(|r| &mut r.description));
        match slots.as_mut_slice() {
            [] => line.fail(format!("'{id}' isn't a belt, unit, research or resource, so it can't be described")),
            [slot] if !slot.is_empty() => line.fail(format!("'{id}' is described twice")),
            [slot] => {
                **slot = text;
                Ok(())
            }
            _ => line.fail(format!("'{id}' names more than one thing, so the description would be ambiguous")),
        }
    }

    fn building(&self, line: &Line, id: &str) -> Parsed<Building> {
        match self.belts.iter().position(|b| b.id == id) {
            Some(index) => Ok(Building::Belt(BeltId(index as u16))),
            None => match self.units.iter().position(|u| u.id == id) {
                Some(index) => Ok(Building::Unit(UnitId(index as u16))),
                None => line.fail(format!("'{id}' isn't a belt or a unit")),
            },
        }
    }

    fn goal(&self, line: &Line) -> Parsed<Goal> {
        Ok(match line.strs().as_slice() {
            ["goal", "deliver", item, n] => Goal::Deliver(self.resource(line, item)?, line.positive(n)?),
            ["goal", "make", item, n] => Goal::Make(self.resource(line, item)?, line.positive(n)?),
            ["goal", "stock", item, n] => Goal::Stock(self.resource(line, item)?, line.positive(n)?),
            ["goal", "metric", metric, n] => Goal::Metric(self.metric(line, metric)?, line.positive(n)?),
            ["goal", "research", tech] => Goal::Research(self.tech(line, tech)?),
            _ => {
                return line.fail(
                    "expected 'goal deliver|make|stock <resource> <n>', 'goal metric <metric> <n>' or 'goal research <name>'",
                )
            }
        })
    }

    /// `puzzle <id> ... end`: goals, and optionally a map of its own.
    fn add_puzzle(&mut self, block: &Block) -> Parsed<()> {
        let id = match block.header.strs().as_slice() {
            [_, id] => id.to_string(),
            _ => return block.header.fail("'puzzle' needs exactly one name"),
        };
        if self.puzzles.iter().any(|p| p.id == id) {
            return block.header.fail(format!("puzzle '{id}' is declared twice"));
        }
        let mut puzzle = PuzzleDef {
            name: id.clone(),
            id,
            description: String::new(),
            fresh: None,
            researched: Vec::new(),
            only: Vec::new(),
            goals: Vec::new(),
            limit: None,
        };
        // The pack's own map, stock and opening, which a fresh puzzle replaces piece by piece.
        let pack_map = (std::mem::take(&mut self.patches), std::mem::take(&mut self.starter), std::mem::take(&mut self.stock));
        let (mut own_map, mut own_starter, mut own_stock) = (false, false, false);
        let mut map_line: Option<&Line> = None;
        let mut result = Ok(());
        for line in &block.body {
            let step = match line.strs().as_slice() {
                ["name", value] => {
                    puzzle.name = value.to_string();
                    Ok(())
                }
                ["about", value] => {
                    puzzle.description = value.trim().to_string();
                    Ok(())
                }
                ["fresh"] => {
                    puzzle.fresh = Some(WorldGen::default());
                    Ok(())
                }
                ["limit", value] => line.positive(value).map(|v| puzzle.limit = Some(v)),
                ["only", names @ ..] if !names.is_empty() => names.iter().try_for_each(|name| {
                    let building = self.building(line, name)?;
                    puzzle.only.push(building);
                    Ok(())
                }),
                ["researched", names @ ..] if !names.is_empty() => {
                    map_line = Some(line);
                    names.iter().try_for_each(|name| {
                        let tech = self.tech(line, name)?;
                        puzzle.researched.push(tech);
                        Ok(())
                    })
                }
                ["goal", ..] => self.goal(line).map(|goal| puzzle.goals.push(goal)),
                ["stock", "none"] => {
                    own_stock = true;
                    map_line = Some(line);
                    Ok(())
                }
                ["place", "none"] => {
                    own_starter = true;
                    map_line = Some(line);
                    Ok(())
                }
                [head @ ("patch" | "stock" | "place"), ..] => {
                    own_map |= *head == "patch";
                    own_stock |= *head == "stock";
                    own_starter |= *head == "place";
                    map_line = Some(line);
                    self.map_statement(line)
                }
                _ => line.fail(format!("puzzle does not understand '{}'", line.head())),
            };
            if step.is_err() {
                result = step;
                break;
            }
        }
        let (patches, starter, stock) =
            (std::mem::take(&mut self.patches), std::mem::take(&mut self.starter), std::mem::take(&mut self.stock));
        let (pack_patches, pack_starter, pack_stock) = pack_map;
        if let Some(fresh) = &mut puzzle.fresh {
            fresh.patches = if own_map { patches } else { pack_patches.clone() };
            fresh.starter = if own_starter { starter } else { pack_starter.clone() };
            fresh.stock = if own_stock { stock } else { pack_stock.clone() };
        }
        self.patches = pack_patches;
        self.starter = pack_starter;
        self.stock = pack_stock;
        result?;
        if let (None, Some(line)) = (&puzzle.fresh, map_line) {
            return line.fail("only a 'fresh' puzzle has a map, stock, buildings or research of its own");
        }
        if puzzle.goals.is_empty() {
            return block.header.fail(format!("puzzle '{}' has no goals", puzzle.id));
        }
        self.puzzles.push(puzzle);
        Ok(())
    }

    /// A sandbox pack has everything available for free.
    fn open_everything(&mut self) {
        for belt in &mut self.belts {
            belt.cost.clear();
            belt.locked_by = None;
        }
        for unit in &mut self.units {
            unit.cost.clear();
            unit.locked_by = None;
            unit.rules.iter_mut().for_each(|rule| rule.locked_by = None);
        }
    }

    fn finish(mut self) -> Pack {
        if self.sandbox {
            self.open_everything();
        }
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
                        if !unit.outputs.iter().any(|(output, _)| *output == item) {
                            unit.outputs.push((item, rule.port_of(item)));
                        }
                    }
                }
                for entry in rule.takes.iter().chain(&rule.gives) {
                    if let Scope::Network(resource) = entry.scope {
                        if !unit.links.iter().any(|link| link.resource == resource) {
                            unit.links.push(Link { resource, reach: 0 });
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
                belts: self.belts,
                techs: self.techs,
                levels: self.levels.unwrap_or(1),
                span: self.span.unwrap_or(DEFAULT_SPAN),
                title: self.title.unwrap_or_else(|| "Untitled rules".to_string()),
                summary: self.summary,
                sheet: self.sheet,
                puzzles: self.puzzles,
            },
            worldgen: WorldGen { patches: self.patches, starter: self.starter, stock: self.stock },
        }
    }
}

impl Pack {
    /// Reads a pack, or names the first problem and the line it is on.
    pub fn parse(text: &str) -> Result<Pack, PackError> {
        let (top, blocks) = split_lines(text)?;
        let mut builder = Builder::default();
        builder.belts.push(BeltDef {
            id: BELT.to_string(),
            name: "Belt".to_string(),
            description: String::new(),
            speed: DEFAULT_BELT_SPEED,
            style: 0,
            model: None,
            corner_model: None,
            cost: Vec::new(),
            locked_by: None,
        });
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
        for line in top.iter().filter(|l| l.head() == "belt") {
            builder.declare_belt(line)?;
        }
        for line in top.iter().filter(|l| l.head() == "unit") {
            builder.declare_unit(line)?;
        }
        for line in &top {
            builder.map_statement(line)?;
        }
        let (research, rules): (Vec<&Block>, Vec<&Block>) =
            blocks.iter().partition(|block| block.header.head() == "research");
        let (puzzles, rules): (Vec<&Block>, Vec<&Block>) =
            rules.into_iter().partition(|block| block.header.head() == "puzzle");
        for block in rules {
            builder.add_rule(block)?;
        }
        builder.add_research(&research)?;
        for block in puzzles {
            builder.add_puzzle(block)?;
        }
        // Last, so research can be described too.
        for line in top.iter().filter(|l| l.head() == "describe") {
            builder.describe(line)?;
        }
        Ok(builder.finish())
    }
}
