use openfactor_sim::*;

const ORE: ItemId = ItemId(0);
const PLATE: ItemId = ItemId(1);
const MINER: UnitId = UnitId(0);
const SMELTER: UnitId = UnitId(1);
const ASSEMBLER: UnitId = UnitId(2);
const DEPOT: UnitId = UnitId(3);

fn p(x: i32, y: i32) -> Pos {
    Pos::new(x, y)
}

fn place(x: i32, y: i32, dir: Dir, building: Building) -> Command {
    Command::Place { pos: p(x, y), dir, building }
}

fn unit(x: i32, y: i32, id: UnitId) -> Command {
    place(x, y, Dir::East, Building::Unit(id))
}

fn belts(xs: std::ops::RangeInclusive<i32>, y: i32) -> Vec<Command> {
    xs.map(|x| place(x, y, Dir::East, Building::Belt(BeltId(0)))).collect()
}

fn pack(mine: u32, smelt: u32, assemble: u32, extra: &str) -> Pack {
    Pack::parse(&pack_text(mine, smelt, assemble, extra)).unwrap()
}

fn pack_text(mine: u32, smelt: u32, assemble: u32, extra: &str) -> String {
    format!(
        r#"
        resource ore
        resource plate
        resource widget
        layer oreField
        unit miner placeOn oreField
        unit smelter
        unit assembler
        unit depot stores any
        metric made
        unitRule mine
            appliesTo miner
            rate {mine}
            map oreField in 1
            local ore out 1
        end
        unitRule smelt
            appliesTo smelter
            rate {smelt}
            local ore in 1
            local plate out 1
        end
        unitRule assemble
            appliesTo assembler
            rate {assemble}
            local plate in 2
            local widget out 1
            log made 1
        end
        {extra}
        "#
    )
}

const ONE_ORE: &str = "patch oreField at 0 0 size 1 1 amount 1000000";

fn ore_world() -> World {
    World::new(&pack(30, 30, 60, ONE_ORE))
}

fn fast_miner_world() -> World {
    World::new(&pack(1, 30, 60, ONE_ORE))
}

fn run(world: &mut World, commands: &[Command], ticks: u32) {
    assert_eq!(world.step(commands), vec![]);
    for _ in 1..ticks {
        world.step(&[]);
    }
}

/// Every item ever created must be in the world, consumed or recorded destroyed.
fn assert_conserved(world: &World) {
    let totals = world.item_totals();
    for index in 0..world.content().resources().len() {
        let item = ItemId(index as u16);
        let stats = world.stats(item);
        let held = totals.get(&item).copied().unwrap_or(0);
        assert_eq!(
            stats.produced,
            held + stats.consumed + stats.destroyed + stats.spent - stats.granted,
            "{item:?} at tick {}",
            world.tick()
        );
    }
}

#[test]
fn miner_belt_assembler_chain_delivers_plates() {
    let mut world = ore_world();
    let mut commands = vec![unit(0, 0, MINER)];
    commands.extend(belts(1..=5, 0));
    commands.push(unit(6, 0, SMELTER));
    commands.extend(belts(7..=10, 0));
    commands.push(unit(11, 0, DEPOT));

    run(&mut world, &commands, 6000);

    // Each stage takes about 30 ticks, so expect close to one plate per ~31
    // ticks, less the time the first item spends in transit.
    let delivered = world.stock(p(11, 0), PLATE);
    assert!((180..=200).contains(&delivered), "delivered {delivered}");
    assert_conserved(&world);
}

#[test]
fn belt_fills_to_capacity_then_drains_at_full_throughput() {
    let mut world = fast_miner_world();
    let mut commands = vec![unit(0, 0, MINER)];
    commands.extend(belts(1..=8, 0));
    run(&mut world, &commands, 2000);

    // 8 tiles of 256 at spacing 64, counting both end positions.
    assert_eq!(world.belt_item_count(), 33);
    assert_conserved(&world);

    run(&mut world, &[unit(9, 0, DEPOT)], 400);
    let first = world.stock(p(9, 0), ORE);
    for _ in 0..400 {
        world.step(&[]);
    }
    assert_eq!(world.stock(p(9, 0), ORE) - first, 100, "one item per 4 ticks");
}

#[test]
fn junction_alternates_between_saturated_feeders() {
    let extra = r#"
        resource ore2
        layer field2
        unit miner2 placeOn field2
        unitRule mine2
            appliesTo miner2
            map field2 in 1
            local ore2 out 1
        end
        patch oreField at 0 0 size 1 1 amount 1000000
        patch field2 at 0 2 size 1 1 amount 1000000
    "#;
    let pack = pack(1, 30, 60, extra);
    let other = pack.content.resource_id("ore2").unwrap();
    let miner2 = pack.content.unit_id("miner2").unwrap();
    let mut world = World::new(&pack);

    let mut commands = vec![
        unit(0, 0, MINER),
        unit(0, 2, miner2),
        place(4, 0, Dir::South, Building::Belt(BeltId(0))),
        place(4, 2, Dir::North, Building::Belt(BeltId(0))),
        unit(9, 1, DEPOT),
    ];
    commands.extend(belts(1..=3, 0));
    commands.extend(belts(1..=3, 2));
    commands.extend(belts(4..=8, 1));
    run(&mut world, &commands, 4000);

    let a = world.stock(p(9, 1), ORE);
    let b = world.stock(p(9, 1), other);
    assert!(a > 400, "a={a}");
    assert!(a.abs_diff(b) <= 2, "a={a} b={b}");
    assert_conserved(&world);
}

#[test]
fn closed_ring_circulates_without_losing_items() {
    let mut world = ore_world();
    let mut commands = vec![
        place(1, 0, Dir::East, Building::Belt(BeltId(0))),
        place(2, 0, Dir::South, Building::Belt(BeltId(0))),
        place(2, 1, Dir::West, Building::Belt(BeltId(0))),
        place(1, 1, Dir::North, Building::Belt(BeltId(0))),
    ];
    commands.push(unit(0, 0, MINER));
    run(&mut world, &commands, 200);

    let count = world.belt_item_count();
    assert!(count > 0);
    assert_eq!(world.line_count(), 1);
    run(&mut world, &[Command::Remove { pos: p(0, 0) }], 2000);
    assert_eq!(world.belt_item_count(), count);
    assert_conserved(&world);
}

#[test]
fn editing_a_belt_keeps_items_on_surviving_tiles() {
    let mut world = fast_miner_world();
    let mut commands = vec![unit(0, 0, MINER)];
    commands.extend(belts(1..=8, 0));
    run(&mut world, &commands, 2000);
    assert_eq!(world.line_count(), 1);

    run(&mut world, &[Command::Remove { pos: p(5, 0) }], 1);
    assert_eq!(world.line_count(), 2);
    assert!(world.stats(ORE).destroyed >= 4, "the removed tile's items are gone");
    assert_conserved(&world);

    run(&mut world, &[place(5, 0, Dir::East, Building::Belt(BeltId(0)))], 1000);
    assert_eq!(world.line_count(), 1);
    assert_conserved(&world);
}

#[test]
fn invalid_commands_are_rejected_without_effect() {
    let mut world = ore_world();
    let before = world.state_hash();
    let commands = [
        unit(5, 5, MINER),
        unit(5, 5, UnitId(99)),
        Command::Remove { pos: p(5, 5) },
    ];
    let reasons: Vec<Reject> = world.step(&commands).iter().map(|r| r.reason).collect();
    assert_eq!(reasons, [Reject::NoResource, Reject::UnknownUnit, Reject::NothingToRemove]);
    assert_ne!(before, world.state_hash(), "the tick still advances");
    assert_eq!(world.line_count(), 0);
}

#[test]
fn render_views_match_the_world() {
    let mut world = ore_world();
    let mut commands = vec![unit(0, 0, MINER)];
    commands.extend(belts(1..=3, 0));
    run(&mut world, &commands, 200);

    assert_eq!(world.buildings().count(), 4);
    assert_eq!(world.layer_tiles().count(), 1);
    assert_eq!(world.belt_items().count(), world.belt_item_count());
    for (pos, dir, offset, item) in world.belt_items() {
        assert!((1..=3).contains(&pos.x) && pos.y == 0, "{pos:?}");
        assert_eq!((dir, item), (Dir::East, ORE));
        assert!(offset <= TILE);
    }
}

// Packs: behaviour comes from data.

#[test]
fn a_new_unit_needs_only_rules() {
    let extra = r#"
        resource ingot
        unit press
        unitRule pressIngots
            appliesTo press
            rate 2
            ore in 1
            ingot out 3
        end
        patch oreField at 0 0 size 1 1 amount 10
    "#;
    let pack = pack(1, 30, 60, extra);
    let press = pack.content.unit_id("press").unwrap();
    let ingot = pack.content.resource_id("ingot").unwrap();
    let mut world = World::new(&pack);

    let mut commands = vec![unit(0, 0, MINER), unit(2, 0, press), unit(4, 0, DEPOT)];
    commands.extend(belts(1..=1, 0));
    commands.extend(belts(3..=3, 0));
    run(&mut world, &commands, 2000);

    assert_eq!(world.stock(p(4, 0), ingot), 30, "10 ore tiles' worth, three ingots each");
    assert_eq!(world.layer_tiles().count(), 0, "the field is mined out");
    assert_conserved(&world);
}

