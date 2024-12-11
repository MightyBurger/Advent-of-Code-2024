#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos {
    col: i32,
    row: i32,
}

use std::ops::{Add, Sub};
impl Add for Pos {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            col: self.col + other.col,
            row: self.row + other.row,
        }
    }
}

impl Sub for Pos {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            col: self.col - other.col,
            row: self.row - other.row,
        }
    }
}

use std::collections::HashSet;
fn explore(grid: &Vec<Vec<u32>>, pos: Pos, marks: &mut HashSet<Pos>) {
    let this_num = grid[pos.col as usize][pos.row as usize];
    if this_num == 9 {
        marks.insert(pos);
        return;
    }
    for offset in [
        Pos { row: 0, col: -1 },
        Pos { row: 0, col: 1 },
        Pos { row: -1, col: 0 },
        Pos { row: 1, col: 0 },
    ]
    .into_iter()
    {
        let next_pos = pos + offset;
        if let Some(next_num) = grid
            .get(next_pos.col as usize)
            .and_then(|v| v.get(next_pos.row as usize))
        {
            if *next_num == this_num + 1 {
                explore(grid, pos + offset, marks);
            }
        }
    }
}
fn process(input: &str) -> i32 {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut sum = 0;
    for col in 0..cols {
        for row in 0..rows {
            if grid[col][row] == 0 {
                let mut marks: HashSet<Pos> = HashSet::new();
                explore(
                    &grid,
                    Pos {
                        row: row as i32,
                        col: col as i32,
                    },
                    &mut marks,
                );
                sum += marks.iter().count() as i32;
            }
        }
    }
    sum
}

fn main() {
    let input = include_str!("../../../day10/input1.txt");
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
        let check = include_str!("../../../day10/check1.txt");
        assert_eq!(process(check), 36)
    }
}
