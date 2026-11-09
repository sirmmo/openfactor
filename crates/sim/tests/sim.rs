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
    xs.map(|x| place(x, y, Dir::East, Building::Belt)).collect()
}

fn pack(mine: u32, smelt: u32, assemble: u32, extra: &str) -> Pack {
    let text = format!(
        r#"
        resource ore
        resource plate
        resource widget
        layer oreField
        unit miner placeOn oreField
        unit smelter
        unit assembler
        unit depot stores any
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
        end
        {extra}
        "#
    );
    Pack::parse(&text).unwrap()
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
            held + stats.consumed + stats.destroyed,
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
        place(4, 0, Dir::South, Building::Belt),
        place(4, 2, Dir::North, Building::Belt),
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
        place(1, 0, Dir::East, Building::Belt),
        place(2, 0, Dir::South, Building::Belt),
        place(2, 1, Dir::West, Building::Belt),
        place(1, 1, Dir::North, Building::Belt),
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

    run(&mut world, &[place(5, 0, Dir::East, Building::Belt)], 1000);
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
fn pack_mistakes_name_the_line() {
    let cases: [(&str, usize, &str); 8] = [
        ("resource a\nunitRule r\nappliesTo nobody\na out 1\nend", 2, "unknown unit 'nobody'"),
        ("unit u\nunitRule r\nappliesTo u\nfrobnicate 1\nend", 4, "this rule statement"),
        ("unit u\nunitRule r\nappliesTo u\nlocal ores out 1\nend", 4, "unknown resource 'ores'"),
        ("unit u\nunitRule u\nrate 1\nend", 2, "doesn't take or give"),
        ("resource a\nunit u\nunitRule u\na out 1.5\nend", 4, "whole number"),
        ("unit u\nmapRule grow\nrate 1\nend", 2, "doesn't support yet"),
        ("resource a\nunit u\nunitRule u\na out 1", 3, "never closed"),
        ("unit belt", 1, "built into the engine"),
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
    assert_eq!(pack.content.units()[0].outputs, vec![ItemId(0)]);
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
            0..=54 => Command::Place { pos, dir, building: Building::Belt },
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

fn fuzz_world() -> World {
    World::new(&pack(3, 5, 7, "patch oreField at 0 0 size 14 14 amount 1000000"))
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
