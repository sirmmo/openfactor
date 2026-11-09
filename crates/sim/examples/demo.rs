//! Builds rows of miner -> belt -> smelter -> belt -> depot and reports speed.
//! Usage: demo [rows] [ticks]

use openfactor_sim::*;
use std::time::Instant;

const LENGTH: i32 = 20;

fn main() {
    let mut args = std::env::args().skip(1).map(|arg| arg.parse::<i32>().expect("numeric argument"));
    let rows = args.next().unwrap_or(1000);
    let ticks = args.next().unwrap_or(3600) as u32;

    let ore = ItemId(0);
    let plate = ItemId(1);
    let patches = vec![Patch {
        origin: Pos::new(0, 0),
        width: 1,
        height: rows as u32 * 2,
        item: ore,
        amount_per_tile: 1_000_000,
    }];
    let mut world = World::new(Content::example(), &WorldGen { patches }).unwrap();

    let at = |x, y, building| Command::Place { pos: Pos::new(x, y), dir: Dir::East, building };
    let mut commands = Vec::new();
    for row in 0..rows {
        let y = row * 2;
        commands.push(at(0, y, Building::Miner));
        commands.extend((1..=LENGTH).map(|x| at(x, y, Building::Belt)));
        commands.push(at(LENGTH + 1, y, Building::Assembler { recipe: RecipeId(0) }));
        commands.extend((LENGTH + 2..=2 * LENGTH).map(|x| at(x, y, Building::Belt)));
        commands.push(at(2 * LENGTH + 1, y, Building::Depot));
    }

    assert!(world.step(&commands).is_empty());
    let started = Instant::now();
    for _ in 1..ticks {
        world.step(&[]);
    }
    let elapsed = started.elapsed().as_secs_f64();

    println!("rows:            {rows}");
    println!("belt lines:      {}", world.line_count());
    println!("items on belts:  {}", world.belt_item_count());
    println!("plates made:     {}", world.stats(plate).produced);
    println!("final tick:      {}", world.tick());
    println!("state hash:      {:016x}", world.state_hash());
    println!("ticks/second:    {:.0}", f64::from(ticks - 1) / elapsed);
}
