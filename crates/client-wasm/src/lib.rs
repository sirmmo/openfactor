//! Browser sandbox bindings: a plain numeric ABI, so no JS glue generator is needed.
//!
//! Edits are queued and applied on the next `of_step`. A returned pointer is
//! only valid until the next call, because wasm memory can move when it grows.

use openfactor_sim::{
    BeltId, Building, Command, Dir, ItemId, MetricId, Pack, Port, Pos, Route, Rule, Scope, Status, UnitId,
    World,
};
use std::cell::RefCell;

const BASE_PACK: &str = include_str!("../../../packs/base.gbx");
const DIRS: [Dir; 4] = [Dir::North, Dir::East, Dir::South, Dir::West];

struct Game {
    world: World,
    queued: Vec<Command>,
    snapshot: Vec<i32>,
    pack_json: String,
    inspection: String,
}

impl Game {
    fn new() -> Game {
        let pack = Pack::parse(BASE_PACK).expect("the base pack is checked by a test");
        Game {
            world: World::new(&pack),
            queued: pack.worldgen.starter.clone(),
            snapshot: Vec::new(),
            pack_json: pack_json(&pack),
            inspection: String::new(),
        }
    }

    /// Building codes: belt tiers come first, then the pack's units.
    fn belt_count(&self) -> i32 {
        self.world.content().belts().len() as i32
    }

    fn building_code(&self, building: Building) -> i32 {
        match building {
            Building::Belt(belt) => i32::from(belt.0),
            Building::Unit(unit) => self.belt_count() + i32::from(unit.0),
        }
    }

    fn building_from_code(&self, kind: i32) -> Building {
        if kind < self.belt_count() {
            Building::Belt(BeltId(kind as u16))
        } else {
            // An out-of-range code is refused by the simulation as an unknown unit.
            Building::Unit(UnitId((kind - self.belt_count()) as u16))
        }
    }

    fn write_snapshot(&mut self) {
        let mut out = std::mem::take(&mut self.snapshot);
        let layers = self.world.content().layers();
        out.clear();

        let mark = reserve_count(&mut out);
        for (pos, layer, _) in self.world.layer_tiles() {
            if !layers[usize::from(layer.0)].sprites.is_empty() {
                out.extend([pos.x, pos.y, i32::from(layer.0)]);
            }
        }
        finish_count(&mut out, mark, 3);

        let mark = reserve_count(&mut out);
        for (pos, layer, value) in self.world.layer_tiles() {
            if let Some(overlay) = layers[usize::from(layer.0)].overlay {
                let strength = value.min(u64::from(overlay.full)) * 255 / u64::from(overlay.full);
                out.extend([pos.x, pos.y, i32::from(layer.0), strength as i32]);
            }
        }
        finish_count(&mut out, mark, 4);

        let mark = reserve_count(&mut out);
        for (pos, dir, building) in self.world.buildings() {
            let status = match building {
                Building::Unit(_) => self.world.status(pos).map_or(0, status_code),
                Building::Belt(_) => 0,
            };
            out.extend([pos.x, pos.y, dir_code(dir), self.building_code(building), status]);
        }
        finish_count(&mut out, mark, 5);

        let mark = reserve_count(&mut out);
        for (pos, dir, offset, item) in self.world.belt_items() {
            out.extend([pos.x, pos.y, dir_code(dir), offset as i32, i32::from(item.0)]);
        }
        finish_count(&mut out, mark, 5);
        self.snapshot = out;
    }

    /// The building at `pos` as JSON text, or `null`.
    fn inspection_json(&self, pos: Pos) -> String {
        let view = match self.world.inspect(pos) {
            Some(view) => view,
            None => return "null".to_string(),
        };
        let pairs = |list: Vec<String>| format!("[{}]", list.join(","));
        let held = view.held.iter().map(|(item, count)| format!("[{},{}]", item.0, count)).collect();
        let networks = view
            .networks
            .iter()
            .map(|n| format!("[{},{},{},{}]", n.resource.0, n.members, n.producers, n.stock))
            .collect();
        let setting = view.setting.map_or("null".to_string(), |value| value.to_string());
        let job = view.job.map_or("null".to_string(), |(rule, left)| format!("[{rule},{left}]"));
        let missing = match view.status {
            Status::NoNetwork(item) => i32::from(item.0),
            _ => -1,
        };
        format!(
            r#"{{"kind":{},"dir":{},"status":{},"missing":{},"setting":{},"job":{},"held":{},"networks":{}}}"#,
            self.building_code(view.building),
            dir_code(view.dir),
            status_code(view.status),
            missing,
            setting,
            job,
            pairs(held),
            pairs(networks)
        )
    }
}

fn reserve_count(out: &mut Vec<i32>) -> usize {
    out.push(0);
    out.len()
}

fn finish_count(out: &mut [i32], mark: usize, record_size: usize) {
    out[mark - 1] = ((out.len() - mark) / record_size) as i32;
}

fn dir_code(dir: Dir) -> i32 {
    DIRS.iter().position(|&d| d == dir).expect("all directions are listed") as i32
}

