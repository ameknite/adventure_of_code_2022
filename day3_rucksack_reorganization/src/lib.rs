use std::fs;

pub fn data() -> String {
    fs::read_to_string("input.txt").unwrap().trim().to_string()
}

// --- Day 3: Rucksack Reorganization ---

pub fn part1(data: &str) -> u32 {
    data.split_whitespace()
        .flat_map(|rucksack| {
            let (first, second) = rucksack.split_at(rucksack.len() / 2);
            first.chars().find(|&c| second.contains(c)).map(|c| -> u32 {
                match c {
                    'a'..='z' => u32::from(c) - u32::from(b'a') + 1,
                    'A'..='Z' => u32::from(c) - u32::from(b'A') + 27,
                    _ => 0,
                }
            })
        })
        .sum()
}

pub fn part2(data: &str) -> u32 {
    let v = data.split_whitespace().collect::<Vec<_>>();
    v.chunks(3)
        .flat_map(|group| {
            let (first, second, third) = (group[0], group[1], group[2]);
            first
                .chars()
                .find(|&c| second.contains(c) && third.contains(c))
                .map(|c| match c {
                    'a'..='z' => u32::from(c) - u32::from(b'a') + 1,
                    'A'..='Z' => u32::from(c) - u32::from(b'A') + 27,
                    _ => 0,
                })
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part1() {
        let result = part1(DATA);
        assert_eq!(result, 157);
    }

    #[test]
    fn test_part2() {
        let result = part2(DATA);
        assert_eq!(result, 70);
    }
}
