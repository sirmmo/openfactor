//! Every timed round can be won, and none is a walkover: each test plays one
//! with a plain layout at a person's pace, waiting whenever it can't afford
//! the next piece.

use openfactor_sim::*;

const TOYS: &str = include_str!("../../../packs/toys.gbx");
const OUTPOST: &str = include_str!("../../../packs/outpost.gbx");
const DINER: &str = include_str!("../../../packs/diner.gbx");

/// Ticks a person takes for one action: placing a building or dragging a row
/// of belts (two seconds).
const PACE: u32 = 120;

/// One action of a plan: `"<unit or belt> x y <E|W|N|S>"`, or `"<unit> x..=x y dir"`
/// for a row of the same thing.
fn actions(pack: &Pack, plan: &[&str]) -> Vec<Vec<Command>> {
    let mut out = Vec::new();
    for step in plan {
        let words: Vec<&str> = step.split_whitespace().collect();
        let [name, xs, y, dir] = words[..] else { panic!("bad step {step:?}") };
        let building = match pack.content.belt_id(name) {
            Some(belt) => Building::Belt(belt),
            None => Building::Unit(pack.content.unit_id(name).unwrap_or_else(|| panic!("no {name}"))),
        };
        let dir = match dir {
            "E" => Dir::East,
            "W" => Dir::West,
            "N" => Dir::North,
            "S" => Dir::South,
            _ => panic!("bad direction in {step:?}"),
        };
        let (from, to) = match xs.split_once("..=") {
            Some((a, b)) => (a.parse().unwrap(), b.parse().unwrap()),
            None => (xs.parse().unwrap(), xs.parse().unwrap()),
        };
        let y: i32 = y.parse().unwrap();
        let xs: Vec<i32> = if from <= to { (from..=to).collect() } else { (to..=from).rev().collect() };
        out.push(xs.into_iter().map(|x| Command::Place { pos: Pos::new(x, y), dir, building }).collect());
    }
    out
}

/// Plays a round and returns the tick it was won and its time limit.
fn play(text: &str, round: &str, plan: &[&str]) -> (Tick, u32) {
    let pack = Pack::parse(text).unwrap();
    let index = pack.content.puzzles().iter().position(|p| p.id == round).unwrap();
    let limit = pack.content.puzzles()[index].limit.expect("a timed round");
    let (mut world, starter) = World::start(&pack, Start::Puzzle(index as u16));
    assert_eq!(world.step(&starter), vec![]);
    let mut queue: std::collections::VecDeque<Command> = Default::default();
    let mut actions = actions(&pack, plan).into_iter();
    let mut idle = 0;
    loop {
        if queue.is_empty() && idle == 0 {
            if let Some(action) = actions.next() {
                queue.extend(action);
                idle = PACE;
            }
        }
        let state = world.puzzle().unwrap();
        if let Some(tick) = state.solved {
            return (tick - state.started, limit);
        }
        assert!(!state.failed, "{round}: time ran out at {:?} with {} pieces left", state.progress, queue.len());
        match queue.front() {
            Some(&command) => match world.step(&[command])[..] {
                [] => {
                    queue.pop_front();
                }
                [Rejected { reason: Reject::CantAfford, .. }] => {}
                ref other => panic!("{round}: {command:?} refused: {other:?}"),
            },
            None => {
                world.step(&[]);
                idle = idle.saturating_sub(1);
            }
        }
    }
}

/// A plain layout at a steady pace wins with time to spare for thinking,
/// but uses a fair part of the clock.
fn assert_fair(round: &str, (won, limit): (Tick, u32)) {
    println!("{round}: won at tick {won} of {limit}");
    let share = won as f64 / f64::from(limit);
    assert!((0.3..0.7).contains(&share), "{round} uses {:.0}% of its time", share * 100.0);
}

#[test]
fn brick_rush_needs_more_than_the_opening_line() {
    // The opening line alone runs out of time.
    let pack = Pack::parse(TOYS).unwrap();
    let index = pack.content.puzzles().iter().position(|p| p.id == "brickRush").unwrap() as u16;
    let (mut world, starter) = World::start(&pack, Start::Puzzle(index));
    world.step(&starter);
    while !world.puzzle().unwrap().failed {
        assert!(world.puzzle().unwrap().solved.is_none(), "the opening line wins it alone");
        world.step(&[]);
    }
    // Moulds down the resin floor, all selling into one shop, do it.
    let mut plan = vec!["shop 5 10 E"];
    let rows: Vec<String> = (3..=9).rev().map(|y| format!("belt 5 {y} S|hopper 3 {y} E|mould 4 {y} E")).collect();
    plan.extend(rows.iter().flat_map(|row| row.split('|')));
    assert_fair("brickRush", play(TOYS, "brickRush", &plan));
}

const RACING_LINE: [&str; 10] = [
    "hopper 3 5 E",
    "shaper 4 5 E",
    "assembler 5 5 E",
    "shop 6 5 E",
    "hopper 3 6 E",
    "belt 4 6 E",
    "press 5 6 N",
    "tap 14 2 S",
    "belt 14 3 S",
    "belt 14..=5 4 W",
];

#[test]
fn pit_stop_can_be_won() {
    let mut plan = RACING_LINE.to_vec();
    plan.push("belt 4 4 S");
    assert_fair("pitStop", play(TOYS, "pitStop", &plan));
}

#[test]
fn gift_wrap_can_be_won() {
    let mut plan = RACING_LINE.to_vec();
    plan.extend([
        "belt 4 4 S",
        "hopper 3 8 E",
        "mould 4 8 E",
        "boxer 5 8 E",
        "shop 6 8 E",
        "hopper 3 9 E",
        "mould 4 9 E",
        "belt 5 9 N",
    ]);
    assert_fair("giftWrap", play(TOYS, "giftWrap", &plan));
}

#[test]
fn dust_storm_can_be_won() {
    let plan = [
        "dock 11 1 E",
        "electrolyser 12 1 W",
        "melter 13 1 W",
        "drill 14 1 W",
        "pylon 12 3 E",
        "panel 12 4 E",
    ];
    assert_fair("dustStorm", play(OUTPOST, "dustStorm", &plan));
}

#[test]
fn saturday_night_can_be_won() {
    let plan = [
        // Burgers: patties from the grill, buns from the oven, chopped veg from below.
        "coldRoom 3 6 E",
        "belt 4..=5 6 E",
        "grill 6 6 E",
        "belt 7 6 E",
        "pass 8 6 E",
        "table 9 6 E",
        "till 10 6 E",
        "shelf 3 4 E",
        "mixer 4 4 E",
        "belt 5 4 E",
        "oven 6 4 E",
        "belt 7 4 E",
        "belt 8 4 S",
        "belt 8 5 S",
        "gasMain 5 5 E",
        "gasPipe 6 5 E",
        "crate 3 8 E",
        "board 4 8 E",
        "belt 5..=7 8 E",
        "belt 8 8 N",
        "belt 8 7 N",
        // Pizza: the bench takes dough, cheese and chopped veg; the oven bakes it.
        "shelf 3 10 E",
        "mixer 4 10 E",
        "belt 5 10 E",
        "bench 6 10 E",
        "oven 7 10 E",
        "table 8 10 E",
        "till 9 10 E",
        "gasMain 8 9 E",
        "gasPipe 7 9 E",
        "fridge 3 11 E",
        "belt 4..=5 11 E",
        "belt 6 11 N",
        "crate 3 9 E",
        "board 4 9 E",
        "belt 5 9 E",
        "belt 6 9 S",
    ];
    assert_fair("saturday", play(DINER, "saturday", &plan));
}