fn status_code(status: Status) -> i32 {
    match status {
        Status::Idle => 0,
        Status::Working => 1,
        Status::NoNetwork(_) => 2,
        Status::Conditions => 3,
        Status::Full => 4,
        Status::Standby => 5,
    }
}

fn json_string(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 2);
    out.push('"');
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c if u32::from(c) < 0x20 => out.push(' '),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn json_tint(tint: Option<u32>) -> String {
    tint.map_or("null".to_string(), |value| value.to_string())
}

fn json_amounts(list: &[openfactor_sim::Amount]) -> String {
    let entries: Vec<String> = list
        .iter()
        .map(|entry| {
            let (scope, id) = match entry.scope {
                Scope::Local(item) => ("l", item.0),
                Scope::Global(item) => ("g", item.0),
                Scope::Network(item) => ("n", item.0),
                Scope::Map { layer, .. } => ("m", layer.0),
            };
            format!(r#"["{scope}",{id},{}]"#, entry.amount)
        })
        .collect();
    format!("[{}]", entries.join(","))
}

fn json_rule(rule: &Rule) -> String {
    format!(r#"{{"rate":{},"in":{},"out":{}}}"#, rule.rate, json_amounts(&rule.takes), json_amounts(&rule.gives))
}

fn port_code(port: Port) -> i32 {
    match port {
        Port::Front => 0,
        Port::Left => 1,
        Port::Right => 2,
    }
}

/// What the page needs to build its toolbar and draw the pack's content.
fn pack_json(pack: &Pack) -> String {
    let content = &pack.content;
    let belts: Vec<String> = content
        .belts()
        .iter()
        .map(|b| format!(r#"{{"name":{},"speed":{},"style":{}}}"#, json_string(&b.name), b.speed, b.style))
        .collect();
    let units: Vec<String> = content
        .units()
        .iter()
        .map(|unit| {
            let (route, range) = match unit.route {
                None => ("null", 0),
                Some(Route::Split) => (r#""split""#, 0),
                Some(Route::Filter) => (r#""filter""#, 0),
                Some(Route::Tunnel { range }) => (r#""tunnel""#, range),
            };
            let links: Vec<String> = unit.links.iter().map(|l| format!("[{},{}]", l.resource.0, l.reach)).collect();
            let rules: Vec<String> = unit.rules.iter().map(json_rule).collect();
            let mut ports: Vec<i32> = match unit.route {
                Some(Route::Tunnel { .. }) => vec![0],
                Some(_) => vec![0, 1, 2],
                None => unit.outputs.iter().map(|&(_, port)| port_code(port)).collect(),
            };
            ports.sort_unstable();
            ports.dedup();
            format!(
                r#"{{"id":{},"name":{},"sprite":{},"tint":{},"group":{},"sends":{},"accepts":{},"route":{},"range":{},"links":[{}],"rules":[{}],"ports":{:?}}}"#,
                json_string(&unit.id),
                json_string(&unit.name),
                unit.sprite,
                json_tint(unit.tint),
                json_string(&unit.group),
                unit.sends_items(),
                unit.accepts_items(),
                route,
                range,
                links.join(","),
                rules.join(","),
                ports
            )
        })
        .collect();
    let resources: Vec<String> = content
        .resources()
        .iter()
        .map(|r| format!(r#"{{"name":{},"sprite":{},"tint":{}}}"#, json_string(&r.name), r.sprite, json_tint(r.tint)))
        .collect();
    let layers: Vec<String> = content
        .layers()
        .iter()
        .map(|layer| {
            let haze = layer.overlay.map_or("null".to_string(), |o| o.color.to_string());
            format!(
                r#"{{"name":{},"sprites":{:?},"tint":{},"haze":{}}}"#,
                json_string(&layer.name),
                layer.sprites,
                json_tint(layer.tint),
                haze
            )
        })
        .collect();
    let metrics: Vec<String> =
        content.metrics().iter().map(|m| format!(r#"{{"name":{}}}"#, json_string(&m.name))).collect();
    format!(
        r#"{{"belts":[{}],"units":[{}],"resources":[{}],"layers":[{}],"metrics":[{}]}}"#,
        belts.join(","),
        units.join(","),
        resources.join(","),
        layers.join(","),
        metrics.join(",")
    )
}

thread_local! {
    static GAME: RefCell<Game> = RefCell::new(Game::new());
}

fn with_game<T>(f: impl FnOnce(&mut Game) -> T) -> T {
    GAME.with(|game| f(&mut game.borrow_mut()))
}

#[no_mangle]
pub extern "C" fn of_reset() {
    with_game(|game| *game = Game::new());
}

/// The pack's definitions as JSON text. Read `of_pack_json_len` bytes from here.
#[no_mangle]
pub extern "C" fn of_pack_json() -> *const u8 {
    with_game(|game| game.pack_json.as_ptr())
}

#[no_mangle]
pub extern "C" fn of_pack_json_len() -> u32 {
    with_game(|game| game.pack_json.len() as u32)
}

/// Queues a placement. `kind` numbers the belt tiers first, then the pack's units.
#[no_mangle]
pub extern "C" fn of_place(x: i32, y: i32, dir: i32, kind: i32) {
    let dir = DIRS[dir.rem_euclid(4) as usize];
    let pos = Pos::new(x, y);
    with_game(|game| {
        let building = game.building_from_code(kind);
        game.queued.push(Command::Place { pos, dir, building });
    });
}

/// Queues a setting: a rule index, a resource for a filter, or negative for the default.
#[no_mangle]
pub extern "C" fn of_configure(x: i32, y: i32, value: i32) {
    let setting = (value >= 0).then_some(value as u16);
    with_game(|game| game.queued.push(Command::Configure { pos: Pos::new(x, y), setting }));
}

/// Describes the building at a tile as JSON. Read `of_inspect_len` bytes from here.
#[no_mangle]
pub extern "C" fn of_inspect(x: i32, y: i32) -> *const u8 {
    with_game(|game| {
        game.inspection = game.inspection_json(Pos::new(x, y));
        game.inspection.as_ptr()
    })
}

#[no_mangle]
pub extern "C" fn of_inspect_len() -> u32 {
    with_game(|game| game.inspection.len() as u32)
}

#[no_mangle]
pub extern "C" fn of_remove(x: i32, y: i32) {
    with_game(|game| game.queued.push(Command::Remove { pos: Pos::new(x, y) }));
}

/// Applies queued edits and advances `ticks` ticks. Returns how many edits were rejected.
#[no_mangle]
pub extern "C" fn of_step(ticks: u32) -> u32 {
    with_game(|game| {
        let mut rejected = 0;
        if ticks > 0 {
            let commands = std::mem::take(&mut game.queued);
            rejected = game.world.step(&commands).len() as u32;
            for _ in 1..ticks {
                game.world.step(&[]);
            }
        }
        rejected
    })
}

/// Writes the view as four counted lists of i32 records: ground tiles (x, y, layer),
/// haze tiles (x, y, layer, strength 0..=255), buildings (x, y, dir, kind, status)
/// and belt items (x, y, dir, offset, item).
#[no_mangle]
pub extern "C" fn of_snapshot() -> *const i32 {
    with_game(|game| {
        game.write_snapshot();
        game.snapshot.as_ptr()
    })
}

#[no_mangle]
pub extern "C" fn of_snapshot_len() -> u32 {
    with_game(|game| game.snapshot.len() as u32)
}

#[no_mangle]
pub extern "C" fn of_tick() -> u32 {
    with_game(|game| game.world.tick() as u32)
}

#[no_mangle]
pub extern "C" fn of_produced(item: u32) -> u32 {
    with_game(|game| game.world.stats(ItemId(item as u16)).produced as u32)
}

#[no_mangle]
pub extern "C" fn of_consumed(item: u32) -> u32 {
    with_game(|game| game.world.stats(ItemId(item as u16)).consumed as u32)
}

#[no_mangle]
pub extern "C" fn of_metric(metric: u32) -> u32 {
    with_game(|game| game.world.metric(MetricId(metric as u16)).total as u32)
}

#[no_mangle]
pub extern "C" fn of_hash_hi() -> u32 {
    with_game(|game| (game.world.state_hash() >> 32) as u32)
}

#[no_mangle]
pub extern "C" fn of_hash_lo() -> u32 {
    with_game(|game| game.world.state_hash() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn resource(name: &str) -> u32 {
        with_game(|game| game.world.content().resource_id(name).expect("the base pack defines it").0 as u32)
    }

    #[test]
    fn base_pack_builds_and_runs() {
        of_reset();
        assert_eq!(of_step(7200), 0);
        assert!(of_produced(resource("gear")) > 0);
        assert!(of_produced(resource("wire")) > 0);
        assert!(of_metric(0) > 0);
        assert!(!of_snapshot().is_null() && of_snapshot_len() > 0);
        println!("starter hash {:08x}{:08x}", of_hash_hi(), of_hash_lo());
    }

    #[test]
    fn pack_json_lists_the_content() {
        let json = with_game(|game| game.pack_json.clone());
        assert!(json.contains(r#""id":"depot","name":"Depot","sprite":58"#));
        assert!(json.contains(r#""name":"Express belt","speed":32,"style":1"#));
        assert!(json.contains(r#""name":"iron ground","sprites":[3, 32]"#));
        assert!(json.contains(r#""route":"tunnel","range":5"#));
    }

    #[test]
    fn a_placed_building_can_be_inspected_and_configured() {
        of_reset();
        let kind = with_game(|game| {
            let unit = game.world.content().unit_id("workshop").expect("the base pack defines it");
            game.building_code(Building::Unit(unit))
        });
        of_place(100, 100, 1, kind);
        of_configure(100, 100, 2);
        assert_eq!(of_step(1), 0);
        of_inspect(100, 100);
        let json = with_game(|game| game.inspection.clone());
        assert!(json.contains(&format!(r#""kind":{kind}"#)), "{json}");
        assert!(json.contains(r#""setting":2"#), "{json}");
        of_inspect(-100, -100);
        assert_eq!(with_game(|game| game.inspection.clone()), "null");
    }
}
