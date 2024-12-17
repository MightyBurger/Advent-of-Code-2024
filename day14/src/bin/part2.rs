#![allow(unused_variables)]
#![allow(dead_code)]

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
    // x then y
    fn xy(x: i32, y: i32) -> Self {
        Self { col: y, row: x }
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Guard {
    pos: Vec2,
    vel: Vec2,
}

fn printmap(guards: &Vec<Guard>, cols: i32, rows: i32, steps: i32) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    for row in 0..rows {
        for col in 0..cols {
            let count = guards
                .iter()
                .filter(|guard| guard.pos == Vec2::new(col, row))
                .count();
            if count == 0 {
                print!(" ");
            } else {
                //print!("{count}");
                print!("■");
            }
        }
        println!();
    }
    println!("=== Step {steps} ===");
}

fn is_unique(guards: &Vec<Guard>) -> bool {
    for guard in guards.iter() {
        if guards
            .iter()
            .filter(|checkguard| checkguard.pos == guard.pos)
            .count()
            > 1
        {
            return false;
        }
    }
    true
}

use regex::Regex;
use std::time::Duration;

fn process(input: &str, cols: i32, rows: i32) -> i32 {
    let rx = Regex::new(r#"p=(?<PX>\d+),(?<PY>\d+) v=(?<VX>[-\d]+),(?<VY>[-\d]+)"#).unwrap();

    let mut guards: Vec<Guard> = input
        .lines()
        .filter_map(|line| {
            let cap = rx.captures(line)?;
            let pos = Vec2::xy(cap["PX"].parse().unwrap(), cap["PY"].parse().unwrap());
            let vel = Vec2::xy(cap["VX"].parse().unwrap(), cap["VY"].parse().unwrap());
            Some(Guard { pos, vel })
        })
        .collect();

    let mut steps = 0;

    while !is_unique(&guards) {
        steps += 1;
        for guard in guards.iter_mut() {
            guard.pos = guard.pos + guard.vel;
            guard.pos.col = guard.pos.col.rem_euclid(cols);
            guard.pos.row = guard.pos.row.rem_euclid(rows);
        }
    }

    printmap(&guards, cols, rows, steps);

    steps
}

fn main() {
    let input = include_str!("../../../day14/input1.txt");
    let result = process(input, 103, 101);
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
        let check = include_str!("../../../day14/check1.txt");
        assert_eq!(process(check, 11, 7), 12)
    }
}
