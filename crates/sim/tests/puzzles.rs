//! Every puzzle can be solved, and none is a walkover.
//!
//! `solutions/<pack>.sol` holds a plain solution for puzzles of each pack.
//! Every puzzle listed under a `chapter` must have one, and it must be
//! `fresh`, so it stands on its own. The test plays each solution at a
//! person's pace, one action every two seconds, waiting whenever the next
//! action can't be afforded yet or needs research still running:
//!
//! - a timed puzzle's clock must leave time to read and think (at least 1.4
//!   times the solution's time, plus thirty seconds) without being a walkover
//!   (at most 2.5 times, plus a minute);
//! - an untimed one must be won within twenty minutes;
//! - doing nothing must never win.
//!
//! A solution file looks like this:
//!
//! ```text
//! puzzle <id>
//!     <building> <x> <y> <E|W|N|S> [<level>]   # one building
//!     belt 4..=9 2 E                           # a dragged row (or column: 4 2..=9 S)
//!     set <x> <y> <rule|resource|none>         # a unit's recipe, or a sorter's pick
//!     remove <x> <y>
//!     research <tech>
//!     wait <ticks>
//! end
//! ```

use std::collections::{BTreeMap, VecDeque};

use openfactor_sim::*;

/// Ticks a person takes for one action: placing a building, dragging a row
/// of belts, choosing a setting or starting research (two seconds).
const PACE: u32 = 120;
/// How long an untimed puzzle may take, played at that pace.
const UNTIMED: u64 = 20 * 60 * 60;
/// A fair clock for a solution that takes `won` ticks.
fn fair(won: u64) -> std::ops::RangeInclusive<u64> {
    (won * 14 / 10 + 1800)..=(won * 25 / 10 + 3600)
}

/// What the player does: commands sent together, or a pause.
enum Action {
    Commands(Vec<Command>),
    Wait(u32),
}

fn parse_coord(text: &str) -> (i32, i32) {
    let parse = |t: &str| t.parse::<i32>().unwrap_or_else(|_| panic!("bad number {t:?}"));
    match text.split_once("..=") {
        Some((a, b)) => (parse(a), parse(b)),
        None => (parse(text), parse(text)),
    }
}

fn span(from: i32, to: i32) -> Vec<i32> {
    if from <= to {
        (from..=to).collect()
    } else {
        (to..=from).rev().collect()
    }
}

/// Reads a solution file into each puzzle's actions.
fn solutions(pack: &Pack, text: &str) -> BTreeMap<String, Vec<Action>> {
    let content = &pack.content;
    let mut out = BTreeMap::new();
    let mut current: Option<(String, Vec<Action>)> = None;
    // What stands where, so `set` can name a recipe.
    let mut units_at: BTreeMap<(i32, i32, i32), UnitId> = BTreeMap::new();
    for (number, raw) in text.lines().enumerate() {
        let line = raw.split('#').next().unwrap().trim();
        if line.is_empty() {
            continue;
        }
        let at = || format!("solution line {}: {raw:?}", number + 1);
        let words: Vec<&str> = line.split_whitespace().collect();
        match (&mut current, words.as_slice()) {
            (None, ["puzzle", id]) => {
                assert!(content.puzzles().iter().any(|p| p.id == *id), "{}: no puzzle {id}", at());
                units_at.clear();
                current = Some((id.to_string(), Vec::new()));
            }
            (Some(_), ["end"]) => {
                let (id, actions) = current.take().unwrap();
                assert!(out.insert(id.clone(), actions).is_none(), "{}: {id} solved twice", at());
            }
            (Some((_, actions)), ["wait", ticks]) => actions.push(Action::Wait(ticks.parse().expect("ticks"))),
            (Some((_, actions)), ["research", tech]) => {
                let tech = content.tech_id(tech).unwrap_or_else(|| panic!("{}: no research {tech}", at()));
                actions.push(Action::Commands(vec![Command::Research { tech }]));
            }
            (Some((_, actions)), ["remove", x, y]) => {
                let pos = Pos::new(x.parse().unwrap(), y.parse().unwrap());
                actions.push(Action::Commands(vec![Command::Remove { pos }]));
            }
            (Some((_, actions)), ["set", x, y, choice]) => {
                let (x, y): (i32, i32) = (x.parse().unwrap(), y.parse().unwrap());
                let setting = if *choice == "none" {
                    None
                } else if let Some(item) = content.resource_id(choice) {
                    Some(item.0)
                } else {
                    let unit = units_at.get(&(x, y, 0)).unwrap_or_else(|| panic!("{}: nothing placed there", at()));
                    let rules = &content.units()[usize::from(unit.0)].rules;
                    let index = rules.iter().position(|rule| rule.name == *choice);
                    Some(index.unwrap_or_else(|| panic!("{}: no recipe or resource {choice}", at())) as u16)
                };
                actions.push(Action::Commands(vec![Command::Configure { pos: Pos::new(x, y), setting }]));
            }
            (Some((_, actions)), [name, xs, ys, dir, level @ ..]) if level.len() <= 1 => {
                let building = match content.belt_id(name) {
                    Some(belt) => Building::Belt(belt),
                    None => Building::Unit(content.unit_id(name).unwrap_or_else(|| panic!("{}: no building {name}", at()))),
                };
                let dir = match *dir {
                    "E" => Dir::East,
                    "W" => Dir::West,
                    "N" => Dir::North,
                    "S" => Dir::South,
                    _ => panic!("{}: direction is E, W, N or S", at()),
                };
                let z: i32 = level.first().map_or(0, |z| z.parse().expect("level"));
                let ((x0, x1), (y0, y1)) = (parse_coord(xs), parse_coord(ys));
                assert!(x0 == x1 || y0 == y1, "{}: a row or a column, not a block", at());
                let mut commands = Vec::new();
                for x in span(x0, x1) {
                    for y in span(y0, y1) {
                        if let Building::Unit(unit) = building {
                            units_at.insert((x, y, z), unit);
                        }
                        commands.push(Command::Place { pos: Pos::at(x, y, z), dir, building });
                    }
                }
                actions.push(Action::Commands(commands));
            }
            _ => panic!("{}: not understood", at()),
        }
    }
    assert!(current.is_none(), "a solution is missing its 'end'");
    out
}

