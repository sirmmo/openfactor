//! Browser sandbox bindings: a plain numeric ABI, so no JS glue generator is needed.
//!
//! Edits are queued and applied on the next `of_step`. A returned pointer is
//! only valid until the next call, because wasm memory can move when it grows.

use openfactor_sim::{
    BeltId, Building, Command, Dir, ItemId, LoadError, MetricId, Pack, Port, Pos, Reject, Route, Rule, Scope, Status,
    TechId, Test, UnitId, World,
};
use std::cell::RefCell;

const BASE_PACK: &str = include_str!("../../../packs/base.gbx");
const DIRS: [Dir; 4] = [Dir::North, Dir::East, Dir::South, Dir::West];

struct Game {
    pack: Pack,
    world: World,
    queued: Vec<Command>,
    snapshot: Vec<i32>,
    pack_json: String,
    inspection: String,
    /// Why the latest refused edit was refused, as a `reject_code`.
    last_reject: i32,
    /// The latest `of_save`.
    saved: Vec<u8>,
    /// Where the page writes a save for `of_load`.
    incoming: Vec<u8>,
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
            last_reject: 0,
            saved: Vec::new(),
            incoming: Vec::new(),
            pack,
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
            out.extend([pos.x, pos.y, pos.z, dir_code(dir), self.building_code(building), status]);
        }
        finish_count(&mut out, mark, 6);

        let mark = reserve_count(&mut out);
        for (pos, dir, offset, item) in self.world.belt_items() {
            out.extend([pos.x, pos.y, pos.z, dir_code(dir), offset as i32, i32::from(item.0)]);
        }
        finish_count(&mut out, mark, 6);
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
        let level = |value: Option<i32>| value.map_or("null".to_string(), |z| z.to_string());
        format!(
            r#"{{"kind":{},"dir":{},"status":{},"missing":{},"setting":{},"job":{},"held":{},"networks":{},"sendsTo":{},"receivesFrom":{}}}"#,
            self.building_code(view.building),
            dir_code(view.dir),
            status_code(view.status),
            missing,
            setting,
            job,
            pairs(held),
            pairs(networks),
            level(view.sends_to),
            level(view.receives_from)
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

fn reject_code(reason: Reject) -> i32 {
    match reason {
        Reject::Occupied => 1,
        Reject::NoResource => 2,
        Reject::Locked => 3,
        Reject::CantAfford => 4,
        Reject::ResearchBusy => 5,
        Reject::MissingRequirement => 6,
        Reject::NoSuchLevel => 8,
        Reject::Unsupported => 9,
        Reject::CarriesLoad => 10,
        _ => 7,
    }
}

fn status_code(status: Status) -> i32 {
    match status {
        Status::Idle => 0,
        Status::Working => 1,
        Status::NoNetwork(_) => 2,
        Status::Conditions => 3,
        Status::Full => 4,
        Status::Standby => 5,
        Status::Unlinked => 6,
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

fn json_cost(cost: &[(ItemId, u32)]) -> String {
    let pairs: Vec<String> = cost.iter().map(|(item, amount)| format!("[{},{}]", item.0, amount)).collect();
    format!("[{}]", pairs.join(","))
}

fn json_lock(locked_by: Option<TechId>) -> String {
    locked_by.map_or("null".to_string(), |tech| tech.0.to_string())
}

fn json_name(name: &Option<String>) -> String {
    name.as_deref().map_or("null".to_string(), json_string)
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
                Scope::Map { layer, radius } => {
                    return format!(r#"["m",{},{},{radius}]"#, layer.0, entry.amount);
                }
            };
            format!(r#"["{scope}",{id},{}]"#, entry.amount)
        })
        .collect();
    format!("[{}]", entries.join(","))
}

fn json_rule(rule: &Rule) -> String {
    let conditions: Vec<String> = rule
        .conditions
        .iter()
        .map(|c| {
            let test = match c.test {
                Test::AtLeast => "atLeast",
                Test::AtMost => "atMost",
                Test::Greater => "greater",
                Test::Less => "less",
                Test::Is => "is",
            };
            format!(r#"[{},"{test}",{}]"#, c.layer.0, c.value)
        })
        .collect();
    let sides: Vec<String> = rule.sides.iter().map(|&(item, port)| format!("[{},{}]", item.0, port_code(port))).collect();
    let logs: Vec<String> = rule.logs.iter().map(|(metric, amount)| format!("[{},{amount}]", metric.0)).collect();
    format!(
        r#"{{"name":{},"rate":{},"in":{},"out":{},"lock":{},"when":[{}],"sides":[{}],"logs":[{}]}}"#,
        json_string(&rule.name),
        rule.rate,
        json_amounts(&rule.takes),
        json_amounts(&rule.gives),
        json_lock(rule.locked_by),
        conditions.join(","),
        sides.join(","),
        logs.join(",")
    )
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
        .map(|b| {
            format!(
                r#"{{"id":{},"name":{},"about":{},"speed":{},"style":{},"cost":{},"lock":{},"model":{},"corner":{}}}"#,
                json_string(&b.id),
                json_string(&b.name),
                json_string(&b.description),
                b.speed,
                b.style,
                json_cost(&b.cost),
                json_lock(b.locked_by),
                json_name(&b.model),
                json_name(&b.corner_model)
            )
        })
        .collect();
    let units: Vec<String> = content
        .units()
        .iter()
        .map(|unit| {
            let (route, range, down) = match unit.route {
                None => ("null", 0, false),
                Some(Route::Split) => (r#""split""#, 0, false),
                Some(Route::Filter) => (r#""filter""#, 0, false),
                Some(Route::Tunnel { range }) => (r#""tunnel""#, range, false),
                Some(Route::Lift { range, down }) => (r#""lift""#, range, down),
            };
            let links: Vec<String> = unit.links.iter().map(|l| format!("[{},{}]", l.resource.0, l.reach)).collect();
            let rules: Vec<String> = unit.rules.iter().map(json_rule).collect();
            let mut ports: Vec<i32> = match unit.route {
                Some(Route::Tunnel { .. } | Route::Lift { .. }) => vec![0],
                Some(_) => vec![0, 1, 2],
                None => unit.outputs.iter().map(|&(_, port)| port_code(port)).collect(),
            };
            ports.sort_unstable();
            ports.dedup();
            format!(
                r#"{{"id":{},"name":{},"about":{},"sprite":{},"tint":{},"group":{},"sends":{},"accepts":{},"route":{},"range":{},"down":{},"links":[{}],"rules":[{}],"ports":{:?},"cost":{},"lock":{},"delivers":{},"storesAny":{},"placeOn":{:?},"model":{}}}"#,
                json_string(&unit.id),
                json_string(&unit.name),
                json_string(&unit.description),
                unit.sprite,
                json_tint(unit.tint),
                json_string(&unit.group),
                unit.sends_items(),
                unit.accepts_items(),
                route,
                range,
                down,
                links.join(","),
                rules.join(","),
                ports,
                json_cost(&unit.cost),
                json_lock(unit.locked_by),
                unit.delivers,
                unit.stores_any,
                unit.place_on.iter().map(|layer| layer.0).collect::<Vec<u16>>(),
                json_name(&unit.model)
            )
        })
        .collect();
    let resources: Vec<String> = content
        .resources()
        .iter()
        .map(|r| {
            format!(
                r#"{{"id":{},"name":{},"about":{},"sprite":{},"tint":{}}}"#,
                json_string(&r.id),
                json_string(&r.name),
                json_string(&r.description),
                r.sprite,
                json_tint(r.tint)
            )
        })
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
    let techs: Vec<String> = content
        .techs()
        .iter()
        .map(|tech| {
            let requires: Vec<u16> = tech.requires.iter().map(|required| required.0).collect();
            format!(
                r#"{{"id":{},"name":{},"about":{},"requires":{:?},"cost":{},"time":{}}}"#,
                json_string(&tech.id),
                json_string(&tech.name),
                json_string(&tech.description),
                requires,
                json_cost(&tech.cost),
                tech.time
            )
        })
        .collect();
    format!(
        r#"{{"levels":{},"span":{},"belts":[{}],"units":[{}],"resources":[{}],"layers":[{}],"metrics":[{}],"techs":[{}]}}"#,
        content.levels(),
        content.span(),
        belts.join(","),
        units.join(","),
        resources.join(","),
        layers.join(","),
        metrics.join(","),
        techs.join(",")
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
pub extern "C" fn of_place(x: i32, y: i32, z: i32, dir: i32, kind: i32) {
    let dir = DIRS[dir.rem_euclid(4) as usize];
    let pos = Pos::at(x, y, z);
    with_game(|game| {
        let building = game.building_from_code(kind);
        game.queued.push(Command::Place { pos, dir, building });
    });
}

/// Queues a setting: a rule index, a resource for a filter, or negative for the default.
#[no_mangle]
pub extern "C" fn of_configure(x: i32, y: i32, z: i32, value: i32) {
    let setting = (value >= 0).then_some(value as u16);
    with_game(|game| game.queued.push(Command::Configure { pos: Pos::at(x, y, z), setting }));
}

/// Describes the building at a tile as JSON. Read `of_inspect_len` bytes from here.
#[no_mangle]
pub extern "C" fn of_inspect(x: i32, y: i32, z: i32) -> *const u8 {
    with_game(|game| {
        game.inspection = game.inspection_json(Pos::at(x, y, z));
        game.inspection.as_ptr()
    })
}

#[no_mangle]
pub extern "C" fn of_inspect_len() -> u32 {
    with_game(|game| game.inspection.len() as u32)
}

#[no_mangle]
pub extern "C" fn of_remove(x: i32, y: i32, z: i32) {
    with_game(|game| game.queued.push(Command::Remove { pos: Pos::at(x, y, z) }));
}

/// Applies queued edits and advances `ticks` ticks. Returns how many edits were rejected.
#[no_mangle]
pub extern "C" fn of_step(ticks: u32) -> u32 {
    with_game(|game| {
        let mut rejected = 0;
        if ticks > 0 {
            let commands = std::mem::take(&mut game.queued);
            let refused = game.world.step(&commands);
            rejected = refused.len() as u32;
            if let Some(last) = refused.last() {
                game.last_reject = reject_code(last.reason);
            }
            for _ in 1..ticks {
                game.world.step(&[]);
            }
        }
        rejected
    })
}

/// Writes the view as four counted lists of i32 records: ground tiles (x, y, layer),
/// haze tiles (x, y, layer, strength 0..=255), buildings (x, y, z, dir, kind, status)
/// and belt items (x, y, z, dir, offset, item).
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

/// Why the latest refused edit was refused. 0 if nothing has been.
#[no_mangle]
pub extern "C" fn of_last_reject() -> i32 {
    with_game(|game| game.last_reject)
}

#[no_mangle]
pub extern "C" fn of_stock(item: u32) -> u32 {
    with_game(|game| game.world.global(ItemId(item as u16)) as u32)
}

/// Queues the start of a research.
#[no_mangle]
pub extern "C" fn of_research(tech: u32) {
    with_game(|game| game.queued.push(Command::Research { tech: TechId(tech as u16) }));
}

/// Whether a research has finished: 1 if so, otherwise 0.
#[no_mangle]
pub extern "C" fn of_researched(tech: u32) -> u32 {
    with_game(|game| u32::from(game.world.researched(TechId(tech as u16))))
}

/// The running research, or -1.
#[no_mangle]
pub extern "C" fn of_research_active() -> i32 {
    with_game(|game| game.world.active_research().map_or(-1, |(tech, _)| i32::from(tech.0)))
}

/// Ticks left on the running research.
#[no_mangle]
pub extern "C" fn of_research_left() -> u32 {
    with_game(|game| game.world.active_research().map_or(0, |(_, left)| left))
}

#[no_mangle]
pub extern "C" fn of_consumed(item: u32) -> u32 {
    with_game(|game| game.world.stats(ItemId(item as u16)).consumed as u32)
}

#[no_mangle]
pub extern "C" fn of_metric(metric: u32) -> u32 {
    with_game(|game| game.world.metric(MetricId(metric as u16)).total as u32)
}

/// Saves the world. Read `of_save_len` bytes from here. Edits queued since
/// the last `of_step` aren't included, so save between steps.
#[no_mangle]
pub extern "C" fn of_save() -> *const u8 {
    with_game(|game| {
        game.saved = game.world.save();
        game.saved.as_ptr()
    })
}

#[no_mangle]
pub extern "C" fn of_save_len() -> u32 {
    with_game(|game| game.saved.len() as u32)
}

/// Room for a save of `len` bytes, for the page to copy one into before `of_load`.
#[no_mangle]
pub extern "C" fn of_load_buffer(len: u32) -> *mut u8 {
    with_game(|game| {
        game.incoming = vec![0; len as usize];
        game.incoming.as_mut_ptr()
    })
}

/// Replaces the world with the save in the load buffer. Returns 0 if it
/// loaded, 1 if the save is damaged, 2 if it comes from a newer version and
/// 3 if it was made with other rules. On failure the world is unchanged.
#[no_mangle]
pub extern "C" fn of_load() -> i32 {
    with_game(|game| {
        let bytes = std::mem::take(&mut game.incoming);
        match World::load(&game.pack, &bytes) {
            Ok(world) => {
                game.world = world;
                game.queued.clear();
                game.last_reject = 0;
                0
            }
            Err(LoadError::Damaged(_)) => 1,
            Err(LoadError::Version(_)) => 2,
            Err(LoadError::OtherPack) => 3,
        }
    })
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

    fn load_bytes(bytes: &[u8]) -> i32 {
        let pointer = of_load_buffer(bytes.len() as u32);
        // SAFETY: the buffer was just allocated with exactly this length.
        unsafe { std::ptr::copy_nonoverlapping(bytes.as_ptr(), pointer, bytes.len()) };
        of_load()
    }

    fn saved_bytes() -> Vec<u8> {
        let pointer = of_save();
        // SAFETY: of_save keeps the bytes alive until the next call.
        unsafe { std::slice::from_raw_parts(pointer, of_save_len() as usize) }.to_vec()
    }

    #[test]
    fn a_save_loads_back_into_the_same_world() {
        of_reset();
        of_step(900);
        let hash = (of_hash_hi(), of_hash_lo());
        let bytes = saved_bytes();
        of_step(500);
        assert_ne!((of_hash_hi(), of_hash_lo()), hash);
        assert_eq!(load_bytes(&bytes), 0);
        assert_eq!((of_hash_hi(), of_hash_lo(), of_tick()), (hash.0, hash.1, 900));
        assert_eq!(load_bytes(&bytes[..bytes.len() / 2]), 1);
        assert_eq!(load_bytes(b"nonsense"), 1);
        assert_eq!(of_tick(), 900, "a failed load leaves the world alone");
    }

    #[test]
    fn base_pack_builds_and_runs() {
        of_reset();
        assert_eq!(of_step(7200), 0);
        assert!(of_produced(resource("ironPlate")) > 100);
        assert!(of_stock(resource("ironPlate")) > 100);
        assert!(of_metric(0) > 0);
        assert!(!of_snapshot().is_null() && of_snapshot_len() > 0);
        println!("starter hash {:08x}{:08x}", of_hash_hi(), of_hash_lo());
    }

    #[test]
    fn pack_json_lists_the_content() {
        let json = with_game(|game| game.pack_json.clone());
        assert!(json.contains(r#""id":"depot","name":"Depot","about":"Accepts anything."#));
        assert!(json.contains(r#""id":"express","name":"Express belt","about":"A belt that carries"#));
        assert!(json.contains(r#""name":"Gear","about":"The basic moving part.","sprite":114"#));
        assert!(json.contains(r#""placeOn":[0, 1, 2, 3]"#), "{json}");
        assert!(json.contains(r#""name":"iron ground","sprites":[3, 32]"#));
        assert!(json.contains(r#""route":"tunnel","range":5"#));
        assert!(json.contains(r#""name":"Electricity","about":"Steam power, pumps, pipes and poles: the start of powered machines.","requires":[1, 2]"#), "{json}");
    }

    #[test]
    fn a_placed_building_can_be_inspected_and_configured() {
        of_reset();
        let kind = with_game(|game| {
            let unit = game.world.content().unit_id("workshop").expect("the base pack defines it");
            game.building_code(Building::Unit(unit))
        });
        of_place(100, 100, 0, 1, kind);
        of_configure(100, 100, 0, 0);
        assert_eq!(of_step(1), 0);
        of_inspect(100, 100, 0);
        let json = with_game(|game| game.inspection.clone());
        assert!(json.contains(&format!(r#""kind":{kind}"#)), "{json}");
        assert!(json.contains(r#""setting":0"#), "{json}");
        of_inspect(-100, -100, 0);
        assert_eq!(with_game(|game| game.inspection.clone()), "null");

        of_configure(100, 100, 0, 2);
        assert_eq!(of_step(1), 1);
        assert_eq!(of_last_reject(), 3, "the circuit recipe needs research");
    }

    #[test]
    fn levels_are_part_of_the_bindings() {
        of_reset();
        let (belt, pillar) = with_game(|game| {
            let pillar = game.world.content().unit_id("pillar").expect("the base pack defines it");
            (0, game.building_code(Building::Unit(pillar)))
        });
        // Too far from anything on the ground to stand up.
        of_place(100, 100, 1, 1, belt);
        assert_eq!(of_step(1), 1);
        assert_eq!(of_last_reject(), 9);
        with_game(|game| {
            let elevation = game.world.content().tech_id("elevation").expect("the base pack defines it");
            assert!(game.pack_json.contains(r#""levels":4,"span":3"#));
            assert!(game.pack_json.contains(r#""route":"lift","range":3,"down":true"#), "{}", game.pack_json);
            assert_eq!(game.world.researched(elevation), false);
        });
        of_place(100, 100, 0, 1, pillar);
        assert_eq!(of_step(1), 1);
        assert_eq!(of_last_reject(), 3, "the pillar needs research");
    }

    /// The browser check replays exactly these calls and compares the printed hash.
    #[test]
    fn the_starter_line_can_be_carried_over_a_bridge() {
        of_reset();
        of_step(1);
        let (routing, elevation, hoist, chute) = with_game(|game| {
            let content = game.world.content();
            let tech = |id: &str| u32::from(content.tech_id(id).expect("the base pack defines it").0);
            let code = |id: &str| game.building_code(Building::Unit(content.unit_id(id).expect("the base pack defines it")));
            (tech("routing"), tech("elevation"), code("hoist"), code("chute"))
        });
        for tech in [routing, elevation] {
            // Give the line time to deliver what the research costs.
            of_step(1800);
            of_research(tech);
            assert_eq!(of_step(1), 0);
            of_step(of_research_left());
            assert_eq!(of_researched(tech), 1);
        }
        of_remove(12, 2, 0);
        of_remove(17, 2, 0);
        assert_eq!(of_step(1), 0);
        of_place(12, 2, 0, 1, hoist);
        of_place(12, 2, 1, 1, hoist);
        for x in 13..=16 {
            of_place(x, 2, 1, 1, 0);
        }
        of_place(17, 2, 1, 1, chute);
        of_place(17, 2, 0, 1, chute);
        assert_eq!(of_step(1), 0);
        let before = of_stock(resource("ironPlate"));
        of_step(3600);
        assert!(of_stock(resource("ironPlate")) > before + 50, "plates reach the depot over the bridge");
        println!("bridge hash {:08x}{:08x}", of_hash_hi(), of_hash_lo());
    }

    #[test]
    fn research_runs_through_the_bindings() {
        of_reset();
        of_step(1);
        assert_eq!(of_research_active(), -1);
        of_research(0);
        assert_eq!(of_step(1), 0);
        assert_eq!(of_research_active(), 0);
        assert!(of_research_left() > 0);
        of_step(of_research_left());
        assert_eq!(of_researched(0), 1);
    }
}
