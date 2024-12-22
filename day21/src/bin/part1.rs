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

trait Button {
    fn pos(&self) -> Vec2;
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum KeypadBtn {
    A,
    N0,
    N1,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
}

impl Button for KeypadBtn {
    fn pos(&self) -> Vec2 {
        match self {
            Self::A => Vec2 { col: 2, row: 3 },
            Self::N0 => Vec2 { col: 1, row: 3 },
            Self::N1 => Vec2 { col: 0, row: 2 },
            Self::N2 => Vec2 { col: 1, row: 2 },
            Self::N3 => Vec2 { col: 2, row: 2 },
            Self::N4 => Vec2 { col: 0, row: 1 },
            Self::N5 => Vec2 { col: 1, row: 1 },
            Self::N6 => Vec2 { col: 2, row: 1 },
            Self::N7 => Vec2 { col: 0, row: 0 },
            Self::N8 => Vec2 { col: 1, row: 0 },
            Self::N9 => Vec2 { col: 2, row: 0 },
        }
    }
}

impl TryFrom<char> for KeypadBtn {
    type Error = &'static str;
    fn try_from(item: char) -> Result<Self, Self::Error> {
        match item {
            'A' => Ok(Self::A),
            '0' => Ok(Self::N0),
            '1' => Ok(Self::N1),
            '2' => Ok(Self::N2),
            '3' => Ok(Self::N3),
            '4' => Ok(Self::N4),
            '5' => Ok(Self::N5),
            '6' => Ok(Self::N6),
            '7' => Ok(Self::N7),
            '8' => Ok(Self::N8),
            '9' => Ok(Self::N9),
            other => Err("could not convert character to keypad btn"),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum DpadBtn {
    A,
    Up,
    Left,
    Down,
    Right,
}

impl Button for DpadBtn {
    fn pos(&self) -> Vec2 {
        match self {
            Self::A => Vec2 { col: 2, row: 0 },
            Self::Up => Vec2 { col: 1, row: 0 },
            Self::Left => Vec2 { col: 0, row: 1 },
            Self::Down => Vec2 { col: 1, row: 1 },
            Self::Right => Vec2 { col: 2, row: 1 },
        }
    }
}

impl TryFrom<char> for DpadBtn {
    type Error = &'static str;
    fn try_from(item: char) -> Result<Self, Self::Error> {
        match item {
            'A' => Ok(Self::A),
            '^' => Ok(Self::Up),
            '<' => Ok(Self::Left),
            'v' => Ok(Self::Down),
            '>' => Ok(Self::Right),
            other => Err("could not convert character to dpad btn"),
        }
    }
}

// Generate the moves necessary to cause the controlled robot to press a sequence of buttons ending
// in A. The resulting move seequence will end in an A.
fn det_cost(
    mut pos: Vec2,
    row_with_void: i32,
    indirections: i32,
    buttons: impl IntoIterator<Item = impl Button + std::fmt::Debug>,
) -> i32 {
    let mut moves = Vec::new();

    enum Order {
        HorizFirst,
        VertFirst,
    }

    for btn in buttons {
        let order = if row_with_void == btn.pos().row {
            Order::HorizFirst
        } else {
            Order::VertFirst
        };

        match order {
            Order::HorizFirst => {
                while btn.pos().col > pos.col {
                    pos.col += 1;
                    moves.push(DpadBtn::Right);
                }
                while btn.pos().col < pos.col {
                    pos.col -= 1;
                    moves.push(DpadBtn::Left);
                }
                while btn.pos().row > pos.row {
                    pos.row += 1;
                    moves.push(DpadBtn::Down);
                }
                while btn.pos().row < pos.row {
                    pos.row -= 1;
                    moves.push(DpadBtn::Up);
                }
            }
            Order::VertFirst => {
                while btn.pos().row > pos.row {
                    pos.row += 1;
                    moves.push(DpadBtn::Down);
                }
                while btn.pos().row < pos.row {
                    pos.row -= 1;
                    moves.push(DpadBtn::Up);
                }
                while btn.pos().col > pos.col {
                    pos.col += 1;
                    moves.push(DpadBtn::Right);
                }
                while btn.pos().col < pos.col {
                    pos.col -= 1;
                    moves.push(DpadBtn::Left);
                }
            }
        }

        moves.push(DpadBtn::A);
    }

    //dbg!(&moves);
    for m in moves.iter() {
        match m {
            DpadBtn::A => print!("A"),
            DpadBtn::Up => print!("^"),
            DpadBtn::Left => print!("<"),
            DpadBtn::Down => print!("v"),
            DpadBtn::Right => print!(">"),
        }
    }
    println!();

    if indirections == 0 {
        moves.len() as i32
    } else {
        // moves.len() as i32 + det_cost(DpadBtn::A.pos(), indirections - 1, moves)
        det_cost(DpadBtn::A.pos(), 1, indirections - 1, moves)
    }
}

fn process(input: &str) -> i32 {
    let indirections = 2;
    input
        .lines()
        .filter(|line| line.len() > 0)
        .map(|line| {
            let numstr: String = line.chars().filter(|c| c.is_numeric()).collect();
            let num: i32 = numstr.parse().unwrap();
            let buttons: Vec<KeypadBtn> = line
                .chars()
                .filter_map(|char| char.try_into().ok())
                .collect();
            println!("Determining the cost of {:?}", buttons);
            let presses = det_cost(KeypadBtn::A.pos(), 0, indirections, buttons.into_iter());
            println!("==================================================================");
            println!("Above calculation was for: {line}");
            println!("Presses required: {presses}");
            println!("Numeric part of the code: {num}");
            println!("Complexity: {}", presses * num);
            presses * num
        })
        .sum()
}

fn main() {
    let input = include_str!("../../../day21/input1.txt");
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
        let check = include_str!("../../../day21/check1.txt");
        assert_eq!(process(check), 126384)
    }
}
