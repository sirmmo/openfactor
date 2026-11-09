use openfactor_sim::*;

const ORE: ItemId = ItemId(0);
const PLATE: ItemId = ItemId(1);
const SMELT: RecipeId = RecipeId(0);

fn p(x: i32, y: i32) -> Pos {
    Pos::new(x, y)
}

fn place(x: i32, y: i32, dir: Dir, building: Building) -> Command {
    Command::Place { pos: p(x, y), dir, building }
}

fn belts(xs: std::ops::RangeInclusive<i32>, y: i32) -> Vec<Command> {
    xs.map(|x| place(x, y, Dir::East, Building::Belt)).collect()
}

fn patch(x: i32, y: i32, width: u32, height: u32, item: ItemId) -> Patch {
    Patch { origin: p(x, y), width, height, item, amount_per_tile: 1_000_000 }
}

fn world_with(content: Content, patches: Vec<Patch>) -> World {
    World::new(content, &WorldGen { patches }).unwrap()
}

fn ore_world() -> World {
    world_with(Content::example(), vec![patch(0, 0, 1, 1, ORE)])
}

fn run(world: &mut World, commands: &[Command], ticks: u32) {
    assert_eq!(world.step(commands), vec![]);
    for _ in 1..ticks {
        world.step(&[]);
    }
}

