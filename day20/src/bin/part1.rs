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
    fn offsets4() -> impl Iterator<Item = Self> {
        [
            Self::new(1, 0),
            Self::new(-1, 0),
            Self::new(0, 1),
            Self::new(0, -1),
        ]
        .into_iter()
    }
    fn offsets8() -> impl Iterator<Item = Self> {
        [
            Self::new(1, 0),
            Self::new(1, -1),
            Self::new(0, -1),
            Self::new(-1, -1),
            Self::new(-1, 0),
            Self::new(-1, 1),
            Self::new(0, 1),
            Self::new(1, 1),
        ]
        .into_iter()
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

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Node {
    pos: Vec2,
}

// From a node, find all the next available nodes
fn next(
    walls: &HashSet<Vec2>,
    from: &Node,
    cheat: Option<&(Vec2, Vec2)>,
    cols: i32,
    rows: i32,
) -> Vec<(Node, i32)> {
    let mut next = Vec::new();

    // Direct move
    for offset in Vec2::offsets4() {
        let nextpos = from.pos + offset;
        if !walls.contains(&nextpos) {
            if (0..cols).contains(&nextpos.col) && (0..rows).contains(&nextpos.row) {
                next.push((Node { pos: nextpos }, 1));
            }
        }
    }

    // Shortcut
    if let Some((cheat_start, cheat_end)) = cheat {
        for offset in Vec2::offsets4() {
            if from.pos + offset == *cheat_start {
                next.push((Node { pos: *cheat_end }, 2));
            }
        }
    }

    next
}

use rayon::prelude::*;

fn process(input: &str, must_be_better_than_by: i32) -> i32 {
    let mut walls: HashSet<Vec2> = HashSet::new();
    let mut start: Vec2 = Vec2::default();
    let mut end: Vec2 = Vec2::default();

    let rows = input.lines().count() as i32;
    let cols = input.lines().nth(0).unwrap().chars().count() as i32;

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
    let (_, no_cheat_cost) = dijkstra(
        &Node { pos: start },
        |node| next(&walls, node, None, cols, rows),
        |node| node.pos == end,
    )
    .unwrap();

    let mut cheats: HashSet<(Vec2, Vec2)> = HashSet::new();

    for row in 0..rows {
        for col in 0..cols {
            for offset in Vec2::offsets4() {
                let cheat_start = Vec2::new(col as i32, row as i32);
                let cheat_end = cheat_start + offset;
                if walls.contains(&cheat_start) && !walls.contains(&cheat_end) {
                    cheats.insert((cheat_start, cheat_end));
                }
            }
        }
    }

    println!("Found {} possible cheats. Testing each...", cheats.len());

    let bar = indicatif::ProgressBar::new(cheats.len() as u64);

    cheats
        .par_iter()
        .filter(|cheat| {
            bar.inc(1);
            let (_, cheat_cost) = dijkstra(
                &Node { pos: start },
                |node| next(&walls, node, Some(cheat), cols, rows),
                |node| node.pos == end,
            )
            .unwrap();
            cheat_cost <= no_cheat_cost - must_be_better_than_by
        })
        .count() as i32
}

fn main() {
    let input = include_str!("../../../day20/input1.txt");
    let result = process(input, 100);
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
        let check = include_str!("../../../day20/check1.txt");
        assert_eq!(process(check, 1), 44) // just summed
    }
}
