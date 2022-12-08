use std::error;

use day8_treetop_tree_house::*;

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("Part 1: {:?}", part1(&input())?);
    println!("Part 2: {:?}", part2(&input())?);
    Ok(())
}
