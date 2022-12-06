use std::error::Error;

use day6_tuning_trouble::*;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Part 1: {:?}", part1(&input()?));
    println!("Part 2: {:?}", part2(&input()?));
    Ok(())
}
