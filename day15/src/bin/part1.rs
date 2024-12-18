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

use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn offset(&self) -> Vec2 {
        match self {
            Self::Up => Vec2::new(0, -1),
            Self::Down => Vec2::new(0, 1),
            Self::Left => Vec2::new(-1, 0),
            Self::Right => Vec2::new(1, 0),
        }
    }
}

impl TryFrom<char> for Dir {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Self::Up),
            'v' => Ok(Self::Down),
            '<' => Ok(Self::Left),
            '>' => Ok(Self::Right),
            _ => Err(()),
        }
    }
}

fn process(input: &str) -> i32 {
    let mut input_iter = input.split("\n\n");
    let input1 = input_iter.next().unwrap();
    let input2 = input_iter.next().unwrap();

    let mut walls: HashSet<Vec2> = HashSet::new();
    let mut boxes: HashSet<Vec2> = HashSet::new();
    let mut player = Vec2::default();
    for (row, line) in input1.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            let pos = Vec2::new(col as i32, row as i32);
            match c {
                '#' => {
                    walls.insert(pos);
                }
                'O' => {
                    boxes.insert(pos);
                }
                '@' => player = pos,
                '.' => (),
                other => println!(
                    "Warning: encountered unexpected character {} at {:?}",
                    other, pos
                ),
            }
        }
    }

    // Shadowing to remove mutability. The walls should never change. Cool
    let walls = walls;

    let moves: Vec<Dir> = input2
        .chars()
        .filter_map(|c| Dir::try_from(c).ok())
        .collect();

    for dir in moves.iter() {
        let offset = dir.offset();
        let mut how_far_ahead = 1;
        // Keep looking until we get past the boxes.
        while boxes.contains(&(player + offset * how_far_ahead)) {
            how_far_ahead += 1;
        }
        // Did we hit a wall, or is there room?
        if walls.contains(&(player + offset * how_far_ahead)) {
            continue;
        }
        // There's room. It's an empty spot at player + offset * how_far_ahead.
        boxes.insert(player + offset * how_far_ahead);
        boxes.remove(&(player + offset));
        player = player + offset;
    }

    boxes.iter().map(|bx| 100 * bx.row + bx.col).sum()
}

fn main() {
    let input = include_str!("../../../day15/input1.txt");
    let result = process(input);
    println!("The result is {}", result);
}

// ----------------------------------------------------
// -------------------- Unit Tests --------------------
// ----------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let check = include_str!("../../../day15/check1.txt");
        assert_eq!(process(check), 10092)
    }
}
