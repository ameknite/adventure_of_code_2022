use std::{
    error::{self, Error},
    fs,
    str::FromStr,
};

pub fn input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

// Advent of Code 2022
// --- Day 8: Treetop Tree House ---
#[derive(Debug, Clone)]
struct Tree {
    height: u32,
    visible_from_left: bool,
    visible_from_right: bool,
    trees_visible_left: usize,
    trees_visible_right: usize,
}

impl Tree {
    fn new(height: u32) -> Self {
        Tree {
            height,
            visible_from_left: false,
            visible_from_right: false,
            trees_visible_left: 0,
            trees_visible_right: 0,
        }
    }
}

#[derive(Debug, Clone)]
// Colum or Row
struct List {
    trees: Vec<Tree>,
}

impl List {
    fn check_right_visibility(&mut self) -> Result<(), Box<dyn error::Error>> {
        for i in 0..self.trees.len() {
            let (left_trees, right_trees) = self.trees.split_at_mut(i + 1);
            let tree = left_trees
                .last_mut()
                .ok_or("Error checking right visibility")?;
            if right_trees.is_empty() {
                tree.visible_from_right = true;
            }
            tree.visible_from_right = right_trees.iter().all(|right| right.height < tree.height);
            for right in right_trees.iter() {
                if right.height >= tree.height {
                    tree.trees_visible_right += 1;
                    break;
                } else {
                    tree.trees_visible_right += 1;
                }
            }
        }
        Ok(())
    }

    fn check_left_visibility(&mut self) -> Result<(), Box<dyn error::Error>> {
        for i in 0..self.trees.len() {
            let (left_trees, right_trees) = self.trees.split_at_mut(i);
            let tree = right_trees
                .first_mut()
                .ok_or("Error checking left visibility")?;
            if left_trees.is_empty() {
                tree.visible_from_left = true;
            }

            tree.visible_from_left = left_trees.iter().all(|left| left.height < tree.height);

            for left in left_trees.iter().rev() {
                if left.height >= tree.height {
                    tree.trees_visible_left += 1;
                    break;
                } else {
                    tree.trees_visible_left += 1;
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
struct ForestMap {
    rows: Vec<List>,
    columns: Vec<List>,
}

impl FromStr for ForestMap {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows = s
            .lines()
            .map(|line| {
                line.chars()
                    .flat_map(|c| c.to_digit(10))
                    .map(Tree::new)
                    .collect::<Vec<_>>()
            })
            .map(|trees| List { trees })
            .collect::<Vec<_>>();
        let columns = ForestMap::row_to_column(&rows);
        Ok(ForestMap { rows, columns })
    }
}

impl ForestMap {
    fn row_to_column(rows: &Vec<List>) -> Vec<List> {
        let mut columns = vec![
            List {
                trees: Vec::with_capacity(rows.len())
            };
            rows.len()
        ];
        for row in rows.iter() {
            for (j, tree) in row.trees.iter().enumerate() {
                columns[j].trees.push(tree.clone());
            }
        }
        columns
    }

    fn calculate_visibility(&mut self) -> Result<(), Box<dyn error::Error>> {
        for row in self.rows.iter_mut() {
            row.check_left_visibility()?;
            row.check_right_visibility()?;
        }
        for column in self.columns.iter_mut() {
            column.check_left_visibility()?;
            column.check_right_visibility()?;
        }
        Ok(())
    }

    fn visible_trees(&self) -> u32 {
        let mut total = 0;
        for i in 0..self.rows.len() {
            for (j, tree_row) in self.rows[i].trees.iter().enumerate() {
                let tree_column = &self.columns[j].trees[i];
                if tree_row.visible_from_left
                    || tree_row.visible_from_right
                    || tree_column.visible_from_left
                    || tree_column.visible_from_right
                {
                    total += 1;
                }
            }
        }
        total
    }

    fn highest_scenic_score(&self) -> Option<usize> {
        let mut scores = Vec::with_capacity(self.rows.len() * self.columns.len());
        for i in 0..self.rows.len() {
            for (j, tree_row) in self.rows[i].trees.iter().enumerate() {
                let tree_column = &self.columns[j].trees[i];
                scores.push(
                    tree_column.trees_visible_left
                        * tree_column.trees_visible_right
                        * tree_row.trees_visible_left
                        * tree_row.trees_visible_right,
                );
            }
        }
        scores.iter().max().copied()
    }
}

pub fn part1(input: &str) -> Result<u32, Box<dyn error::Error>> {
    let mut tree_map = input.parse::<ForestMap>()?;
    tree_map.calculate_visibility()?;
    Ok(tree_map.visible_trees())
}

pub fn part2(input: &str) -> Result<Option<usize>, Box<dyn error::Error>> {
    let mut tree_map = input.parse::<ForestMap>()?;
    tree_map.calculate_visibility()?;
    Ok(tree_map.highest_scenic_score())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_part1() {
        let result = part1(INPUT).unwrap();
        assert_eq!(result, 21);
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT).unwrap();
        assert_eq!(result, Some(8));
    }
}
