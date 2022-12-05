use std::{fs, num::ParseIntError, ops::RangeInclusive, str::FromStr};

pub fn data() -> String {
    fs::read_to_string("input.txt").unwrap().trim().to_string()
}

// Advent of Code 2022
// --- Day 4: Camp Cleanup ---
struct Section {
    id: u32,
}

impl FromStr for Section {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Section { id: s.parse()? })
    }
}

struct Assignment {
    start: Section,
    end: Section,
}

impl Assignment {
    fn range(&self) -> RangeInclusive<u32> {
        self.start.id..=self.end.id
    }

    fn one_contains_the_other(&self, other: &Assignment) -> bool {
        self.range().into_iter().all(|x| other.range().contains(&x))
            || other.range().into_iter().all(|x| self.range().contains(&x))
    }
    fn one_overlaps_the_other(&self, other: &Assignment) -> bool {
        self.range().into_iter().any(|x| other.range().contains(&x))
            || other.range().into_iter().any(|x| self.range().contains(&x))
    }
}

impl FromStr for Assignment {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('-').unwrap();
        Ok(Assignment {
            start: start.parse()?,
            end: end.parse()?,
        })
    }
}

struct Pair {
    first: Assignment,
    second: Assignment,
}

impl FromStr for Pair {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a1, a2) = s.split_once(',').unwrap();
        Ok(Pair {
            first: a1.parse()?,
            second: a2.parse()?,
        })
    }
}

pub fn part1(data: &str) -> u32 {
    data.lines()
        .flat_map(|pair| pair.parse::<Pair>())
        .filter(|pair| pair.first.one_contains_the_other(&pair.second))
        .count() as u32
}
pub fn part2(data: &str) -> u32 {
    data.lines()
        .flat_map(|pair| pair.parse::<Pair>())
        .filter(|pair| pair.first.one_overlaps_the_other(&pair.second))
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_part1() {
        let result = part1(DATA);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part2() {
        let result = part2(DATA);
        assert_eq!(result, 4);
    }
}
