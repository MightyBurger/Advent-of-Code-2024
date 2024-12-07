#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn offset(&self) -> Pos {
        match self {
            Dir::Up => Pos { row: -1, col: 0 },
            Dir::Down => Pos { row: 1, col: 0 },
            Dir::Left => Pos { row: 0, col: -1 },
            Dir::Right => Pos { row: 0, col: 1 },
        }
    }
    fn turn(&self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }
}

impl Default for Dir {
    fn default() -> Self {
        Dir::Up
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Pos {
    row: i32,
    col: i32,
}

use std::ops::Add;
impl Add for Pos {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Obstacle,
    Open(bool),
}

#[derive(Default, Debug, Clone, Copy)]
struct Guard {
    pos: Pos,
    dir: Dir,
}

#[derive(Debug, Clone)]
struct Map(Vec<Vec<Tile>>);

impl Map {
    fn in_bounds(&self, pos: Pos) -> bool {
        let rows = self.0.len() as i32;
        let Some(first_col) = self.0.get(0) else {
            return false;
        };
        let cols = first_col.len() as i32;

        pos.row >= 0 && pos.row < rows && pos.col >= 0 && pos.col < cols
    }
    fn tile(&self, pos: Pos) -> Option<Tile> {
        if self.in_bounds(pos) {
            Some(self.0[pos.row as usize][pos.col as usize])
        } else {
            None
        }
    }
    fn update(&mut self, pos: Pos, tile: Tile) {
        if self.in_bounds(pos) {
            self.0[pos.row as usize][pos.col as usize] = tile;
        }
    }
    fn explored(&self) -> i32 {
        self.0
            .iter()
            .flatten()
            .filter(|tile| matches!(tile, Tile::Open(true)))
            .count() as i32
    }
}

#[derive(Debug, Clone)]
// The Game struct contains both the map and the guard.
struct Game {
    map: Map,
    guard: Option<Guard>,
}

impl Game {
    fn new(map: Map, guard: Option<Guard>) -> Self {
        Self { map, guard }
    }
    fn init(input: &str) -> Self {
        let mut guard: Option<Guard> = None;

        let map: Vec<Vec<Tile>> = input
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col, c)| match c {
                        '#' => Tile::Obstacle,
                        '.' => Tile::Open(false),
                        '^' => {
                            guard = Some(Guard {
                                pos: Pos {
                                    row: row as i32,
                                    col: col as i32,
                                },
                                dir: Dir::Up,
                            });
                            Tile::Open(false)
                        }

                        _ => panic!("failed to parse map"),
                    })
                    .collect()
            })
            .collect();

        Self::new(Map(map), guard)
    }

    // Is the guard in-bounds?
    fn guard_in(&self) -> bool {
        self.guard.is_some()
    }

    // Step the simulation forward one timestep.
    fn step(&mut self) {
        // There's only work to do if a guard is on the map.
        let Some(guard) = self.guard else { return };

        // Mark current position as explored.
        self.map.update(guard.pos, Tile::Open(true));

        // The position the guard is facing
        let facing = guard.pos + guard.dir.offset();
        // The tile the guard is facing
        let facing = self.map.tile(facing);

        let next_dir = match facing {
            Some(Tile::Obstacle) => guard.dir.turn(),
            Some(Tile::Open(_)) | None => guard.dir,
        };

        let next_pos = guard.pos + next_dir.offset();

        // Move the guard.
        self.guard = match self.map.in_bounds(next_pos) {
            true => Some(Guard {
                pos: next_pos,
                dir: next_dir,
            }),
            false => None,
        }
    }
}

fn process(input: &str) -> i32 {
    let mut game = Game::init(&input);
    while game.guard_in() {
        game.step();
    }

    game.map.explored()
}

fn main() {
    let input = include_str!("../../../day06/input.txt");
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
        let check = include_str!("../../../day06/check.txt");
        assert_eq!(process(check), 41)
    }

    #[test]
    fn did_position_work() {
        let pos1 = Pos { row: 1, col: 2 };
        let pos2 = Pos { row: 5, col: 10 };
        let posadd = Pos { row: 6, col: 12 };
        assert_eq!(posadd, pos1 + pos2);
    }
}
