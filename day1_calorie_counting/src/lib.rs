use std::fs;

pub fn data() -> String {
    fs::read_to_string("input.txt").unwrap()
}

pub fn part1(data: &str) -> u32 {
    data.split("\n\n")
        .map(|elf| elf.lines().flat_map(|food| food.parse::<u32>()).sum())
        .max()
        .unwrap()
}

pub fn part2(data: &str) -> u32 {
    let mut v = data
        .split("\n\n")
        .map(|elf| elf.lines().flat_map(|food| food.parse::<u32>()).sum())
        .collect::<Vec<_>>();
    v.sort_unstable();
    v.iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_part1() {
        let result = part1(DATA);
        assert_eq!(result, 24000);
    }
    #[test]
    fn test_part2() {
        let result = part2(DATA);
        assert_eq!(result, 45000);
    }
}
