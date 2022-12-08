use std::error::Error;

use day7_no_space_left_on_device::*;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Part 1: {:?}", part1(&input()?));
    println!("Part 2: {:?}", part2(&input()?));
    Ok(())
}