#[test]
fn rules_read_and_write_globals_and_map_layers() {
    let extra = r#"
        resource coins
        layer smog
        unit mint
        unit bank
        unitRule strike
            appliesTo mint
            rate 2
            global coins out 5
            map smog out 1 radius 1
            map smog atMost 2
        end
        unitRule spend
            appliesTo bank
            rate 1
            global coins in 1
        end
    "#;
    let pack = pack(30, 30, 60, extra);
    let content = &pack.content;
    let (mint, bank) = (content.unit_id("mint").unwrap(), content.unit_id("bank").unwrap());
    let (coins, smog) = (content.resource_id("coins").unwrap(), content.layer_id("smog").unwrap());
    let mut world = World::new(&pack);

    run(&mut world, &[unit(0, 0, mint)], 100);
    // The `atMost 2` condition stops it once the third strike has landed.
    assert_eq!(world.global(coins), 15);
    assert_eq!(world.layer_value(smog, p(0, 0)), 3);
    assert_eq!(world.layer_value(smog, p(1, 0)), 3);
    assert_eq!(world.layer_value(smog, p(1, 1)), 0, "radius is a diamond");

    run(&mut world, &[unit(5, 5, bank)], 100);
    assert_eq!(world.global(coins), 0, "another unit spends the shared stock");
}

#[test]
fn map_rules_apply_per_tile_on_their_schedule() {
    let text = r#"
        layer soil
        layer water
        layer grass
        mapRule grow
            rate 10
            map soil atLeast 5
            map water in 2
            map grass out 3
        end
        patch soil at 0 0 size 1 1 amount 5
        patch soil at 1 0 size 1 1 amount 4
        patch soil at 2 0 size 1 1 amount 5
        patch water at 0 0 size 3 1 amount 20
        patch water at 2 0 size 1 1 amount 3
    "#;
    let pack = Pack::parse(text).unwrap();
    let content = &pack.content;
    let (water, grass) = (content.layer_id("water").unwrap(), content.layer_id("grass").unwrap());
    let mut world = World::new(&pack);

    run(&mut world, &[], 9);
    assert_eq!(world.layer_value(grass, p(0, 0)), 0, "not due until ten ticks have passed");
    run(&mut world, &[], 1);
    assert_eq!(world.layer_value(grass, p(0, 0)), 3);

    run(&mut world, &[], 90);
    assert_eq!((world.layer_value(water, p(0, 0)), world.layer_value(grass, p(0, 0))), (0, 30));
    assert_eq!(
        (world.layer_value(water, p(1, 0)), world.layer_value(grass, p(1, 0))),
        (20, 0),
        "too little soil"
    );
    assert_eq!(
        (world.layer_value(water, p(2, 0)), world.layer_value(grass, p(2, 0))),
        (1, 3),
        "stops when the input runs short"
    );
}

#[test]
fn a_map_rule_can_clear_what_units_emit() {
    let text = r#"
        layer smog
        unit stack
        unitRule puff
            appliesTo stack
            rate 1
            map smog out 2
            map smog atMost 0
        end
        mapRule clear
            rate 3
            map smog in 1
        end
    "#;
    let pack = Pack::parse(text).unwrap();
    let smog = pack.content.layer_id("smog").unwrap();
    let mut world = World::new(&pack);
    run(&mut world, &[unit(0, 0, UnitId(0))], 1000);
    let value = world.layer_value(smog, p(0, 0));
    assert!(value <= 2, "the rule holds it down, found {value}");
    assert!(world.tick() == 1000);
}

#[test]
fn log_totals_a_metric_each_time_a_rule_finishes() {
    let text = r#"
        metric heat name "Heat"
        unit furnace
        unitRule burn
            appliesTo furnace
            rate 3
            log -timestamp=Time heat 2
        end
    "#;
    let pack = Pack::parse(text).unwrap();
    let heat = pack.content.metric_id("heat").unwrap();
    assert_eq!(pack.content.metrics()[0].name, "Heat");
    let mut world = World::new(&pack);
    run(&mut world, &[unit(0, 0, UnitId(0))], 40);
    assert_eq!(world.metric(heat), MetricValue { total: 20, last_tick: 39 });
}

#[test]
fn map_rules_and_metrics_are_part_of_the_state_hash() {
    let hash = |text: &str| {
        let pack = Pack::parse(text).unwrap();
        let mut world = World::new(&pack);
        run(&mut world, &[], 5);
        world.state_hash()
    };
    let base = "layer a\npatch a at 0 0 size 1 1 amount 9\n";
    let plain = hash(base);
    let with_rule = hash(&format!("{base}mapRule r\nrate 100\nmap a in 1\nend"));
    let with_metric = hash(&format!("{base}metric m\n"));
    assert_ne!(plain, with_rule, "rule definitions are hashed even before they fire");
    assert_ne!(plain, with_metric);

    let changes_state = hash(&format!("{base}mapRule r\nrate 2\nmap a in 1\nend"));
    assert_ne!(with_rule, changes_state);
}

#[test]
fn pack_mistakes_name_the_line() {
    let cases: [(&str, usize, &str); 40] = [
        ("resource a\nunit u\ncost u", 3, "pairs of a resource"),
        ("resource a\nunit u\ncost u a 1 a 2", 3, "listed twice"),
        ("resource a\ncost nothing a 1", 2, "unknown unit"),
        ("stock missing 1", 1, "unknown resource"),
        ("research r\nrequires nope\nend", 2, "unknown research"),
        ("research r\nunlocks nothing\nend", 2, "can't be unlocked"),
        ("unit u\nresearch r\nunlocks u\nend\nresearch s\nunlocks u\nend", 6, "more than one research"),
        ("research r\nrequires s\nend\nresearch s\nrequires r\nend", 1, "form a loop"),
        ("resource a\nunit u routes split\nunitRule u\na out 1\nend", 3, "can't have rules"),
        ("unit u routes teleport", 1, "split, filter, tunnel, up or down"),
        ("unit u range 3", 1, "after 'routes tunnel'"),
        ("unit u reach 3", 1, "after 'links"),
        ("resource a\nunit u links a reach 99", 2, "at most 32"),
        ("resource a\nunit u\nunitRule u\na out 1 up\nend", 4, "front, left or right"),
        ("resource a\nunit u\nunitRule r\nappliesTo u\na out 1\nend\nunitRule s\nappliesTo u\na out 1 left\nend", 7, "two different sides"),
        ("belt express\nbelt express", 2, "declared twice"),
        ("belt express\nunit express", 2, "already the name of a belt"),
        ("layer a\nlayer b\nmapRule r\nmap b out 1\nend", 3, "endless empty tiles"),
        ("layer a\nmapRule r\nmap a atMost 3\nmap a out 1\nend", 2, "endless empty tiles"),
        ("layer a\nmapRule r\nmap a in 1\nmap a out 1 radius 2\nend", 4, "'radius' isn't allowed"),
        ("resource x\nlayer a\nmapRule r\nmap a in 1\nglobal x out 1\nend", 5, "only use 'rate' and 'map'"),
        ("resource a\nunit u\nunitRule u\nlocal a in 1\nlocal a in 2\nend", 5, "already takes"),
        ("unit u\nunitRule u\nlog -timestamp=Time missing 1\nend", 3, "unknown metric 'missing'"),
        ("resource a\nunitRule r\nappliesTo nobody\na out 1\nend", 2, "unknown unit 'nobody'"),
        ("unit u\nunitRule r\nappliesTo u\nfrobnicate 1\nend", 4, "this rule statement"),
        ("unit u\nunitRule r\nappliesTo u\nlocal ores out 1\nend", 4, "unknown resource 'ores'"),
        ("unit u\nunitRule u\nrate 1\nend", 2, "doesn't take or give"),
        ("resource a\nunit u\nunitRule u\na out 1.5\nend", 4, "whole number"),
        ("unit u\nzoneRule grow\nrate 1\nend", 2, "doesn't support yet"),
        ("resource a\nunit u\nunitRule u\na out 1", 3, "never closed"),
        ("unit belt", 1, "built into the engine"),
        ("levels 99", 1, "at most 8"),
        ("span 99", 1, "at most 16"),
        ("levels 0", 1, "at least 1"),
        ("unit u\nplace u 0 0 east height 2", 2, "does not understand 'height'"),
        ("describe nothing \"x\"", 1, "isn't a belt, unit, research or resource"),
        ("unit u\ndescribe u", 2, "expected 'describe"),
        ("unit u\ndescribe u \"  \"", 2, "is empty"),
        ("unit u\ndescribe u \"a\"\ndescribe u \"b\"", 3, "described twice"),
        ("resource u\nunit u\ndescribe u \"a\"", 3, "more than one thing"),
    ];
    for (text, line, fragment) in cases {
        let error = Pack::parse(text).unwrap_err();
        assert_eq!(error.line, line, "{error}");
        assert!(error.message.contains(fragment), "{error}");
    }
}

#[test]
fn declarations_can_follow_the_rules_that_use_them() {
    let text = "unitRule u\na out 1\nend\nresource a # trailing comment\nunit u // another\n";
    let pack = Pack::parse(text).unwrap();
    assert_eq!(pack.content.units()[0].outputs, vec![(ItemId(0), Port::Front)]);
}

#[test]
fn everything_a_player_builds_or_researches_can_be_described() {
    let text = "describe gold \"Shiny.\"\ndescribe belt \"Moves things.\"\ndescribe u \"A unit.\"\n\
                describe t \"Learn it.\"\nresource gold\nunit u\nresearch t\nunlocks u\nend";
    let pack = Pack::parse(text).unwrap();
    let content = &pack.content;
    assert_eq!(content.resources()[0].description, "Shiny.");
    assert_eq!(content.belts()[0].description, "Moves things.");
    assert_eq!(content.units()[0].description, "A unit.");
    assert_eq!(content.techs()[0].description, "Learn it.");
    let plain = World::new(&Pack::parse("resource gold\nunit u\nresearch t\nunlocks u\nend").unwrap());
    assert_eq!(World::new(&pack).state_hash(), plain.state_hash());
}

