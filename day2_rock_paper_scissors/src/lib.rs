use std::{fs, ops::Add};

pub fn data() -> String {
    fs::read_to_string("input.txt").unwrap().trim().to_string()
}

enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

enum Outcome {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

use Move::*;
use Outcome::*;

impl Move {
    fn new(c: Option<char>) -> Move {
        match c {
            Some('A') | Some('X') => Rock,
            Some('B') | Some('Y') => Paper,
            Some('C') | Some('Z') => Scissors,
            _ => unreachable!(),
        }
    }
}
impl Outcome {
    fn new(c: Option<char>) -> Outcome {
        match c {
            Some('X') => Lose,
            Some('Y') => Draw,
            Some('Z') => Win,
            _ => unreachable!(),
        }
    }
}

impl Add<Outcome> for Move {
    type Output = u32;

    fn add(self, other: Outcome) -> Self::Output {
        self as u32 + other as u32
    }
}
impl Add<Move> for Outcome {
    type Output = u32;

    fn add(self, other: Move) -> Self::Output {
        self as u32 + other as u32
    }
}

pub fn part1(data: &str) -> u32 {
    data.split('\n')
        .map(|round| -> u32 {
            let mut round = round.chars();
            let opponent = Move::new(round.next());
            let you = Move::new(round.nth(1));
            match (opponent, you) {
                (Rock, Rock) => Rock + Draw,
                (Rock, Paper) => Paper + Win,
                (Rock, Scissors) => Scissors + Lose,

                (Paper, Rock) => Rock + Lose,
                (Paper, Paper) => Paper + Draw,
                (Paper, Scissors) => Scissors + Win,

                (Scissors, Rock) => Rock + Win,
                (Scissors, Paper) => Paper + Lose,
                (Scissors, Scissors) => Scissors + Draw,
            }
        })
        .sum()
}

pub fn part2(data: &str) -> u32 {
    data.split('\n')
        .map(|round| -> u32 {
            let mut round = round.chars();
            let opponent = Move::new(round.next());
            let outcome = Outcome::new(round.nth(1));
            match (opponent, outcome) {
                (Rock, Lose) => Scissors + Lose,
                (Rock, Draw) => Rock + Draw,
                (Rock, Win) => Paper + Win,

                (Paper, Lose) => Rock + Lose,
                (Paper, Draw) => Paper + Draw,
                (Paper, Win) => Scissors + Win,

                (Scissors, Lose) => Paper + Lose,
                (Scissors, Draw) => Scissors + Draw,
                (Scissors, Win) => Rock + Win,
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "A Y
B X
C Z";

    #[test]
    fn test_part1() {
        let result = part1(DATA);
        assert_eq!(result, 15);
    }
    #[test]
    fn test_part2() {
        let result = part2(DATA);
        assert_eq!(result, 12);
    }
}
