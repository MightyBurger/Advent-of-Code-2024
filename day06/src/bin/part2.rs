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

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    Open(Explored),
}

impl Tile {
    fn mark(&self, dir: Dir) -> Self {
        match self {
            Self::Obstacle => Self::Obstacle,
            Self::Open(explored) => Self::Open(explored.mark(dir)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Explored {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl Explored {
    fn unexplored() -> Self {
        Self {
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }
}

impl Explored {
    fn mark(&self, dir: Dir) -> Self {
        match dir {
            Dir::Up => Self { up: true, ..*self },
            Dir::Down => Self {
                down: true,
                ..*self
            },
            Dir::Left => Self {
                left: true,
                ..*self
            },
            Dir::Right => Self {
                right: true,
                ..*self
            },
        }
    }
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
    fn mark(&mut self, pos: Pos, dir: Dir) {
        if self.in_bounds(pos) {
            self.0[pos.row as usize][pos.col as usize] =
                self.0[pos.row as usize][pos.col as usize].mark(dir);
        }
    }
}

#[derive(Debug, Clone)]
// The Game struct contains both the map and the guard.
struct Game {
    map: Map,
    guard: Option<Guard>,
}

#[derive(Debug, Clone, Copy)]
enum GameResult {
    GuardLeft,
    GuardLoop,
}
#[derive(Debug, Clone, Copy)]
enum UpdateResult {
    GuardLeft,
    GuardLoop,
    Undecided,
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
                        '.' => Tile::Open(Explored::unexplored()),
                        '^' => {
                            guard = Some(Guard {
                                pos: Pos {
                                    row: row as i32,
                                    col: col as i32,
                                },
                                dir: Dir::Up,
                            });
                            Tile::Open(Explored::unexplored())
                        }

                        _ => panic!("failed to parse map"),
                    })
                    .collect()
            })
            .collect();

        Self::new(Map(map), guard)
    }

    // Step the simulation forward one timestep.
    fn step(&mut self) -> UpdateResult {
        // There's only work to do if a guard is on the map.
        let Some(guard) = self.guard else {
            return UpdateResult::GuardLeft;
        };

        // Was this already explored in our current direction?
        let guards_tile = self.map.tile(guard.pos).expect("guard in map");
        if let Tile::Open(explored) = guards_tile {
            match guard.dir {
                Dir::Up if explored.up => {
                    return UpdateResult::GuardLoop;
                }
                Dir::Down if explored.down => {
                    return UpdateResult::GuardLoop;
                }
                Dir::Left if explored.left => {
                    return UpdateResult::GuardLoop;
                }
                Dir::Right if explored.right => {
                    return UpdateResult::GuardLoop;
                }
                _ => (),
            }
        }

        // Mark current position as explored.
        self.map.mark(guard.pos, guard.dir);

        // The tile the guard is facing
        let facing = self.map.tile(guard.pos + guard.dir.offset());

        // In every step, the guard either turns or moves.
        match facing {
            Some(Tile::Obstacle) => {
                self.guard = Some(Guard {
                    dir: guard.dir.turn(),
                    ..guard
                });
                UpdateResult::Undecided
            }
            Some(Tile::Open(_)) => {
                self.guard = Some(Guard {
                    pos: guard.pos + guard.dir.offset(),
                    ..guard
                });
                UpdateResult::Undecided
            }
            None => {
                self.guard = None;
                UpdateResult::GuardLeft
            }
        }
    }

    fn simulate(&mut self) -> (GameResult, i32) {
        let mut counter: i32 = 0;
        loop {
            match self.step() {
                UpdateResult::GuardLeft => return (GameResult::GuardLeft, counter),
                UpdateResult::GuardLoop => return (GameResult::GuardLoop, counter),
                UpdateResult::Undecided => {
                    counter += 1;
                }
            }
        }
    }

    #[allow(dead_code)]
    fn disp(&self) {
        for row in self.map.0.iter() {
            for tile in row.iter() {
                match tile {
                    Tile::Obstacle => print!("#"),
                    Tile::Open(explored) => {
                        match (explored.up, explored.down, explored.left, explored.right) {
                            (false, false, false, false) => print!("."),
                            (true, _, false, false) | (_, true, false, false) => print!("|"),
                            (false, false, true, _) | (false, false, _, true) => print!("-"),
                            _ => print!("+"),
                        }
                    }
                }
            }
            println!();
        }
    }
}

use rayon::prelude::*;
use std::collections::HashSet;

fn process(input: &str) -> i32 {
    let start_game = Game::init(&input);

    // First, identify every position the guard has visited
    let mut testgame = start_game.clone();
    let mut visited: HashSet<Pos> = HashSet::new();
    loop {
        testgame.step();
        if let Some(guard) = testgame.guard {
            if guard.pos != start_game.guard.unwrap().pos {
                visited.insert(guard.pos);
            }
        } else {
            break;
        }
    }

    let total = visited.len();
    println!("There are {} positions to explore.", total);

    // Now, rerun the simulation again, placing obstacles wherever the guard visited.
    visited
        .par_iter()
        .map(|new_obst_pos| {
            let mut newgame = start_game.clone();
            newgame.map.update(*new_obst_pos, Tile::Obstacle);
            newgame.simulate()
        })
        .filter(|(result, _)| matches!(result, GameResult::GuardLoop))
        .count() as i32
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
        assert_eq!(process(check), 6)
    }

    #[test]
    fn michael_test() {
        let test = r".##..
....#
.....
.^.#.
.....";
        assert_eq!(process(test), 1);
    }

    #[test]
    fn tight_corner_test() {
        let test = r".....
...#.
....#
.....
...^.
...#.";
        assert_eq!(process(test), 1);
    }

    #[test]
    fn did_position_work() {
        let pos1 = Pos { row: 1, col: 2 };
        let pos2 = Pos { row: 5, col: 10 };
        let posadd = Pos { row: 6, col: 12 };
        assert_eq!(posadd, pos1 + pos2);
    }
}