#[test]
fn every_element_of_the_base_pack_is_described() {
    let pack = Pack::parse(BASE_PACK).unwrap();
    let content = &pack.content;
    let missing: Vec<&str> = (content.belts().iter().map(|b| (&b.id, &b.description)))
        .chain(content.units().iter().map(|u| (&u.id, &u.description)))
        .chain(content.techs().iter().map(|t| (&t.id, &t.description)))
        .chain(content.resources().iter().map(|r| (&r.id, &r.description)))
        .filter(|(_, description)| description.is_empty())
        .map(|(id, _)| id.as_str())
        .collect();
    assert!(missing.is_empty(), "not described: {missing:?}");
}

#[test]
fn cosmetic_changes_keep_the_hash_but_rule_changes_do_not() {
    let hash = |text: &str| World::new(&Pack::parse(text).unwrap()).state_hash();
    let base = hash("resource a name \"A\" sprite 1\nunit u sprite 5\nunitRule u\na out 1\nend");
    let renamed = hash("resource a name \"B\" sprite 2\nunit u sprite 6\nunitRule u\na out 1\nend");
    let changed = hash("resource a\nunit u\nunitRule u\nrate 2\na out 1\nend");
    assert_eq!(base, renamed);
    assert_ne!(base, changed);
}

// Replay tests: random but reproducible command streams.

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn below(&mut self, limit: u64) -> u64 {
        self.next() % limit
    }
}

fn random_commands(rng: &mut Rng) -> Vec<Command> {
    const DIRS: [Dir; 4] = [Dir::North, Dir::East, Dir::South, Dir::West];
    let mut commands = Vec::new();
    for _ in 0..rng.below(4) {
        let pos = p(rng.below(14) as i32, rng.below(14) as i32);
        let dir = DIRS[rng.below(4) as usize];
        let roll = rng.below(100);
        let building = |id| Building::Unit(id);
        commands.push(match roll {
            0..=54 => Command::Place { pos, dir, building: Building::Belt(BeltId(0)) },
            55..=64 => Command::Place { pos, dir, building: building(MINER) },
            65..=72 => {
                let id = if rng.below(2) == 0 { SMELTER } else { ASSEMBLER };
                Command::Place { pos, dir, building: building(id) }
            }
            73..=77 => Command::Place { pos, dir, building: building(DEPOT) },
            _ => Command::Remove { pos },
        });
    }
    commands
}

const FUZZ_EXTRA: &str = r#"
        patch oreField at 0 0 size 14 14 amount 1000000
        mapRule drain
            rate 7
            map oreField in 1
        end
    "#;

fn fuzz_pack_text() -> String {
    pack_text(3, 5, 7, FUZZ_EXTRA)
}

fn fuzz_world() -> World {
    World::new(&Pack::parse(&fuzz_pack_text()).unwrap())
}

fn command_log(seed: u64, ticks: usize) -> Vec<Vec<Command>> {
    let mut rng = Rng(seed);
    (0..ticks).map(|_| random_commands(&mut rng)).collect()
}

#[test]
fn replaying_a_command_log_gives_the_same_state_every_tick() {
    for seed in 1..=20 {
        let log = command_log(0x9E37_79B9_7F4A_7C15_u64.wrapping_mul(seed), 3000);
        let mut first = fuzz_world();
        let mut second = fuzz_world();
        for commands in &log {
            let rejected_first = first.step(commands);
            let rejected_second = second.step(commands);
            assert_eq!(rejected_first, rejected_second);
            assert_eq!(first.state_hash(), second.state_hash(), "tick {}", first.tick());
            assert_conserved(&first);
        }
    }
}

#[test]
fn different_command_logs_diverge() {
    let run_log = |seed| {
        let mut world = fuzz_world();
        for commands in &command_log(seed, 500) {
            world.step(commands);
        }
        world.state_hash()
    };
    assert_ne!(run_log(11), run_log(12));
}

#[test]
fn a_copy_of_the_state_continues_identically() {
    let log = command_log(77, 2000);
    let mut original = fuzz_world();
    for commands in &log[..1000] {
        original.step(commands);
    }
    let mut copy = original.clone();
    for commands in &log[1000..] {
        original.step(commands);
        copy.step(commands);
        assert_eq!(original.state_hash(), copy.state_hash());
    }
}

// Infrastructure: routing, tiers, ports, networks and settings.

/// A source of `ore` at (0, 0) plus whatever `extra` declares.
fn infra(extra: &str) -> Pack {
    let text = format!(
        r#"
        resource ore
        resource plate
        resource juice
        unit src
        unit src2
        unit bin stores any
        unit split routes split
        unit sort routes filter
        unit hole routes tunnel range 3
        unit cook
        unit plant
        unit user
        unit pole links juice reach 2
        unit pipe links juice reach 1
        belt fast speed 64
        unitRule src
            rate 1
            local ore out 1
        end
        unitRule src2
            rate 1
            local plate out 1
        end
        unitRule cook1
            appliesTo cook
            rate 1
            local ore in 1
            local plate out 1
        end
        unitRule cook2
            appliesTo cook
            rate 1
            local plate in 1
            local juice out 1
        end
        unitRule plant
            rate 10
            network juice out 10
        end
        unitRule user
            rate 1
            network juice in 5
            local plate out 1
        end
        {extra}
        "#
    );
    Pack::parse(&text).unwrap()
}

const SRC: UnitId = UnitId(0);
const SRC2: UnitId = UnitId(1);
const BIN: UnitId = UnitId(2);
const SPLIT: UnitId = UnitId(3);
const SORT: UnitId = UnitId(4);
const HOLE: UnitId = UnitId(5);
const COOK: UnitId = UnitId(6);
const PLANT: UnitId = UnitId(7);
const USER: UnitId = UnitId(8);
const POLE: UnitId = UnitId(9);
const PIPE: UnitId = UnitId(10);
const BELT: Building = Building::Belt(BeltId(0));
const FAST: Building = Building::Belt(BeltId(1));

fn put(x: i32, y: i32, dir: Dir, id: UnitId) -> Command {
    place(x, y, dir, Building::Unit(id))
}

#[test]
fn a_splitter_shares_evenly_between_its_outputs() {
    let mut world = World::new(&infra(""));
    let commands = vec![
        put(0, 1, Dir::East, SRC),
        place(1, 1, Dir::East, BELT),
        put(2, 1, Dir::East, SPLIT),
        put(3, 1, Dir::East, BIN),
        put(2, 0, Dir::East, BIN),
        put(2, 2, Dir::East, BIN),
    ];
    run(&mut world, &commands, 3000);
    let counts: Vec<u64> = [p(3, 1), p(2, 0), p(2, 2)].iter().map(|&pos| world.stock(pos, ORE)).collect();
    assert!(counts[0] > 100, "{counts:?}");
    assert!(counts.iter().all(|&c| c.abs_diff(counts[0]) <= 1), "{counts:?}");
    assert_conserved(&world);
}

#[test]
fn a_splitter_sends_everything_to_the_output_that_is_left() {
    let mut world = World::new(&infra(""));
    let commands = vec![
        put(0, 0, Dir::East, SRC),
        place(1, 0, Dir::East, BELT),
        put(2, 0, Dir::East, SPLIT),
        put(2, 1, Dir::East, BIN),
    ];
    run(&mut world, &commands, 2000);
    assert!(world.stock(p(2, 1), ORE) > 100);
    assert_conserved(&world);
}

#[test]
fn a_sorter_separates_the_chosen_resource() {
    let mut world = World::new(&infra(""));
    let commands = vec![
        put(1, 0, Dir::South, SRC),
        put(1, 2, Dir::North, SRC2),
        place(1, 1, Dir::East, BELT),
        place(2, 1, Dir::East, BELT),
        put(3, 1, Dir::East, SORT),
        put(4, 1, Dir::East, BIN),
        put(3, 0, Dir::East, BIN),
        Command::Configure { pos: p(3, 1), setting: Some(PLATE.0) },
    ];
    run(&mut world, &commands, 2000);
    assert!(world.stock(p(4, 1), PLATE) > 50);
    assert_eq!(world.stock(p(4, 1), ORE), 0);
    assert!(world.stock(p(3, 0), ORE) > 50);
    assert_eq!(world.stock(p(3, 0), PLATE), 0);
    assert_conserved(&world);
}

#[test]
fn a_tunnel_carries_items_past_what_is_in_between() {
    let mut world = World::new(&infra(""));
    let commands = vec![
        put(0, 0, Dir::East, SRC),
        put(1, 0, Dir::East, HOLE),
        // Crossing traffic runs over the tunnel without mixing.
        place(2, 1, Dir::North, BELT),
        place(2, 0, Dir::North, BELT),
        put(4, 0, Dir::East, HOLE),
        place(5, 0, Dir::East, BELT),
        put(6, 0, Dir::East, BIN),
    ];
    run(&mut world, &commands, 500);
    assert!(world.stock(p(6, 0), ORE) > 50);
    assert_eq!(world.belt_items().filter(|item| item.0 == p(2, 0)).count(), 0);
    assert_conserved(&world);
}

#[test]
fn a_tunnel_does_not_reach_beyond_its_range() {
    let mut world = World::new(&infra(""));
    let commands = vec![
        put(0, 0, Dir::East, SRC),
        put(1, 0, Dir::East, HOLE),
        put(5, 0, Dir::East, HOLE),
        place(6, 0, Dir::East, BELT),
        put(7, 0, Dir::East, BIN),
    ];
    run(&mut world, &commands, 500);
    assert_eq!(world.stock(p(7, 0), ORE), 0);
    assert_conserved(&world);
}

