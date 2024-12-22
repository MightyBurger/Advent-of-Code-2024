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
    const NULLSPACE: Vec2;
    fn pos(&self) -> Vec2;
    fn from_pos(pos: Vec2) -> Self;
    fn apos() -> Vec2;
    fn optimal_path_dpad(&self, from: Vec2) -> impl Iterator<Item = DpadBtn> {
        let dest = self.pos();

        let mut current_pos = from;
        let mut path: Vec<DpadBtn> = Vec::new();

        while current_pos != dest {
            // Prefer left first.
            if dest.col < current_pos.col && current_pos + Vec2::new(-1, 0) != Self::NULLSPACE {
                current_pos.col -= 1;
                path.push(DpadBtn::Left);
            }
            // Prefer down next.
            else if dest.row > current_pos.row && current_pos + Vec2::new(0, 1) != Self::NULLSPACE
            {
                current_pos.row += 1;
                path.push(DpadBtn::Down);
            }
            // Prefer either up or right; shouldn't matter.
            else if dest.col > current_pos.col && current_pos + Vec2::new(1, 0) != Self::NULLSPACE
            {
                current_pos.col += 1;
                path.push(DpadBtn::Right);
            } else if dest.row < current_pos.row
                && current_pos + Vec2::new(0, -1) != Self::NULLSPACE
            {
                current_pos.row -= 1;
                path.push(DpadBtn::Up);
            }
        }
        path.push(DpadBtn::A);
        path.into_iter()
    }
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
    const NULLSPACE: Vec2 = Vec2 { col: 0, row: 3 };
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
    fn from_pos(pos: Vec2) -> Self {
        use KeypadBtn::*;
        match (pos.col, pos.row) {
            (2, 3) => A,
            (1, 3) => N0,
            (0, 2) => N1,
            (1, 2) => N2,
            (2, 2) => N3,
            (0, 1) => N4,
            (1, 1) => N5,
            (2, 1) => N6,
            (0, 0) => N7,
            (1, 0) => N8,
            (2, 0) => N9,
            _ => panic!("???"),
        }
    }
    fn apos() -> Vec2 {
        Self::A.pos()
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
    const NULLSPACE: Vec2 = Vec2 { col: 0, row: 3 };
    fn pos(&self) -> Vec2 {
        match self {
            Self::A => Vec2 { col: 2, row: 0 },
            Self::Up => Vec2 { col: 1, row: 0 },
            Self::Left => Vec2 { col: 0, row: 1 },
            Self::Down => Vec2 { col: 1, row: 1 },
            Self::Right => Vec2 { col: 2, row: 1 },
        }
    }
    fn from_pos(pos: Vec2) -> Self {
        use DpadBtn::*;
        match (pos.col, pos.row) {
            (2, 0) => A,
            (1, 0) => Up,
            (0, 1) => Left,
            (1, 1) => Down,
            (2, 1) => Right,
            _ => panic!("???"),
        }
    }
    fn apos() -> Vec2 {
        Self::A.pos()
    }
    // fn optimal_path_dpad(&self, from: Vec2) -> impl Iterator<Item = DpadBtn> {
    //     let from = Self::from_pos(from);
    //     use DpadBtn::*;
    //     let path = match (from, self) {
    //         (A, A) => vec![A],
    //         (A, Up) => vec![Left, A],
    //         (A, Left) => vec![Left, Down, Left, A],
    //         (A, Down) => vec![Left, Down, A],
    //         (A, Right) => vec![Down, A],
    //
    //         (Up, A) => vec![Right, A],
    //         (Up, Up) => vec![A],
    //         (Up, Left) => vec![Down, Left, A],
    //         (Up, Down) => vec![Down, A],
    //         (Up, Right) => vec![Down, Right, A],
    //
    //         (Left, A) => vec![Right, Right, Up, A],
    //         (Left, Up) => vec![Right, Up, A],
    //         (Left, Left) => vec![A],
    //         (Left, Down) => vec![Right, A],
    //         (Left, Right) => vec![Right, Right, A],
    //
    //         (Down, A) => vec![Right, Up, A],
    //         (Down, Up) => vec![Up, A],
    //         (Down, Left) => vec![Left, A],
    //         (Down, Down) => vec![A],
    //         (Down, Right) => vec![Right, A],
    //
    //         (Right, A) => vec![Up, A],
    //         (Right, Up) => vec![Left, Up, A],
    //         (Right, Left) => vec![Left, Left, A],
    //         (Right, Down) => vec![Left, A],
    //         (Right, Right) => vec![A],
    //     };
    //     path.into_iter()
    // }
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

enum PadType {
    Keypad,
    Dpad,
}
// Generate the moves necessary to cause the controlled robot to press a sequence of buttons ending
// in A. The resulting move seequence will end in an A.
fn det_cost(
    tp: PadType,
    indirections: i32,
    buttons: impl IntoIterator<Item = impl Button + std::fmt::Debug>,
) -> i32 {
    let mut moves = Vec::new();

    let mut pos = match tp {
        PadType::Keypad => <KeypadBtn as Button>::apos(),
        PadType::Dpad => <DpadBtn as Button>::apos(),
    };
    dbg!(&pos);

    for btn in buttons {
        for step in btn.optimal_path_dpad(pos) {
            moves.push(step);
        }
        pos = btn.pos();
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
        det_cost(PadType::Dpad, indirections - 1, moves)
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
            let presses = det_cost(PadType::Keypad, indirections, buttons.into_iter());
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
