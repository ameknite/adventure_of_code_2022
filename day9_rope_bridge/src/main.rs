use std::error::Error;

use day9_rope_bridge::*;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Part 1: {:?}", part1(&input()));
    println!("Part 2: {:?}", part2(&input()));
    Ok(())
}