#[test]
fn belt_tiers_set_throughput_and_items_cross_between_them() {
    let carried = |building, ticks| {
        let mut world = World::new(&infra(""));
        let mut commands = vec![put(0, 0, Dir::East, SRC)];
        commands.extend((1..=4).map(|x| place(x, 0, Dir::East, building)));
        commands.push(put(5, 0, Dir::East, BIN));
        run(&mut world, &commands, ticks);
        assert_conserved(&world);
        world.stock(p(5, 0), ORE)
    };
    let (slow, fast) = (carried(BELT, 2000), carried(FAST, 2000));
    // The source only makes an item every other tick, which caps the fast belt.
    assert!(fast * 10 >= slow * 19, "slow {slow}, fast {fast}");

    let mut world = World::new(&infra(""));
    let commands = vec![
        put(0, 0, Dir::East, SRC),
        place(1, 0, Dir::East, FAST),
        place(2, 0, Dir::East, FAST),
        place(3, 0, Dir::East, BELT),
        place(4, 0, Dir::East, BELT),
        put(5, 0, Dir::East, BIN),
    ];
    run(&mut world, &commands, 1000);
    assert_eq!(world.line_count(), 2);
    assert!(world.stock(p(5, 0), ORE) > 100);
    assert_conserved(&world);
}

#[test]
fn a_byproduct_leaves_from_its_own_side_and_stops_the_unit_when_stuck() {
    let extra = r#"
        unit mill
        unitRule mill
            rate 5
            local ore in 1
            local plate out 1
            local juice out 1 left
        end
    "#;
    let pack = infra(extra);
    let mill = pack.content.unit_id("mill").unwrap();
    let juice = pack.content.resource_id("juice").unwrap();

    let mut world = World::new(&pack);
    let commands = vec![
        put(0, 1, Dir::East, SRC),
        put(1, 1, Dir::East, mill),
        put(2, 1, Dir::East, BIN),
        put(1, 0, Dir::East, BIN),
    ];
    run(&mut world, &commands, 200);
    assert!(world.stock(p(2, 1), PLATE) > 10);
    assert_eq!(world.stock(p(2, 1), juice), 0);
    assert_eq!(world.stock(p(1, 0), juice), world.stock(p(2, 1), PLATE));

    run(&mut world, &[Command::Remove { pos: p(1, 0) }], 200);
    let stuck = world.stock(p(2, 1), PLATE);
    assert_eq!(world.status(p(1, 1)), Some(Status::Full));
    run(&mut world, &[], 200);
    assert_eq!(world.stock(p(2, 1), PLATE), stuck);
    assert_conserved(&world);
}

#[test]
fn a_chosen_recipe_limits_what_a_unit_accepts_and_runs() {
    let setting_commands = |setting| {
        vec![
            put(0, 0, Dir::East, SRC),
            put(1, 0, Dir::East, COOK),
            put(2, 0, Dir::East, BIN),
            Command::Configure { pos: p(1, 0), setting },
        ]
    };
    let mut world = World::new(&infra(""));
    run(&mut world, &setting_commands(Some(1)), 200);
    assert_eq!(world.stock(p(1, 0), ORE), 0);
    assert_eq!(world.stock(p(2, 0), PLATE), 0);

    let mut world = World::new(&infra(""));
    run(&mut world, &setting_commands(None), 200);
    assert!(world.stock(p(2, 0), PLATE) > 10);

    run(&mut world, &[Command::Configure { pos: p(1, 0), setting: Some(0) }], 10);
    let refused = world.step(&[
        Command::Configure { pos: p(1, 0), setting: Some(9) },
        Command::Configure { pos: p(2, 0), setting: Some(0) },
        Command::Configure { pos: p(30, 30), setting: None },
    ]);
    let reasons: Vec<Reject> = refused.iter().map(|r| r.reason).collect();
    assert_eq!(reasons, vec![Reject::InvalidSetting, Reject::NothingToConfigure, Reject::NothingToConfigure]);
}

#[test]
fn networks_join_through_reach_and_supply_their_members() {
    let mut world = World::new(&infra(""));
    let commands = vec![
        put(0, 0, Dir::East, PLANT),
        put(1, 0, Dir::East, PIPE),
        put(3, 0, Dir::East, POLE),
        put(5, 0, Dir::East, USER),
        put(6, 0, Dir::East, BIN),
    ];
    run(&mut world, &commands, 300);
    assert!(world.stock(p(6, 0), PLATE) > 20);
    let view = world.inspect(p(5, 0)).unwrap();
    assert_eq!(view.networks[0].members, 4);
    assert_eq!(view.networks[0].producers, 1);
    assert_conserved(&world);
}

#[test]
fn a_unit_out_of_reach_reports_that_it_has_no_supply() {
    let mut world = World::new(&infra(""));
    let commands = vec![put(0, 0, Dir::East, PLANT), put(1, 0, Dir::East, POLE), put(9, 0, Dir::East, USER)];
    run(&mut world, &commands, 100);
    assert_eq!(world.status(p(9, 0)), Some(Status::NoNetwork(ItemId(2))));
    assert_eq!(world.status(p(0, 0)), Some(Status::Standby));
    assert_eq!(world.inspect(p(9, 0)).unwrap().networks[0].members, 1);
}

#[test]
fn removing_a_producer_destroys_only_its_share_of_the_network() {
    let mut world = World::new(&infra(""));
    let commands = vec![
        put(0, 0, Dir::East, PLANT),
        put(1, 0, Dir::East, POLE),
        put(2, 0, Dir::East, PLANT),
    ];
    run(&mut world, &commands, 50);
    // Each plant stops at two batches.
    assert_eq!(world.inspect(p(1, 0)).unwrap().networks[0].stock, 40);
    run(&mut world, &[Command::Remove { pos: p(0, 0) }], 1);
    assert_eq!(world.inspect(p(1, 0)).unwrap().networks[0].stock, 20);
    assert_eq!(world.stats(ItemId(2)).destroyed, 20);
    assert_conserved(&world);
}

#[test]
fn a_unit_can_be_placed_on_any_of_its_layers() {
    let extra = r#"
        layer a
        layer b
        unit digger placeOn a placeOn b
        patch a at 0 0 size 1 1 amount 5
        patch b at 1 0 size 1 1 amount 5
    "#;
    let pack = infra(extra);
    let digger = Building::Unit(pack.content.unit_id("digger").unwrap());
    let mut world = World::new(&pack);
    let commands = [place(0, 0, Dir::East, digger), place(1, 0, Dir::East, digger), place(2, 0, Dir::East, digger)];
    let refused = world.step(&commands);
    assert_eq!(refused, vec![Rejected { index: 2, reason: Reject::NoResource }]);
}

fn infra_fuzz_commands(rng: &mut Rng) -> Vec<Command> {
    const DIRS: [Dir; 4] = [Dir::North, Dir::East, Dir::South, Dir::West];
    const UNITS: [UnitId; 11] = [SRC, SRC2, BIN, SPLIT, SORT, HOLE, COOK, PLANT, USER, POLE, PIPE];
    let mut commands = Vec::new();
    for _ in 0..rng.below(4) {
        let pos = p(rng.below(12) as i32, rng.below(12) as i32);
        let dir = DIRS[rng.below(4) as usize];
        commands.push(match rng.below(100) {
            0..=44 => place(pos.x, pos.y, dir, Building::Belt(BeltId(rng.below(2) as u16))),
            45..=74 => put(pos.x, pos.y, dir, UNITS[rng.below(UNITS.len() as u64) as usize]),
            75..=84 => Command::Configure { pos, setting: Some(rng.below(3) as u16) },
            _ => Command::Remove { pos },
        });
    }
    commands
}

#[test]
fn infrastructure_replays_identically_and_conserves_everything() {
    for seed in 1..=10 {
        let mut rng = Rng(0xD1B5_4A32_D192_ED03_u64.wrapping_mul(seed));
        let log: Vec<Vec<Command>> = (0..3000).map(|_| infra_fuzz_commands(&mut rng)).collect();
        let (mut first, mut second) = (World::new(&infra("")), World::new(&infra("")));
        for commands in &log {
            assert_eq!(first.step(commands), second.step(commands));
            assert_eq!(first.state_hash(), second.state_hash(), "tick {}", first.tick());
            assert_conserved(&first);
        }
    }
}

// The base pack.

const BASE_PACK: &str = include_str!("../../../packs/base.gbx");

/// Every pack the game ships, by name.
const ALL_PACKS: [(&str, &str); 5] = [
    ("base", BASE_PACK),
    ("tutorial", include_str!("../../../packs/tutorial.gbx")),
    ("farm", include_str!("../../../packs/farm.gbx")),
    ("chips", include_str!("../../../packs/chips.gbx")),
    ("network", include_str!("../../../packs/network.gbx")),
];

/// Test-only units that supply every resource, as items or over a network.
fn supply_units(base: &Pack) -> String {
    let mut text = String::new();
    for resource in base.content.resources() {
        let id = &resource.id;
        text += &format!("unit give_{id}\nunitRule give_{id}\nlocal {id} out 1\nend\n");
        text += &format!("unit charge_{id}\nunitRule charge_{id}\nnetwork {id} out 1000\nend\n");
        text += &format!("unit reach_{id} links {id} reach 8\n");
    }
    text
}

/// The ground a rule has to stand on, if any. A haze like smog is an effect, not a ground.
fn ground_of(base: &Pack, rule: &Rule) -> Option<LayerId> {
    let is_ground = |layer: LayerId| base.content.layers()[usize::from(layer.0)].overlay.is_none();
    let from_takes = rule.takes.iter().find_map(|entry| match entry.scope {
        Scope::Map { layer, .. } if is_ground(layer) => Some(layer),
        _ => None,
    });
    from_takes.or_else(|| rule.conditions.iter().map(|c| c.layer).find(|&layer| is_ground(layer)))
}

#[test]
fn every_recipe_in_every_pack_runs_when_fed() {
    let mut checked = 0;
    for (name, text) in ALL_PACKS {
        checked += check_every_recipe(name, text);
    }
    assert!(checked > 70, "{checked}");
}

