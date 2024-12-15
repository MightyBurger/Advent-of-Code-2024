#![allow(unused_variables)]
#![allow(dead_code)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos {
    col: i32,
    row: i32,
}

impl Pos {
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

impl Mul<i32> for Pos {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            col: self.col * rhs,
            row: self.row * rhs,
        }
    }
}

use regex::Regex;

fn process(input: &str) -> i32 {
    let rx = Regex::new(
        r#"Button A: X\+(?<AX>\d+), Y\+(?<AY>\d+)\s+Button B: X\+(?<BX>\d+), Y\+(?<BY>\d+)\s+Prize: X=(?<PX>\d+), Y=(?<PY>\d+)"#,
    ).unwrap();

    let mut sum: i32 = 0;
    for group in input.split("\n\n") {
        let Some(cap) = rx.captures(group) else {
            continue;
        };
        let btn_a = Pos::xy(cap["AX"].parse().unwrap(), cap["AY"].parse().unwrap());
        let btn_b = Pos::xy(cap["BX"].parse().unwrap(), cap["BY"].parse().unwrap());
        let target = Pos::xy(cap["PX"].parse().unwrap(), cap["PY"].parse().unwrap());

        let mut min: Option<i32> = None;
        for a in 0..=100 {
            for b in 0..=100 {
                if btn_a * a + btn_b * b == target {
                    match min {
                        Some(tmin) if 3 * a + 1 * b > tmin => min = Some(3 * a + 1 * b),
                        Some(_) => (),
                        None => min = Some(3 * a + 1 * b),
                    }
                }
            }
        }
        if let Some(min) = min {
            sum += min;
        }
    }
    sum
}

fn main() {
    let input = include_str!("../../../day13/input1.txt");
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
        let check = include_str!("../../../day13/check1.txt");
        assert_eq!(process(check), 480)
    }
}
