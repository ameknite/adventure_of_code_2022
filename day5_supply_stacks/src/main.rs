use std::error;

use day5_supply_stacks::*;

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("Part 1: {}", part1(&input())?);
    println!("Part 2: {}", part2(&input())?);
    Ok(())
}