fn check_every_recipe(pack_name: &str, pack_text: &str) -> usize {
    let base = Pack::parse(pack_text).unwrap();
    let supplies = supply_units(&base);
    let mut checked = 0;
    for (unit_index, unit) in base.content.units().iter().enumerate() {
        for (rule_index, rule) in unit.rules.iter().enumerate() {
            let patch = ground_of(&base, rule).map_or(String::new(), |layer| {
                format!("patch {} at 0 0 size 1 1 amount 1000000\n", base.content.layers()[usize::from(layer.0)].id)
            });
            let pack = Pack::parse(&format!("{pack_text}\nsandbox\n{supplies}\n{patch}")).unwrap();
            let helper = |prefix: &str, item: ItemId| {
                let name = format!("{prefix}_{}", base.content.resources()[usize::from(item.0)].id);
                Building::Unit(pack.content.unit_id(&name).unwrap())
            };

            let origin = p(0, 0);
            let building = Building::Unit(UnitId(unit_index as u16));
            let mut commands = vec![Command::Place { pos: origin, dir: Dir::East, building }];
            if unit.rules.len() > 1 {
                commands.push(Command::Configure { pos: origin, setting: Some(rule_index as u16) });
            }
            let mut sides = [(Dir::West, p(-1, 0)), (Dir::North, p(0, -1)), (Dir::South, p(0, 1))].into_iter();
            let mut networks = 0;
            for take in &rule.takes {
                match take.scope {
                    Scope::Local(item) => {
                        let (side, pos) = sides.next().expect("a recipe has at most three solid inputs");
                        commands.push(Command::Place { pos, dir: side.opposite(), building: helper("give", item) });
                    }
                    Scope::Network(item) => {
                        // A long-reach pole joins the unit to its own supply.
                        commands.push(Command::Place { pos: p(networks, 3), dir: Dir::East, building: helper("reach", item) });
                        commands.push(Command::Place { pos: p(networks, 5), dir: Dir::East, building: helper("charge", item) });
                        networks += 1;
                    }
                    _ => {}
                }
            }

            let mut world = World::new(&pack);
            assert_eq!(world.step(&commands), vec![], "placing for rule {}", rule.name);
            for _ in 0..(rule.rate + 200) {
                world.step(&[]);
            }
            let moved = |list: &[Amount], total: &dyn Fn(ItemId) -> u64| {
                list.iter().all(|entry| match entry.scope {
                    Scope::Local(item) | Scope::Network(item) => total(item) > 0,
                    _ => true,
                })
            };
            assert!(
                moved(&rule.gives, &|item| world.stats(item).produced) && moved(&rule.takes, &|item| world.stats(item).consumed),
                "rule {} of unit {} in {pack_name} did not run",
                rule.name,
                unit.id
            );
            assert_conserved(&world);
            checked += 1;
        }
    }
    checked
}

#[test]
fn a_plant_with_spare_capacity_keeps_a_steady_supply() {
    let mut world = World::new(&infra(""));
    let commands = vec![put(0, 0, Dir::East, PLANT), put(1, 0, Dir::East, POLE), put(2, 0, Dir::East, USER), put(3, 0, Dir::East, BIN)];
    run(&mut world, &commands, 20);
    // The user draws 5 per 2 ticks at best, the plant makes 10 per 10 ticks,
    // so it is supply-limited but never starved for a whole plant cycle.
    let before = world.stock(p(3, 0), PLATE);
    run(&mut world, &[], 100);
    assert!(world.stock(p(3, 0), PLATE) - before >= 15);
}

#[test]
fn packed_coal_furnaces_choke_on_their_own_smog() {
    const FEEDER: &str = "unit feeder\nunitRule feeder\nlocal ironOre out 2\nlocal coal out 1\nend";
    let pack = Pack::parse(&format!("{BASE_PACK}\nsandbox\n{FEEDER}")).unwrap();
    let id = |name: &str| Building::Unit(pack.content.unit_id(name).unwrap());
    let smog = pack.content.layer_id("smog").unwrap();
    // Each furnace is fed from the south and delivers to a depot to the north.
    let bay = |x: i32| {
        [
            place(x, 0, Dir::North, id("furnace")),
            place(x, 1, Dir::North, id("feeder")),
            place(x, -1, Dir::North, id("depot")),
        ]
    };

    let mut world = World::new(&pack);
    let commands: Vec<Command> = (0..5).flat_map(bay).collect();
    assert_eq!(world.step(&commands), vec![]);
    let (mut choked, mut highest) = (false, 0);
    for _ in 0..12000 {
        world.step(&[]);
        choked |= world.status(p(2, 0)) == Some(Status::Conditions);
        highest = highest.max(world.layer_value(smog, p(2, 0)));
    }
    assert!(choked, "smog peaked at {highest}");
    assert!(highest < 400, "furnaces should stop before smog runs away: {highest}");
    assert_conserved(&world);

    // One furnace on its own never gets near the limit.
    let mut lone = World::new(&pack);
    assert_eq!(lone.step(&bay(0)), vec![]);
    for _ in 0..12000 {
        lone.step(&[]);
        assert_ne!(lone.status(p(0, 0)), Some(Status::Conditions));
    }
    assert!(lone.stats(pack.content.resource_id("ironPlate").unwrap()).produced > 150);
}

#[test]
fn every_base_pack_input_has_a_source() {
    let base = Pack::parse(BASE_PACK).unwrap();
    let produced: Vec<ItemId> = base
        .content
        .units()
        .iter()
        .flat_map(|unit| &unit.rules)
        .flat_map(|rule| &rule.gives)
        .filter_map(|give| match give.scope {
            Scope::Local(item) | Scope::Network(item) => Some(item),
            _ => None,
        })
        .collect();
    for (index, resource) in base.content.resources().iter().enumerate() {
        assert!(produced.contains(&ItemId(index as u16)), "nothing makes {}", resource.id);
    }
}

#[test]
fn the_base_pack_starter_runs_on_its_own() {
    let pack = Pack::parse(BASE_PACK).unwrap();
    let mut world = World::new(&pack);
    assert_eq!(world.step(&pack.worldgen.starter), vec![]);
    for _ in 0..7200 {
        world.step(&[]);
    }
    let plate = pack.content.resource_id("ironPlate").unwrap();
    assert!(world.stats(plate).produced > 100, "plates");
    assert!(world.global(plate) > 100, "delivered to the shared stock");
    assert_conserved(&world);
}

// Costs and research.

const PROGRESS: &str = r#"
    resource ore
    resource plate
    unit src
    unit bank stores shared
    unit cook
    unit lab
    belt fast speed 32
    cost src plate 5
    cost belt plate 1
    cost fast plate 2
    stock plate 20
    unitRule src
        rate 1
        local ore out 1
    end
    unitRule open
        appliesTo cook
        rate 1
        local plate in 1
        local ore out 1
    end
    unitRule hidden
        appliesTo cook
        rate 1
        local ore in 1
        local plate out 1
    end
    research first
        cost plate 5
        time 10
        unlocks fast hidden
    end
    research second
        requires first
        cost plate 5
        time 3
        unlocks lab
    end
"#;

const STOCK_PLATE: ItemId = ItemId(1);

#[test]
fn buildings_are_paid_for_and_refunded() {
    let pack = Pack::parse(PROGRESS).unwrap();
    let src = Building::Unit(pack.content.unit_id("src").unwrap());
    let mut world = World::new(&pack);
    assert_eq!(world.global(STOCK_PLATE), 20);
    run(&mut world, &[place(0, 0, Dir::East, src), place(1, 0, Dir::East, BELT)], 1);
    assert_eq!(world.global(STOCK_PLATE), 14);
    assert_conserved(&world);
    run(&mut world, &[Command::Remove { pos: p(0, 0) }], 1);
    assert_eq!(world.global(STOCK_PLATE), 19);
    assert_conserved(&world);

    let many: Vec<Command> = (0..5).map(|x| place(x, 3, Dir::East, src)).collect();
    let refused = world.step(&many);
    assert_eq!(refused.len(), 2);
    assert!(refused.iter().all(|r| r.reason == Reject::CantAfford));
    assert_eq!(world.global(STOCK_PLATE), 4);
}

#[test]
fn a_depot_adds_to_the_stock_that_pays_for_things() {
    let pack = Pack::parse(PROGRESS).unwrap();
    let src = pack.content.unit_id("src").unwrap();
    let bank = pack.content.unit_id("bank").unwrap();
    let mut world = World::new(&pack);
    run(&mut world, &[put(0, 0, Dir::East, src), put(1, 0, Dir::East, bank)], 100);
    assert!(world.global(ItemId(0)) > 10);
    assert_eq!(world.stock(p(1, 0), ItemId(0)), 0);
    assert_conserved(&world);
}

#[test]
fn research_unlocks_buildings_and_recipes_in_order() {
    let pack = Pack::parse(PROGRESS).unwrap();
    let (first, second) = (pack.content.tech_id("first").unwrap(), pack.content.tech_id("second").unwrap());
    let fast = Building::Belt(pack.content.belt_id("fast").unwrap());
    let cook = pack.content.unit_id("cook").unwrap();
    let lab = Building::Unit(pack.content.unit_id("lab").unwrap());
    let mut world = World::new(&pack);

    let reasons = |refused: Vec<Rejected>| refused.iter().map(|r| r.reason).collect::<Vec<_>>();
    assert_eq!(reasons(world.step(&[place(0, 0, Dir::East, fast)])), vec![Reject::Locked]);
    assert_eq!(reasons(world.step(&[Command::Research { tech: second }])), vec![Reject::MissingRequirement]);

    assert_eq!(world.step(&[put(5, 5, Dir::East, cook), Command::Research { tech: first }]), vec![]);
    assert_eq!(world.global(STOCK_PLATE), 15);
    assert_eq!(world.active_research().map(|(tech, _)| tech), Some(first));
    assert_eq!(reasons(world.step(&[Command::Research { tech: second }])), vec![Reject::ResearchBusy]);
    assert_eq!(reasons(world.step(&[Command::Configure { pos: p(5, 5), setting: Some(1) }])), vec![Reject::Locked]);
    assert_conserved(&world);

    run(&mut world, &[], 8);
    assert!(world.researched(first));
    assert_eq!(world.active_research(), None);
    run(&mut world, &[place(0, 0, Dir::East, fast)], 1);
    run(&mut world, &[Command::Configure { pos: p(5, 5), setting: Some(1) }], 1);
    assert_eq!(reasons(world.step(&[Command::Research { tech: first }])), vec![Reject::AlreadyResearched]);
    assert_eq!(reasons(world.step(&[Command::Research { tech: TechId(9) }])), vec![Reject::UnknownTech]);

    assert_eq!(reasons(world.step(&[place(9, 9, Dir::East, lab)])), vec![Reject::Locked]);
    run(&mut world, &[Command::Research { tech: second }], 5);
    run(&mut world, &[place(9, 9, Dir::East, lab)], 1);
    assert_conserved(&world);
}

