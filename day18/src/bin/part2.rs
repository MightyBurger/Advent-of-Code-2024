#![allow(unused_variables)]
#![allow(dead_code)]

// -----------------------------------------------------------------------------------
// Vec2 boilerplate (copying and pasting until I bother with putting this in a module)
// -----------------------------------------------------------------------------------

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Vec2 {
    col: i32,
    row: i32,
}

impl Vec2 {
    // col then row
    fn new(col: i32, row: i32) -> Self {
        Self { col, row }
    }
}
use std::ops::{Add, Mul, Sub};
impl Add for Vec2 {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            col: self.col + other.col,
            row: self.row + other.row,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            col: self.col - other.col,
            row: self.row - other.row,
        }
    }
}

impl Mul<i32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            col: self.col * rhs,
            row: self.row * rhs,
        }
    }
}

// -----------------------------------------------------------------------------------
// Code for today's puzzle
// -----------------------------------------------------------------------------------

use itertools::Itertools;
use pathfinding::prelude::dijkstra;
use std::collections::HashSet;

fn next_steps(from: &Vec2, walls: &HashSet<Vec2>, cols: i32, rows: i32) -> Vec<(Vec2, i32)> {
    let step_cost = 1;

    [
        Vec2::new(0, 1),
        Vec2::new(0, -1),
        Vec2::new(1, 0),
        Vec2::new(-1, 0),
    ]
    .into_iter()
    .filter_map(|offset| {
        let next = *from + offset;
        if walls.contains(&next) {
            None
        } else if next.col < 0 || next.col >= cols || next.row < 0 || next.row >= rows {
            None
        } else {
            Some((next, step_cost))
        }
    })
    .collect()
}

fn print_map(bytes: &HashSet<Vec2>, cols: i32, rows: i32) {
    for row in 0..rows {
        for col in 0..cols {
            if bytes.contains(&Vec2::new(col, row)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn process(input: &str, cols: i32, rows: i32) -> Vec2 {
    let start = Vec2::new(0, 0);
    let end = Vec2::new(cols - 1, rows - 1);
    let mut bytes: HashSet<Vec2> = HashSet::new();

    for (i, line) in input.lines().enumerate() {
        println!("testing line {i}");
        let (left, right) = line.split(",").tuples().next().unwrap();
        let this_pos = Vec2::new(left.parse().unwrap(), right.parse().unwrap());
        bytes.insert(this_pos);

        if dijkstra(
            &start,
            |node| next_steps(node, &bytes, cols, rows),
            |node| *node == end,
        )
        .is_none()
        {
            return this_pos;
        }
    }
    panic!("always found a path...");
}

fn main() {
    let input = include_str!("../../../day18/input1.txt");
    let result = process(input, 71, 71);
    println!("The result is {:?}", result);
}

// ----------------------------------------------------
// -------------------- Unit Tests --------------------
// ----------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let check = include_str!("../../../day18/check1.txt");
        assert_eq!(process(check, 7, 7), Vec2::new(6, 1))
    }
}
