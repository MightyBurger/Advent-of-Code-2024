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

use pathfinding::prelude::*;
use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Dir {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum RotDir {
    CW,
    CCW,
}

impl Dir {
    fn offset(&self) -> Vec2 {
        match self {
            Self::North => Vec2::new(0, -1),
            Self::East => Vec2::new(1, 0),
            Self::South => Vec2::new(0, 1),
            Self::West => Vec2::new(-1, 0),
        }
    }
    #[must_use]
    fn rotate(&self, rot: RotDir) -> Self {
        match (rot, self) {
            (RotDir::CW, Dir::North) => Dir::East,
            (RotDir::CW, Dir::East) => Dir::South,
            (RotDir::CW, Dir::South) => Dir::West,
            (RotDir::CW, Dir::West) => Dir::North,
            (RotDir::CCW, Dir::North) => Dir::West,
            (RotDir::CCW, Dir::East) => Dir::North,
            (RotDir::CCW, Dir::South) => Dir::East,
            (RotDir::CCW, Dir::West) => Dir::South,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Node {
    pos: Vec2,
    dir: Dir,
}

// From a node, find all the next available nodes
fn next(walls: &HashSet<Vec2>, from: Node) -> Vec<(Node, i32)> {
    let mut next = Vec::with_capacity(3);
    // You can turn CW...
    next.push((
        Node {
            dir: from.dir.rotate(RotDir::CW),
            ..from
        },
        1000,
    ));
    // CCW...
    next.push((
        Node {
            dir: from.dir.rotate(RotDir::CCW),
            ..from
        },
        1000,
    ));
    // Or you can move forward.
    // TODO: This is not efficient. You can do better.
    if !walls.contains(&(from.pos + from.dir.offset())) {
        next.push((
            Node {
                pos: from.pos + from.dir.offset(),
                ..from
            },
            1,
        ));
    }
    next
}

fn process(input: &str) -> i32 {
    let mut walls: HashSet<Vec2> = HashSet::new();
    let mut start: Vec2 = Vec2::default();
    let mut end: Vec2 = Vec2::default();

    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            let pos = Vec2::new(col as i32, row as i32);
            match c {
                '#' => {
                    walls.insert(pos);
                }
                'S' => start = pos,
                'E' => end = pos,
                _ => (),
            }
        }
    }
    let result = dijkstra(
        &Node {
            pos: start,
            dir: Dir::East,
        },
        |node| next(&walls, *node),
        |node| node.pos == end,
    );

    let (path, cost) = result.expect("The maze should have a solution...");

    for row in 0..input.lines().count() {
        for col in 0..input.lines().next().unwrap().chars().count() {
            let pos = Vec2::new(col as i32, row as i32);
            if pos == start {
                print!("S");
            } else if pos == end {
                print!("E");
            } else if walls.contains(&pos) {
                print!("#");
            } else if let Some(node) = path.iter().find(|node| node.pos == pos) {
                match node.dir {
                    Dir::North => print!("^"),
                    Dir::East => print!(">"),
                    Dir::South => print!("v"),
                    Dir::West => print!("<"),
                }
            } else {
                print!(" ");
            }
        }
        println!();
    }

    cost
}

fn main() {
    let input = include_str!("../../../day16/input1.txt");
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
        let check = include_str!("../../../day16/check1.txt");
        assert_eq!(process(check), 7036)
    }
}