/// Every item ever created must be in the world, consumed or recorded destroyed.
fn assert_conserved(world: &World, items: usize) {
    let totals = world.item_totals();
    for index in 0..items {
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
    let mut commands = vec![place(0, 0, Dir::East, Building::Miner)];
    commands.extend(belts(1..=5, 0));
    commands.push(place(6, 0, Dir::East, Building::Assembler { recipe: SMELT }));
    commands.extend(belts(7..=10, 0));
    commands.push(place(11, 0, Dir::East, Building::Depot));

    run(&mut world, &commands, 6000);

    // Mining and smelting both take 30 ticks, so steady state is one plate per
    // ~30 ticks; allow for the time the first item spends in transit.
    let delivered = world.depot_stock(p(11, 0), PLATE);
    assert!((180..=200).contains(&delivered), "delivered {delivered}");
    assert_conserved(&world, 3);
}

#[test]
fn belt_fills_to_capacity_then_drains_at_full_throughput() {
    let mut content = Content::example();
    content.miner_ticks = 1;
    let mut world = world_with(content, vec![patch(0, 0, 1, 1, ORE)]);

    let mut commands = vec![place(0, 0, Dir::East, Building::Miner)];
    commands.extend(belts(1..=8, 0));
    run(&mut world, &commands, 2000);

    // 8 tiles of 256 at spacing 64, counting both end positions.
    assert_eq!(world.belt_item_count(), 33);
    assert_conserved(&world, 3);

    run(&mut world, &[place(9, 0, Dir::East, Building::Depot)], 400);
    let first = world.depot_stock(p(9, 0), ORE);
    for _ in 0..400 {
        world.step(&[]);
    }
    assert_eq!(world.depot_stock(p(9, 0), ORE) - first, 100, "one item per 4 ticks");
}

#[test]
fn junction_alternates_between_saturated_feeders() {
    let mut content = Content::example();
    content.items.push("ore2".into());
    content.miner_ticks = 1;
    let other = ItemId(3);
    let mut world =
        world_with(content, vec![patch(0, 0, 1, 1, ORE), patch(0, 2, 1, 1, other)]);

    let mut commands = vec![
        place(0, 0, Dir::East, Building::Miner),
        place(0, 2, Dir::East, Building::Miner),
        place(4, 0, Dir::South, Building::Belt),
        place(4, 2, Dir::North, Building::Belt),
        place(9, 1, Dir::East, Building::Depot),
    ];
    commands.extend(belts(1..=3, 0));
    commands.extend(belts(1..=3, 2));
    commands.extend(belts(4..=8, 1));
    run(&mut world, &commands, 4000);

    let a = world.depot_stock(p(9, 1), ORE);
    let b = world.depot_stock(p(9, 1), other);
    assert!(a > 400, "a={a}");
    assert!(a.abs_diff(b) <= 2, "a={a} b={b}");
    assert_conserved(&world, 4);
}

#[test]
fn closed_ring_circulates_without_losing_items() {
    let mut world = ore_world();
    let ring = vec![
        place(1, 0, Dir::East, Building::Belt),
        place(2, 0, Dir::South, Building::Belt),
        place(2, 1, Dir::West, Building::Belt),
        place(1, 1, Dir::North, Building::Belt),
    ];
    let mut commands = ring;
    commands.push(place(0, 0, Dir::East, Building::Miner));
    run(&mut world, &commands, 200);

    let count = world.belt_item_count();
    assert!(count > 0);
    assert_eq!(world.line_count(), 1);
    run(&mut world, &[Command::Remove { pos: p(0, 0) }], 2000);
    assert_eq!(world.belt_item_count(), count);
    assert_conserved(&world, 3);
}

#[test]
fn editing_a_belt_keeps_items_on_surviving_tiles() {
    let mut content = Content::example();
    content.miner_ticks = 1;
    let mut world = world_with(content, vec![patch(0, 0, 1, 1, ORE)]);
    let mut commands = vec![place(0, 0, Dir::East, Building::Miner)];
    commands.extend(belts(1..=8, 0));
    run(&mut world, &commands, 2000);
    assert_eq!(world.line_count(), 1);

    run(&mut world, &[Command::Remove { pos: p(5, 0) }], 1);
    assert_eq!(world.line_count(), 2);
    assert!(world.stats(ORE).destroyed >= 4, "the removed tile's items are gone");
    assert_conserved(&world, 3);

    run(&mut world, &[place(5, 0, Dir::East, Building::Belt)], 1000);
    assert_eq!(world.line_count(), 1);
    assert_conserved(&world, 3);
}

#[test]
fn invalid_commands_are_rejected_without_effect() {
    let mut world = ore_world();
    let before = world.state_hash();
    let commands = [
        place(5, 5, Dir::East, Building::Miner),
        place(5, 5, Dir::East, Building::Assembler { recipe: RecipeId(99) }),
        Command::Remove { pos: p(5, 5) },
    ];
    let reasons: Vec<Reject> = world.step(&commands).iter().map(|r| r.reason).collect();
    assert_eq!(reasons, [Reject::NoResource, Reject::UnknownRecipe, Reject::NothingToRemove]);
    assert_ne!(before, world.state_hash(), "the tick still advances");
    assert_eq!(world.line_count(), 0);
}

#[test]
fn render_views_match_the_world() {
    let mut world = ore_world();
    let mut commands = vec![place(0, 0, Dir::East, Building::Miner)];
    commands.extend(belts(1..=3, 0));
    run(&mut world, &commands, 200);

    assert_eq!(world.buildings().count(), 4);
    assert_eq!(world.resources().count(), 1);
    assert_eq!(world.belt_items().count(), world.belt_item_count());
    for (pos, dir, offset, item) in world.belt_items() {
        assert!((1..=3).contains(&pos.x) && pos.y == 0, "{pos:?}");
        assert_eq!((dir, item), (Dir::East, ORE));
        assert!(offset <= TILE);
    }
}

#[test]
fn invalid_content_is_refused() {
    let mut content = Content::example();
    content.recipes[0].inputs[0].0 = ItemId(42);
    let error = World::new(content, &WorldGen::default()).unwrap_err();
    assert_eq!(error, ContentError::UnknownItem { item: ItemId(42) });
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
        commands.push(match roll {
            0..=54 => Command::Place { pos, dir, building: Building::Belt },
            55..=64 => Command::Place { pos, dir, building: Building::Miner },
            65..=72 => Command::Place {
                pos,
                dir,
                building: Building::Assembler { recipe: RecipeId(rng.below(2) as u16) },
            },
            73..=77 => Command::Place { pos, dir, building: Building::Depot },
            _ => Command::Remove { pos },
        });
    }
    commands
}

fn fuzz_world() -> World {
    let mut content = Content::example();
    content.miner_ticks = 3;
    content.recipes[0].ticks = 5;
    content.recipes[1].ticks = 7;
    world_with(content, vec![patch(0, 0, 14, 14, ORE)])
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
            assert_conserved(&first, 3);
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