#[test]
fn a_locked_recipe_neither_runs_nor_accepts_its_inputs() {
    let pack = Pack::parse(PROGRESS).unwrap();
    let cook = pack.content.unit_id("cook").unwrap();
    let src = pack.content.unit_id("src").unwrap();
    let mut world = World::new(&pack);
    run(&mut world, &[put(0, 0, Dir::East, src), put(1, 0, Dir::East, cook)], 100);
    assert_eq!(world.stock(p(1, 0), ItemId(0)), 0);

    run(&mut world, &[Command::Research { tech: pack.content.tech_id("first").unwrap() }], 20);
    assert!(world.stats(STOCK_PLATE).produced > 0);
    assert_conserved(&world);
}

#[test]
fn research_state_is_part_of_the_hash() {
    let pack = Pack::parse(PROGRESS).unwrap();
    let mut idle = World::new(&pack);
    let mut busy = World::new(&pack);
    idle.step(&[]);
    busy.step(&[Command::Research { tech: TechId(0) }]);
    assert_ne!(idle.state_hash(), busy.state_hash());
}

#[test]
fn a_sandbox_pack_is_free_and_open() {
    let pack = Pack::parse(&format!("{PROGRESS}\nsandbox")).unwrap();
    let fast = Building::Belt(pack.content.belt_id("fast").unwrap());
    let mut world = World::new(&pack);
    run(&mut world, &[place(0, 0, Dir::East, fast)], 1);
    assert_eq!(world.global(STOCK_PLATE), 20);
}

/// Works through the tree as a player would: research anything whose cost can
/// be made with what is open, until nothing changes. Everything must open.
#[test]
fn the_whole_base_pack_research_tree_can_be_reached() {
    let pack = Pack::parse(BASE_PACK).unwrap();
    let content = &pack.content;
    let mut done: Vec<TechId> = Vec::new();
    let open = |locked: Option<TechId>, done: &[TechId]| locked.map_or(true, |tech| done.contains(&tech));
    loop {
        // Items the open buildings and recipes can make, from what is already makeable.
        let mut makeable: Vec<ItemId> = Vec::new();
        loop {
            let before = makeable.len();
            for unit in content.units().iter().filter(|u| open(u.locked_by, &done)) {
                for rule in unit.rules.iter().filter(|r| open(r.locked_by, &done)) {
                    let inputs_ready = rule.takes.iter().all(|take| match take.scope {
                        Scope::Local(item) | Scope::Network(item) | Scope::Global(item) => makeable.contains(&item),
                        Scope::Map { .. } => true,
                    });
                    if !inputs_ready {
                        continue;
                    }
                    for give in &rule.gives {
                        if let Scope::Local(item) | Scope::Network(item) | Scope::Global(item) = give.scope {
                            if !makeable.contains(&item) {
                                makeable.push(item);
                            }
                        }
                    }
                }
            }
            if makeable.len() == before {
                break;
            }
        }
        let next = (0..content.techs().len() as u16).map(TechId).find(|tech| {
            let definition = &content.techs()[usize::from(tech.0)];
            !done.contains(tech)
                && definition.requires.iter().all(|required| done.contains(required))
                && definition.cost.iter().all(|(item, _)| makeable.contains(item))
        });
        match next {
            Some(tech) => done.push(tech),
            None => break,
        }
    }
    let stuck: Vec<&str> = content
        .techs()
        .iter()
        .enumerate()
        .filter(|(index, _)| !done.contains(&TechId(*index as u16)))
        .map(|(_, tech)| tech.id.as_str())
        .collect();
    assert!(stuck.is_empty(), "unreachable research: {stuck:?}");
}

/// Costs must only ever use things the player can make before needing them.
#[test]
fn base_pack_starter_is_affordable() {
    let pack = Pack::parse(BASE_PACK).unwrap();
    let mut world = World::new(&pack);
    assert_eq!(world.step(&pack.worldgen.starter), vec![]);
    let plate = pack.content.resource_id("ironPlate").unwrap();
    assert!(world.global(plate) >= 30, "enough left to get going");
}

// --- Levels

const FLOORS: &str = r#"
    levels 4
    span 2
    resource ore
    resource power
    layer oreField
    layer fumes
    unit miner placeOn oreField
    unit hoist routes up range 2
    unit chute routes down range 2
    unit pillar
    unit bin stores any
    unit plant links power reach 2
    unit lamp links power reach 2
    unit stack
    unitRule mine
        appliesTo miner
        rate 4
        map oreField in 1
        local ore out 1
    end
    unitRule burn
        appliesTo plant
        rate 2
        network power out 10
    end
    unitRule shine
        appliesTo lamp
        rate 2
        network power in 1
        local ore out 1
    end
    unitRule smoke
        appliesTo stack
        rate 2
        map fumes out 1
    end
    patch oreField at 0 0 size 1 1 amount 1000000
"#;

fn floors() -> (World, Floors) {
    let pack = Pack::parse(FLOORS).unwrap();
    let id = |name: &str| pack.content.unit_id(name).unwrap();
    let units = Floors {
        miner: id("miner"),
        hoist: id("hoist"),
        chute: id("chute"),
        pillar: id("pillar"),
        bin: id("bin"),
        plant: id("plant"),
        lamp: id("lamp"),
        stack: id("stack"),
    };
    (World::new(&pack), units)
}

struct Floors {
    miner: UnitId,
    hoist: UnitId,
    chute: UnitId,
    pillar: UnitId,
    bin: UnitId,
    plant: UnitId,
    lamp: UnitId,
    stack: UnitId,
}

fn at3(x: i32, y: i32, z: i32, dir: Dir, building: Building) -> Command {
    Command::Place { pos: Pos::at(x, y, z), dir, building }
}

fn unit3(x: i32, y: i32, z: i32, id: UnitId) -> Command {
    at3(x, y, z, Dir::East, Building::Unit(id))
}

fn belt3(x: i32, y: i32, z: i32, dir: Dir) -> Command {
    at3(x, y, z, dir, Building::Belt(BeltId(0)))
}

fn reasons(world: &mut World, commands: &[Command]) -> Vec<Reject> {
    world.step(commands).into_iter().map(|rejected| rejected.reason).collect()
}

#[test]
fn a_hoist_lifts_items_and_a_chute_brings_them_down() {
    let (mut world, u) = floors();
    run(
        &mut world,
        &[
            unit3(0, 0, 0, u.miner),
            unit3(1, 0, 0, u.hoist),
            unit3(1, 0, 1, u.hoist),
            belt3(2, 0, 1, Dir::East),
            unit3(3, 0, 1, u.chute),
            unit3(3, 0, 0, u.chute),
            unit3(4, 0, 0, u.bin),
        ],
        800,
    );
    assert!(world.stock(p(4, 0), ORE) > 100, "{}", world.stock(p(4, 0), ORE));
    assert_conserved(&world);

    let top = world.inspect(Pos::at(1, 0, 1)).unwrap();
    assert_eq!((top.sends_to, top.receives_from), (None, Some(0)));
    let bottom = world.inspect(p(1, 0)).unwrap();
    assert_eq!((bottom.sends_to, bottom.receives_from), (Some(1), None));
}

#[test]
fn a_belt_can_cross_over_another() {
    let (mut world, u) = floors();
    let ground_belt = |x| belt3(x, 3, 0, Dir::East);
    // The ground belt goes first, because it holds up the far end of the bridge.
    let mut commands: Vec<Command> = (0..=2).map(ground_belt).collect();
    commands.extend([
        unit3(0, 0, 0, u.miner),
        // Down the column at x=1, up one level at y=1, over the ground belt at y=3.
        at3(1, 0, 0, Dir::South, Building::Belt(BeltId(0))),
        unit3(1, 1, 0, u.hoist),
        at3(1, 1, 1, Dir::South, Building::Unit(u.hoist)),
        belt3(1, 2, 1, Dir::South),
        belt3(1, 3, 1, Dir::South),
        belt3(1, 4, 1, Dir::South),
        unit3(1, 5, 1, u.bin),
    ]);
    run(&mut world, &commands, 800);
    assert!(world.stock(Pos::at(1, 5, 1), ORE) > 100);
    assert_eq!(world.line_count(), 3, "the crossing belts stay separate lines");
    assert_conserved(&world);
}

#[test]
fn belts_on_different_levels_do_not_connect() {
    let (mut world, u) = floors();
    // A level-1 belt whose front is over a ground bin delivers nowhere.
    run(
        &mut world,
        &[unit3(0, 0, 0, u.pillar), belt3(0, 0, 1, Dir::East), unit3(1, 0, 0, u.bin)],
        5,
    );
    assert_eq!(world.inspect(p(1, 0)).unwrap().held.len(), 0);
    assert_eq!(world.line_count(), 1);
}

