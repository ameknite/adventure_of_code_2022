use std::{error::Error, fs};

pub fn input() -> Result<String, Box<dyn Error>> {
    Ok(fs::read_to_string("input.txt")?)
}

// Advent of Code 2022
// --- Day 6: Tuning Trouble ---
struct Device {
    datastream: Vec<char>,
    sequence_size: usize,
    characters_until_first_marker: Option<usize>,
}

impl Device {
    fn new(datastream: &str, sequence_size: usize) -> Self {
        Self {
            datastream: datastream.chars().collect(),
            sequence_size,
            characters_until_first_marker: None,
        }
    }
    fn detect_signal(&mut self) {
        self.characters_until_first_marker = self
            .datastream
            .windows(self.sequence_size)
            .enumerate()
            .find(|(_, marker)| {
                let mut v = marker.iter().collect::<Vec<_>>();
                v.sort();
                v.dedup();
                self.sequence_size == v.len()
            })
            .map(|(i, _)| (i + self.sequence_size));
    }
}

pub fn part1(input: &str) -> Option<usize> {
    let mut device = Device::new(input, 4);
    device.detect_signal();
    device.characters_until_first_marker
}

pub fn part2(input: &str) -> Option<usize> {
    let mut device = Device::new(input, 14);
    device.detect_signal();
    device.characters_until_first_marker
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const INPUT2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const INPUT3: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const INPUT4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const INPUT5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn test_part1() {
        let result1 = part1(INPUT1).unwrap();
        let result2 = part1(INPUT2).unwrap();
        let result3 = part1(INPUT3).unwrap();
        let result4 = part1(INPUT4).unwrap();
        let result5 = part1(INPUT5).unwrap();
        assert_eq!(result1, 7);
        assert_eq!(result2, 5);
        assert_eq!(result3, 6);
        assert_eq!(result4, 10);
        assert_eq!(result5, 11);
    }

    #[test]
    fn test_part2() {
        let result1 = part2(INPUT1).unwrap();
        let result2 = part2(INPUT2).unwrap();
        let result3 = part2(INPUT3).unwrap();
        let result4 = part2(INPUT4).unwrap();
        let result5 = part2(INPUT5).unwrap();
        assert_eq!(result1, 19);
        assert_eq!(result2, 23);
        assert_eq!(result3, 23);
        assert_eq!(result4, 29);
        assert_eq!(result5, 26);
    }
}
