use std::{error, fs, str::FromStr};

pub fn input() -> String {
    fs::read_to_string("input.txt")
        .unwrap()
        .trim_end()
        .to_string()
}

// Advent of Code 2022
// --- Day 5: Supply Stacks ---

struct Crate {
    id: char,
}

impl TryFrom<char> for Crate {
    type Error = Box<dyn error::Error>;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value.is_alphabetic() {
            true => Ok(Crate { id: value }),
            _ => Err("Error parsing Crate".into()),
        }
    }
}

struct Stack {
    crates: Vec<Crate>,
}

struct Procedure {
    quantity: usize,
    from: usize,
    to: usize,
}

impl FromStr for Procedure {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s;
        let quantity = s
            .chars()
            .take_while(|&c| c != 'f')
            .filter(|c| c.is_numeric())
            .flat_map(|c| c.to_digit(10))
            .fold(0, |acc, x| acc * 10 + x);
        let from = s
            .chars()
            .skip_while(|&c| c != 'f')
            .take_while(|&c| c != 't')
            .filter(|c| c.is_numeric())
            .flat_map(|c| c.to_digit(10))
            .fold(0, |acc, x| acc * 10 + x);
        let to = s
            .chars()
            .skip_while(|&c| c != 't')
            .filter(|c| c.is_numeric())
            .flat_map(|c| c.to_digit(10))
            .fold(0, |acc, x| acc * 10 + x);
        Ok(Procedure {
            quantity: quantity as usize,
            from: from as usize,
            to: to as usize,
        })
    }
}

struct CargoCrane {
    stacks: Vec<Stack>,
    procedures: Vec<Procedure>,
}

impl FromStr for CargoCrane {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (stacks_drawing, procedures_drawing) =
            s.split_once("\n\n").ok_or("Error parsing Drawing")?;

        let procedures = procedures_drawing
            .lines()
            .flat_map(|line| line.parse())
            .collect::<Vec<Procedure>>();

        let mut stacks_lines = stacks_drawing.lines().rev();
        let mut stacks = stacks_lines
            .next()
            .ok_or("Error parsing stacks")?
            .split_whitespace()
            .map(|_| Stack { crates: Vec::new() })
            .collect::<Vec<Stack>>();
        stacks_lines.for_each(|line| {
            line.chars()
                .skip(1)
                .step_by(4)
                .enumerate()
                .for_each(|(i, crate_)| {
                    if let Ok(crate_) = crate_.try_into() {
                        stacks[i].crates.push(crate_);
                    }
                })
        });

        Ok(CargoCrane { stacks, procedures })
    }
}

impl CargoCrane {
    fn result(&self) -> String {
        self.stacks
            .iter()
            .flat_map(|stack| stack.crates.last())
            .map(|crate_| crate_.id)
            .collect()
    }

    fn rearrange_one_at_a_time(&mut self) {
        self.procedures
            .iter()
            .for_each(|&Procedure { quantity, from, to }| {
                (0..quantity).for_each(|_| {
                    if let Some(crate_) = self.stacks[from - 1].crates.pop() {
                        self.stacks[to - 1].crates.push(crate_);
                    }
                })
            });
    }

    fn rearrange_multiple_at_once(&mut self) {
        self.procedures
            .iter()
            .for_each(|&Procedure { quantity, from, to }| {
                let mut vec_temp = Vec::<Crate>::with_capacity(quantity);
                (0..quantity).for_each(|_| {
                    if let Some(crate_) = self.stacks[from - 1].crates.pop() {
                        vec_temp.push(crate_);
                    }
                });
                self.stacks[to - 1]
                    .crates
                    .extend(vec_temp.into_iter().rev());
            });
    }
}

pub fn part1(input: &str) -> Result<String, Box<dyn error::Error>> {
    let mut cargo_crane = input.parse::<CargoCrane>()?;
    cargo_crane.rearrange_one_at_a_time();
    Ok(cargo_crane.result())
}

pub fn part2(input: &str) -> Result<String, Box<dyn error::Error>> {
    let mut cargo_crane = input.parse::<CargoCrane>()?;
    cargo_crane.rearrange_multiple_at_once();
    Ok(cargo_crane.result())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_part1() {
        let result = part1(INPUT).unwrap();
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT).unwrap();
        assert_eq!(result, "MCD");
    }
}