#[test]
fn upper_levels_need_something_below_them() {
    let (mut world, u) = floors();
    assert_eq!(reasons(&mut world, &[belt3(10, 10, 1, Dir::East)]), vec![Reject::Unsupported]);
    assert_eq!(reasons(&mut world, &[unit3(10, 10, 0, u.pillar)]), vec![]);
    // Within the span of 2, along the grid; a diagonal step counts as two.
    assert_eq!(reasons(&mut world, &[belt3(12, 10, 1, Dir::East)]), vec![]);
    assert_eq!(reasons(&mut world, &[belt3(11, 11, 1, Dir::East)]), vec![]);
    assert_eq!(reasons(&mut world, &[belt3(13, 10, 1, Dir::East)]), vec![Reject::Unsupported]);
    assert_eq!(reasons(&mut world, &[belt3(12, 11, 1, Dir::East)]), vec![Reject::Unsupported]);
    // The level above needs something on level 1, not on the ground.
    assert_eq!(reasons(&mut world, &[belt3(13, 10, 2, Dir::East)]), vec![]);
    assert_eq!(reasons(&mut world, &[belt3(10, 13, 2, Dir::East)]), vec![Reject::Unsupported]);
}

#[test]
fn a_support_cannot_be_pulled_out_from_under_a_load() {
    let (mut world, u) = floors();
    run(
        &mut world,
        &[unit3(10, 10, 0, u.pillar), unit3(11, 10, 0, u.pillar), belt3(12, 10, 1, Dir::East)],
        1,
    );
    // Either pillar can go, but not both.
    assert_eq!(reasons(&mut world, &[Command::Remove { pos: p(11, 10) }]), vec![]);
    assert_eq!(reasons(&mut world, &[Command::Remove { pos: p(10, 10) }]), vec![Reject::CarriesLoad]);
    assert!(world.inspect(p(10, 10)).is_some());
    assert_eq!(reasons(&mut world, &[Command::Remove { pos: Pos::at(12, 10, 1) }]), vec![]);
    assert_eq!(reasons(&mut world, &[Command::Remove { pos: p(10, 10) }]), vec![]);
}

#[test]
fn levels_are_bounded_and_the_ground_stays_on_the_ground() {
    let (mut world, u) = floors();
    assert_eq!(reasons(&mut world, &[belt3(5, 5, -1, Dir::East)]), vec![Reject::NoSuchLevel]);
    assert_eq!(reasons(&mut world, &[belt3(5, 5, 4, Dir::East)]), vec![Reject::NoSuchLevel]);
    // A drill above the ore can't reach it.
    assert_eq!(reasons(&mut world, &[unit3(0, 1, 0, u.pillar), unit3(0, 0, 1, u.miner)]), vec![Reject::NoResource]);

    // A flat pack has no upper levels at all.
    let mut flat = ore_world();
    assert_eq!(reasons(&mut flat, &[belt3(1, 1, 1, Dir::East)]), vec![Reject::NoSuchLevel]);
}

#[test]
fn a_blocked_shaft_breaks_the_lift() {
    let (mut world, u) = floors();
    run(
        &mut world,
        &[unit3(5, 5, 0, u.hoist), unit3(6, 5, 0, u.pillar), unit3(6, 5, 1, u.pillar), unit3(5, 5, 2, u.hoist)],
        2,
    );
    assert_eq!(world.inspect(p(5, 5)).unwrap().sends_to, Some(2));
    run(&mut world, &[unit3(5, 5, 1, u.bin)], 2);
    assert_eq!(world.inspect(p(5, 5)).unwrap().sends_to, None);
    assert_eq!(world.status(p(5, 5)), Some(Status::Unlinked));
}

#[test]
fn lifts_only_pair_with_their_own_kind_and_direction() {
    let (mut world, u) = floors();
    run(&mut world, &[unit3(5, 5, 0, u.hoist), unit3(5, 5, 1, u.chute)], 2);
    assert_eq!(world.status(p(5, 5)), Some(Status::Unlinked));
    assert_eq!(world.status(Pos::at(5, 5, 1)), Some(Status::Unlinked));
}

#[test]
fn networks_reach_between_levels_and_height_counts_as_distance() {
    let (mut world, u) = floors();
    run(
        &mut world,
        &[
            unit3(5, 5, 0, u.plant),
            unit3(5, 6, 0, u.pillar),
            // One up and one along: two steps, inside the reach of 2.
            unit3(5, 6, 1, u.lamp),
            // Two up and two along: four steps from the plant, and three from the lamp.
            unit3(7, 5, 1, u.pillar),
            unit3(7, 5, 2, u.lamp),
        ],
        30,
    );
    let view = |pos| world.inspect(pos).unwrap().networks[0];
    assert_eq!(view(Pos::at(5, 6, 1)).producers, 1);
    assert!(world.stats(ORE).produced > 0, "the lamp in reach worked");
    assert_eq!(view(Pos::at(7, 5, 2)).members, 1);
}

#[test]
fn everything_on_the_map_is_read_from_the_ground_below() {
    let (mut world, u) = floors();
    run(&mut world, &[unit3(8, 9, 0, u.pillar), unit3(8, 8, 1, u.stack)], 11);
    let fumes = world.content().layer_id("fumes").unwrap();
    assert_eq!(world.layer_value(fumes, p(8, 8)), 3, "a rule takes rate + 1 ticks per cycle");
    assert_eq!(world.layer_value(fumes, Pos::at(8, 8, 1)), 3);
}

#[test]
fn level_fuzz_replays_identically_and_conserves_items() {
    let build = |seed: u64| -> Vec<Vec<Command>> {
        let (_, u) = floors();
        let mut rng = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1;
        let mut next = move |bound: u64| {
            rng ^= rng << 13;
            rng ^= rng >> 7;
            rng ^= rng << 17;
            rng % bound
        };
        let units = [u.miner, u.hoist, u.chute, u.pillar, u.bin, u.plant, u.lamp, u.stack];
        let dirs = [Dir::North, Dir::East, Dir::South, Dir::West];
        (0..600)
            .map(|_| {
                (0..next(4))
                    .map(|_| {
                        let pos = Pos::at(next(8) as i32, next(8) as i32, next(5) as i32 - (next(2) as i32 & next(2) as i32));
                        let dir = dirs[next(4) as usize];
                        match next(5) {
                            0 => Command::Remove { pos },
                            1 | 2 => Command::Place { pos, dir, building: Building::Belt(BeltId(0)) },
                            _ => Command::Place { pos, dir, building: Building::Unit(units[next(8) as usize]) },
                        }
                    })
                    .collect()
            })
            .collect()
    };
    for seed in 1..8 {
        let log = build(seed);
        let (mut first, _) = floors();
        let (mut second, _) = floors();
        for commands in &log {
            assert_eq!(first.step(commands), second.step(commands));
            assert_eq!(first.state_hash(), second.state_hash());
            assert_conserved(&first);
        }
    }
}

// Saves.

/// Runs `log`, saving and loading at every `every` ticks, and checks the loaded
/// copy matches the original and keeps matching for the next `ahead` ticks.
fn check_saves(pack: &Pack, log: &[Vec<Command>], every: usize, ahead: usize) -> World {
    let mut world = World::new(pack);
    for (tick, commands) in log.iter().enumerate() {
        world.step(commands);
        if tick % every != every - 1 {
            continue;
        }
        let bytes = world.save();
        let mut copy = World::load(pack, &bytes).unwrap();
        assert_eq!(copy.state_hash(), world.state_hash(), "tick {}", world.tick());
        assert_eq!(copy.save(), bytes, "saving a loaded world gives the same bytes");
        let mut original = world.clone();
        for commands in log.iter().skip(tick + 1).take(ahead) {
            assert_eq!(original.step(commands), copy.step(commands));
            assert_eq!(original.state_hash(), copy.state_hash(), "tick {}", original.tick());
        }
    }
    world
}

#[test]
fn a_saved_world_loads_and_continues_identically() {
    let fuzz = fuzz_world();
    let fuzz_pack = Pack::parse(&fuzz_pack_text()).unwrap();
    assert_eq!(World::new(&fuzz_pack).state_hash(), fuzz.state_hash());
    for seed in 1..=4 {
        check_saves(&fuzz_pack, &command_log(0x51ED_270B_9A3C_4E1F_u64.wrapping_mul(seed), 1500), 250, 120);
        let mut rng = Rng(0xA076_1D64_78BD_642F_u64.wrapping_mul(seed));
        let log: Vec<Vec<Command>> = (0..1500).map(|_| infra_fuzz_commands(&mut rng)).collect();
        check_saves(&infra(""), &log, 250, 120);
    }
}

#[test]
fn saves_keep_levels_research_and_the_base_pack_running() {
    let base = Pack::parse(BASE_PACK).unwrap();
    let mut log: Vec<Vec<Command>> = vec![base.worldgen.starter.clone()];
    log.resize(4000, Vec::new());
    let routing = base.content.tech_id("routing").unwrap();
    log[1900].push(Command::Research { tech: routing });
    // Saved at 2100 with the research running, which finishes at 2500.
    let world = check_saves(&base, &log, 700, 300);
    assert!(world.researched(routing));

    let floors_pack = Pack::parse(FLOORS).unwrap();
    let (_, u) = floors();
    let mut log: Vec<Vec<Command>> = vec![vec![
        unit3(0, 0, 0, u.miner),
        unit3(1, 0, 0, u.hoist),
        unit3(1, 0, 1, u.hoist),
        belt3(2, 0, 1, Dir::East),
        unit3(3, 0, 1, u.chute),
        unit3(3, 0, 0, u.chute),
        unit3(4, 0, 0, u.bin),
    ]];
    log.resize(900, Vec::new());
    let world = check_saves(&floors_pack, &log, 150, 100);
    assert!(world.stock(p(4, 0), ORE) > 100);
}