/// Starts a puzzle, builds its opening, and returns the world.
fn start(pack: &Pack, index: usize) -> World {
    let (mut world, starter) = World::start(pack, Start::Puzzle(index as u16));
    assert_eq!(world.step(&starter), vec![], "the opening builds");
    world
}

fn deadline(pack: &Pack, index: usize) -> u64 {
    pack.content.puzzles()[index].limit.map_or(UNTIMED, u64::from)
}

/// Plays a solution; returns the ticks it took to win, or why it didn't.
fn play(pack: &Pack, index: usize, actions: &[Action]) -> Result<u64, String> {
    let mut world = start(pack, index);
    let begun = world.puzzle().unwrap().started;
    let end = begun + deadline(pack, index);
    let mut actions = actions.iter();
    let mut queue: VecDeque<Command> = VecDeque::new();
    let mut idle = 0;
    loop {
        let state = world.puzzle().unwrap();
        if let Some(tick) = state.solved {
            return Ok(tick - begun);
        }
        if state.failed || world.tick() >= end {
            return Err(format!(
                "not won in time: goals at {:?}, {} actions left",
                state.progress,
                queue.len() + actions.len()
            ));
        }
        if queue.is_empty() && idle == 0 {
            match actions.next() {
                Some(Action::Commands(commands)) => {
                    queue.extend(commands);
                    idle = PACE;
                }
                Some(Action::Wait(ticks)) => idle = *ticks,
                None => {}
            }
        }
        match queue.front() {
            Some(&command) => match world.step(&[command])[..] {
                [] => {
                    queue.pop_front();
                }
                // Not yet: wait for the stock or the research.
                [Rejected {
                    reason: Reject::CantAfford | Reject::Locked | Reject::ResearchBusy | Reject::MissingRequirement,
                    ..
                }] => {}
                [Rejected { reason, .. }] => return Err(format!("{command:?} refused: {reason:?}")),
                _ => unreachable!("one command at a time"),
            },
            None => {
                world.step(&[]);
                idle = idle.saturating_sub(1);
            }
        }
    }
}

/// Doing nothing at all doesn't win.
fn idle_wins(pack: &Pack, index: usize) -> bool {
    let mut world = start(pack, index);
    let end = world.puzzle().unwrap().started + deadline(pack, index).min(UNTIMED);
    while world.tick() < end {
        let state = world.puzzle().unwrap();
        if state.solved.is_some() {
            return true;
        }
        if state.failed {
            return false;
        }
        world.step(&[]);
    }
    false
}

fn check(name: &str, text: &str, solution_text: &str) {
    let pack = Pack::parse(text).unwrap_or_else(|e| panic!("{name}: {e}"));
    let solutions = solutions(&pack, solution_text);
    let mut problems = Vec::new();
    for (index, puzzle) in pack.content.puzzles().iter().enumerate() {
        let Some(actions) = solutions.get(&puzzle.id) else {
            if puzzle.chapter.is_some() {
                problems.push(format!("{}: has a chapter but no solution", puzzle.id));
            }
            continue;
        };
        if puzzle.chapter.is_some() && puzzle.fresh.is_none() {
            problems.push(format!("{}: a chapter puzzle must be 'fresh'", puzzle.id));
            continue;
        }
        if idle_wins(&pack, index) {
            problems.push(format!("{}: doing nothing wins it", puzzle.id));
            continue;
        }
        match (play(&pack, index, actions), puzzle.limit) {
            (Err(why), _) => problems.push(format!("{}: {why}", puzzle.id)),
            (Ok(won), Some(limit)) => {
                println!("{name} {}: won at {won} of {limit}", puzzle.id);
                let fair = fair(won);
                if !fair.contains(&u64::from(limit)) {
                    problems.push(format!(
                        "{}: won at {won}, so the limit should be {} to {}, not {limit}",
                        puzzle.id,
                        fair.start(),
                        fair.end()
                    ));
                }
            }
            (Ok(won), None) => println!("{name} {}: won at {won}, untimed", puzzle.id),
        }
    }
    assert!(problems.is_empty(), "{name}:\n  {}", problems.join("\n  "));
}

macro_rules! pack_test {
    ($test:ident, $pack:literal) => {
        #[test]
        fn $test() {
            check(
                $pack,
                include_str!(concat!("../../../packs/", $pack, ".gbx")),
                include_str!(concat!("solutions/", $pack, ".sol")),
            );
        }
    };
}

pack_test!(factory_puzzles, "base");
pack_test!(tutorial_puzzles, "tutorial");
pack_test!(harvest_puzzles, "farm");
pack_test!(chip_fab_puzzles, "chips");
pack_test!(network_puzzles, "network");
pack_test!(outpost_puzzles, "outpost");
pack_test!(diner_puzzles, "diner");
pack_test!(toy_workshop_puzzles, "toys");
