#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos {
    row: i32,
    col: i32,
}

use std::ops::{Add, Sub};
impl Add for Pos {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }
}

impl Sub for Pos {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            row: self.row - other.row,
            col: self.col - other.col,
        }
    }
}

use itertools::Itertools;
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Tower {
    freq: char,
    pos: Pos,
}
use std::collections::HashSet;
fn process(input: &str) -> i32 {
    let frequencies: HashSet<char> = input.chars().filter(|c| c.is_alphanumeric()).collect();

    let rows = input.lines().count() as i32;
    let cols = input.lines().next().unwrap().chars().count() as i32;

    let mut antinode_pos: HashSet<Pos> = HashSet::new();

    for freq in frequencies {
        let tower_positions: Vec<Pos> = input
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| *c == freq)
                    .map(move |(col, _)| Pos {
                        row: row as i32,
                        col: col as i32,
                    })
            })
            .collect();

        for (a, b) in tower_positions.into_iter().tuple_combinations() {
            let a_to_b = b - a;
            let node1 = b + a_to_b;
            let node2 = a - a_to_b;
            for node in [node1, node2].into_iter() {
                if (0..cols).contains(&node.col) && (0..rows).contains(&node.row) {
                    antinode_pos.insert(node);
                }
            }
        }
    }

    antinode_pos.iter().count() as i32
}

fn main() {
    let input = include_str!("../../../day08/input1.txt");
    let distance = process(input);
    println!("The result is {}", distance);
}

// ----------------------------------------------------
// -------------------- Unit Tests --------------------
// ----------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let check = include_str!("../../../day08/check1.txt");
        assert_eq!(process(check), 14)
    }
}