#[test]
fn damaged_or_foreign_saves_are_refused_without_panicking() {
    let pack = Pack::parse(&fuzz_pack_text()).unwrap();
    let mut world = World::new(&pack);
    for commands in &command_log(99, 800) {
        world.step(commands);
    }
    let bytes = world.save();
    for end in 0..bytes.len() {
        assert!(World::load(&pack, &bytes[..end]).is_err(), "cut at {end}");
    }
    let mut longer = bytes.clone();
    longer.push(0);
    assert!(matches!(World::load(&pack, &longer), Err(LoadError::Damaged(_))));
    let mut newer = bytes.clone();
    newer[4] = 3;
    assert_eq!(World::load(&pack, &newer).unwrap_err(), LoadError::Version(3));
    let other = Pack::parse(&fuzz_pack_text().replace("rate 7", "rate 8")).unwrap();
    assert_eq!(World::load(&other, &bytes).unwrap_err(), LoadError::OtherPack);
    let renamed = Pack::parse(&format!("{}\ndescribe ore \"Rocks.\"", fuzz_pack_text())).unwrap();
    assert!(World::load(&renamed, &bytes).is_ok(), "descriptions are cosmetic");

    // Flipped bytes either fail to load or load into a world that still runs.
    let mut rng = Rng(12345);
    for _ in 0..3000 {
        let mut damaged = bytes.clone();
        for _ in 0..=rng.below(3) {
            let at = 13 + rng.below(damaged.len() as u64 - 13) as usize;
            damaged[at] ^= 1 << rng.below(8);
        }
        if let Ok(mut loaded) = World::load(&pack, &damaged) {
            for _ in 0..20 {
                loaded.step(&[]);
            }
        }
    }
}

// Puzzles and starts.

const PUZZLES: &str = r#"
    resource ore
    resource plate
    layer oreField
    metric made
    unit miner placeOn oreField
    unit depot stores shared
    unit other
    unitRule mine
        appliesTo miner
        rate 10
        map oreField in 1
        local ore out 1
    end
    unitRule tinker
        appliesTo other
        rate 10
        local plate out 1
        log made 1
    end
    patch oreField at 0 0 size 1 1 amount 1000000
    stock plate 100
    cost miner plate 10
    cost depot plate 10
    cost belt plate 1
    research smelting
        cost plate 5
        time 10
    end
    puzzle haul
        name "Haul"
        about "Deliver ore."
        only miner belt depot
        goal deliver ore 5
    end
    puzzle rush
        fresh
        patch oreField at 5 5 size 1 1 amount 1000000
        stock plate 30
        place none
        researched smelting
        goal deliver ore 3
        limit 200
    end
"#;

fn ore_line() -> Vec<Command> {
    let mut commands = vec![unit(0, 0, UnitId(0))];
    commands.extend(belts(1..=3, 0));
    commands.push(unit(4, 0, UnitId(1)));
    commands
}

#[test]
fn every_pack_parses_with_titles_and_puzzles() {
    for (name, text) in ALL_PACKS {
        let pack = Pack::parse(text).unwrap_or_else(|e| panic!("{name}: {e}"));
        assert!(!pack.content.title().is_empty() && !pack.content.summary().is_empty(), "{name}");
        assert!(!pack.content.puzzles().is_empty(), "{name} has puzzles");
        // A pack's art is whatever its `sheet` line names, if it has one.
        let sheet = text.lines().find_map(|line| line.strip_prefix("sheet ")).map(str::trim);
        assert_eq!(pack.content.sheet(), sheet, "{name}");
        for (index, puzzle) in pack.content.puzzles().iter().enumerate() {
            assert!(!puzzle.description.is_empty(), "{name}: {} is described", puzzle.id);
            // Every start builds its opening without a refusal.
            let (mut world, starter) = World::start(&pack, Start::Puzzle(index as u16));
            assert_eq!(world.step(&starter), vec![], "{name}: {}", puzzle.id);
            assert_eq!(world.puzzle().unwrap().puzzle, index as u16);
        }
        for start in [Start::Free, Start::Sandbox] {
            let (mut world, starter) = World::start(&pack, start);
            assert_eq!(world.step(&starter), vec![], "{name}");
        }
    }
}

#[test]
fn a_puzzle_is_solved_by_reaching_its_goals() {
    let pack = Pack::parse(PUZZLES).unwrap();
    let (mut world, starter) = World::start(&pack, Start::Puzzle(0));
    assert!(starter.is_empty());
    let other = Command::Place { pos: p(9, 9), dir: Dir::East, building: Building::Unit(UnitId(2)) };
    assert_eq!(world.step(&[other]), vec![Rejected { index: 0, reason: Reject::NotInPuzzle }]);
    run(&mut world, &ore_line(), 1);
    let state = world.puzzle().unwrap();
    assert_eq!((state.progress.clone(), state.solved, state.failed), (vec![(0, 5)], None, false));
    let mut ticks = 0;
    while world.puzzle().unwrap().solved.is_none() {
        world.step(&[]);
        ticks += 1;
        assert!(ticks < 500, "never solved");
    }
    let state = world.puzzle().unwrap();
    assert_eq!(state.progress, vec![(5, 5)]);
    assert_eq!(state.solved, Some(world.tick()));
    // It stays solved, and the factory carries on.
    run(&mut world, &[], 100);
    assert_eq!(world.puzzle().unwrap().solved, state.solved);
    assert!(world.stats(ItemId(0)).delivered > 5);
}

#[test]
fn a_carried_on_puzzle_counts_from_when_it_begins() {
    let pack = Pack::parse(PUZZLES).unwrap();
    let (mut world, _) = World::start(&pack, Start::Free);
    assert_eq!(world.puzzle(), None);
    run(&mut world, &ore_line(), 200);
    let delivered = world.stats(ItemId(0)).delivered;
    assert!(delivered > 5);
    // Beginning it now doesn't count what was already delivered.
    run(&mut world, &[Command::Begin { puzzle: 0 }], 1);
    assert_eq!(world.puzzle().unwrap().progress[0].0, world.stats(ItemId(0)).delivered - delivered);
    assert!(world.puzzle().unwrap().solved.is_none());
    assert_eq!(
        world.step(&[Command::Begin { puzzle: 1 }, Command::Begin { puzzle: 9 }]),
        vec![Rejected { index: 0, reason: Reject::NeedsNewWorld }, Rejected { index: 1, reason: Reject::UnknownPuzzle }]
    );
}

#[test]
fn a_fresh_puzzle_has_its_own_map_and_a_clock() {
    let pack = Pack::parse(PUZZLES).unwrap();
    let (mut world, starter) = World::start(&pack, Start::Puzzle(1));
    assert!(starter.is_empty());
    let ore = pack.content.layer_id("oreField").unwrap();
    assert_eq!(world.layer_value(ore, p(0, 0)), 0);
    assert!(world.layer_value(ore, p(5, 5)) > 0);
    assert_eq!(world.global(PLATE), 30);
    assert!(world.researched(TechId(0)));
    run(&mut world, &[], 199);
    assert!(!world.puzzle().unwrap().failed);
    world.step(&[]);
    let state = world.puzzle().unwrap();
    assert!(state.failed && state.solved.is_none());
    // Once over, it stays over.
    run(&mut world, &[], 50);
    assert!(world.puzzle().unwrap().failed);
}

#[test]
fn a_sandbox_has_everything_unlocked_and_free() {
    let base = Pack::parse(BASE_PACK).unwrap();
    let (mut world, starter) = World::start(&base, Start::Sandbox);
    assert!(world.is_sandbox());
    world.step(&starter);
    let stock = world.global(base.content.resource_id("ironPlate").unwrap());
    let refinery = Building::Unit(base.content.unit_id("refinery").unwrap());
    assert_eq!(world.step(&[Command::Place { pos: p(50, 50), dir: Dir::East, building: refinery }]), vec![]);
    assert_eq!(world.global(base.content.resource_id("ironPlate").unwrap()), stock);
    assert_eq!(world.step(&[Command::Remove { pos: p(50, 50) }]), vec![]);
    let (free, _) = World::start(&base, Start::Free);
    assert!(!free.is_sandbox());
    assert_ne!(world.state_hash(), free.state_hash());
}

#[test]
fn puzzles_and_sandboxes_survive_a_save() {
    let pack = Pack::parse(PUZZLES).unwrap();
    for start in [Start::Puzzle(0), Start::Puzzle(1), Start::Sandbox] {
        let (mut world, starter) = World::start(&pack, start);
        world.step(&starter);
        world.step(&ore_line());
        for _ in 0..120 {
            world.step(&[]);
        }
        let loaded = World::load(&pack, &world.save()).unwrap();
        assert_eq!(loaded.state_hash(), world.state_hash(), "{start:?}");
        assert_eq!(loaded.puzzle(), world.puzzle());
        assert_eq!(loaded.is_sandbox(), world.is_sandbox());
    }
}

#[test]
fn a_puzzle_needs_goals_and_only_a_fresh_one_has_a_map() {
    let base = "resource ore\nlayer oreField\nunit miner\n";
    let cases = [
        ("puzzle a\nend", "has no goals"),
        ("puzzle a\ngoal deliver ore 1\npatch oreField at 0 0 size 1 1 amount 1\nend", "only a 'fresh' puzzle"),
        ("puzzle a\ngoal deliver rock 1\nend", "unknown resource 'rock'"),
        ("puzzle a\ngoal win\nend", "expected 'goal deliver"),
        ("puzzle a\ngoal deliver ore 1\nonly castle\nend", "'castle' isn't a belt or a unit"),
        ("puzzle a\ngoal deliver ore 1\nend\npuzzle a\ngoal deliver ore 1\nend", "declared twice"),
        ("puzzle a\ngoal deliver ore 1\nweather rain\nend", "puzzle does not understand 'weather'"),
    ];
    for (text, message) in cases {
        let error = Pack::parse(&format!("{base}{text}")).unwrap_err();
        assert!(error.message.contains(message), "{text}: {error}");
    }
}
