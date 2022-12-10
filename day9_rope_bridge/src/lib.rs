use std::{
    collections::HashSet,
    error::{self, Error},
    fmt::Display,
    fs,
    str::FromStr,
};

pub fn input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

// Advent of Code 2022
// --- Day 9: Rope Bridge ---
#[derive(Debug)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug)]
struct Motion {
    direction: Direction,
    steps: i32,
}

impl FromStr for Motion {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, value) = s.split_once(' ').ok_or("Error parsing motion")?;
        Ok(match name {
            "R" => Motion {
                direction: Direction::Right,
                steps: value.parse()?,
            },
            "L" => Motion {
                direction: Direction::Left,
                steps: value.parse()?,
            },
            "U" => Motion {
                direction: Direction::Up,
                steps: value.parse()?,
            },
            "D" => Motion {
                direction: Direction::Down,
                steps: value.parse()?,
            },
            _ => panic!("Error parsing motion"),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn one_move(&mut self, direction: &Direction) {
        match direction {
            Direction::Right => {
                self.x += 1;
            }
            Direction::Left => {
                self.x -= 1;
            }
            Direction::Up => {
                self.y += 1;
            }
            Direction::Down => {
                self.y -= 1;
            }
        }
    }
}

impl Position {
    fn origin() -> Self {
        Self { x: 0, y: 0 }
    }
}

#[derive(Debug)]
struct Rope {
    head: Position,
    knots: Vec<Position>,
}

impl Rope {
    fn with_knots(knots: usize) -> Self {
        Self {
            head: Position::origin(),
            knots: vec![Position::origin(); knots],
        }
    }

    fn move_knots(&mut self, direction: &Direction, positions: &mut HashSet<Position>) {
        self.move_head(direction);
        for i in 0..self.knots.len() {
            let (left, right) = self.knots.split_at_mut(i);
            let head = left.last_mut().unwrap_or(&mut self.head);
            let knot = right.first_mut().unwrap();
            Rope::knot_follows_head(head, knot, direction);

            // Tail positions
            if i == self.knots.len() - 1 {
                positions.insert(self.knots.last().unwrap().clone());
            }
        }
    }

    fn move_head(&mut self, direction: &Direction) {
        self.head.one_move(direction);
    }

    fn knot_follows_head(head: &Position, knot: &mut Position, direction: &Direction) {
        if !Rope::is_knot_adjacent(head, knot) {
            match direction {
                Direction::Right | Direction::Left => {
                    if Rope::is_knot_in_the_same_row(head, knot) {
                        knot.one_move(direction);
                    } else {
                        let head_direction = Rope::head_y_direction(head, knot);
                        knot.one_move(&head_direction);
                        if !Rope::is_knot_in_the_same_column(head, knot) {
                            let head_direction = Rope::head_x_direction(head, knot);
                            knot.one_move(&head_direction);
                        }
                    }
                }
                Direction::Up | Direction::Down => {
                    if Rope::is_knot_in_the_same_column(head, knot) {
                        knot.one_move(direction);
                    } else {
                        let head_direction = Rope::head_x_direction(head, knot);
                        knot.one_move(&head_direction);
                        if !Rope::is_knot_in_the_same_row(head, knot) {
                            let head_direction = Rope::head_y_direction(head, knot);
                            knot.one_move(&head_direction);
                        }
                    }
                }
            }
        }
    }

    fn is_knot_adjacent(head: &Position, knot: &Position) -> bool {
        let x_adjacent = (head.x - knot.x).abs() <= 1;
        let y_adjacent = (head.y - knot.y).abs() <= 1;
        x_adjacent && y_adjacent
    }

    fn head_x_direction(head: &Position, knot: &Position) -> Direction {
        match (head.x - knot.x).signum() {
            1 => Direction::Right,
            -1 => Direction::Left,
            _ => unreachable!(),
        }
    }

    fn head_y_direction(head: &Position, knot: &Position) -> Direction {
        match (head.y - knot.y).signum() {
            1 => Direction::Up,
            -1 => Direction::Down,
            _ => unreachable!(),
        }
    }

    fn is_knot_in_the_same_column(head: &Position, knot: &Position) -> bool {
        head.x == knot.x
    }
    fn is_knot_in_the_same_row(head: &Position, knot: &Position) -> bool {
        head.y == knot.y
    }
}

fn print_coordinates(positions: &HashSet<Position>) {
    let max_x = positions.iter().max_by_key(|p| p.x).unwrap();
    let max_y = positions.iter().max_by_key(|p| p.y).unwrap();
    let min_x = positions.iter().min_by_key(|p| p.x).unwrap();
    let min_y = positions.iter().min_by_key(|p| p.y).unwrap();
    let mut s = String::new();
    for y in min_y.y..=max_y.y {
        for x in min_x.x..=max_x.x {
            let position = Position { x, y };
            if position == Position::origin() {
                s.push('s')
            } else if positions.contains(&position) {
                s.push('#');
            } else {
                s.push('.');
            }
        }
        s.push('\n');
    }
    for line in s.lines().rev() {
        println!("{line}");
    }
}

pub fn part1(input: &str) -> usize {
    let mut rope = Rope::with_knots(1);
    let motions = input
        .lines()
        .flat_map(|line| line.parse())
        .collect::<Vec<Motion>>();
    let mut unique_tail_positions = HashSet::<Position>::new();
    motions.iter().for_each(|motion| {
        for _ in 0..motion.steps {
            rope.move_knots(&motion.direction, &mut unique_tail_positions);
        }
    });
    print_coordinates(&unique_tail_positions);
    unique_tail_positions.len()
}

pub fn part2(input: &str) -> usize {
    let mut rope = Rope::with_knots(9);
    let motions = input
        .lines()
        .flat_map(|line| line.parse())
        .collect::<Vec<Motion>>();
    let mut unique_tail_positions = HashSet::<Position>::new();
    motions.iter().for_each(|motion| {
        for _ in 0..motion.steps {
            rope.move_knots(&motion.direction, &mut unique_tail_positions);
        }
    });

    print_coordinates(&unique_tail_positions);
    unique_tail_positions.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const INPUT2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn test_part1() {
        let result = part1(INPUT);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT2);
        assert_eq!(result, 36);
    }
}
