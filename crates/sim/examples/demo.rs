//! Builds rows of miner -> belt -> smelter -> belt -> depot and reports speed.
//! Usage: demo [rows] [ticks]

use openfactor_sim::*;
use std::time::Instant;

const LENGTH: i32 = 20;
const BASE_PACK: &str = include_str!("../../../packs/base.gbx");

fn main() {
    let mut args = std::env::args().skip(1).map(|arg| arg.parse::<i32>().expect("numeric argument"));
    let rows = args.next().unwrap_or(1000);
    let ticks = args.next().unwrap_or(3600) as u32;

    let ore_column = format!("patch oreField at 0 0 size 1 {} amount 1000000", rows * 2);
    let pack = Pack::parse(&format!("{BASE_PACK}\n{ore_column}")).expect("the base pack is valid");
    let unit = |id: &str| Building::Unit(pack.content.unit_id(id).expect("the base pack defines it"));
    let (miner, smelter, depot) = (unit("miner"), unit("smelter"), unit("depot"));
    let plate = pack.content.resource_id("plate").expect("the base pack defines it");
    let mut world = World::new(&pack);

    let at = |x, y, building| Command::Place { pos: Pos::new(x, y), dir: Dir::East, building };
    let mut commands = Vec::new();
    for row in 0..rows {
        let y = row * 2;
        commands.push(at(0, y, miner));
        commands.extend((1..=LENGTH).map(|x| at(x, y, Building::Belt)));
        commands.push(at(LENGTH + 1, y, smelter));
        commands.extend((LENGTH + 2..=2 * LENGTH).map(|x| at(x, y, Building::Belt)));
        commands.push(at(2 * LENGTH + 1, y, depot));
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
