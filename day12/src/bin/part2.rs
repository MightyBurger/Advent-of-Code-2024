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

use std::ops::{Add, Sub};
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

trait Grid<T> {
    fn grab(&self, position: Pos) -> Option<&T>;
    fn in_bounds(&self, position: Pos) -> bool {
        self.grab(position).is_some()
    }
}

impl<T> Grid<T> for Vec<Vec<T>> {
    fn grab(&self, position: Pos) -> Option<&T> {
        self.get(position.col as usize)?.get(position.row as usize)
    }
}

use std::collections::HashSet;
#[derive(Debug, Clone)]
struct Garden {
    plant: char,
    span: HashSet<Pos>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Facing {
    Top,
    Bottom,
    Left,
    Right,
}

impl Garden {
    fn covers(&self, pos: Pos) -> bool {
        self.span.contains(&pos)
    }
    fn area(&self) -> i32 {
        self.span.len() as i32
    }
    fn perimeter(&self) -> i32 {
        self.span
            .iter()
            .flat_map(|pos| {
                let offsets = vec![
                    Pos::new(-1, 0),
                    Pos::new(1, 0),
                    Pos::new(0, -1),
                    Pos::new(0, 1),
                ];
                offsets
                    .into_iter()
                    .map(|offsets| if self.covers(*pos + offsets) { 0 } else { 1 })
            })
            .sum()
    }

    fn explore_side(
        &self,
        pos: Pos,
        facing: Facing,
        fences: &HashSet<(Pos, Facing)>,
        side_positions: &mut HashSet<Pos>,
    ) {
        side_positions.insert(pos);
        // Explore vertically or horizontally?
        // Recall Pos::new(col, row)
        let offsets = match facing {
            Facing::Top | Facing::Bottom => vec![Pos::new(0, 1), Pos::new(0, -1)],
            Facing::Left | Facing::Right => vec![Pos::new(1, 0), Pos::new(-1, 0)],
        };
        for offset in offsets.into_iter() {
            let looking = offset + pos;
            match fences.get(&(looking, facing)) {
                Some(_) if !side_positions.contains(&looking) => {
                    self.explore_side(looking, facing, fences, side_positions);
                }
                _ => (),
            }
        }
    }

    fn find_side_positions(
        &self,
        pos: Pos,
        facing: Facing,
        fences: &HashSet<(Pos, Facing)>,
    ) -> HashSet<Pos> {
        let mut side_positions: HashSet<Pos> = HashSet::new();
        self.explore_side(pos, facing, fences, &mut side_positions);
        side_positions
    }

    fn sides(&self) -> i32 {
        let mut fences: HashSet<(Pos, Facing)> = HashSet::new();
        for pos in self.span.iter() {
            let offsets = vec![
                (Pos::new(1, 0), Facing::Top),
                (Pos::new(-1, 0), Facing::Bottom),
                (Pos::new(0, -1), Facing::Right),
                (Pos::new(0, 1), Facing::Left),
            ];
            for (offset, facing) in offsets.into_iter() {
                if !self.covers(*pos + offset) {
                    fences.insert((*pos + offset, facing));
                }
            }
        }

        let mut sides: Vec<HashSet<Pos>> = Vec::new();
        for (fence_pos, fence_facing) in fences.iter() {
            let mut explored = false;
            for side in sides.iter() {
                if side.contains(&fence_pos) {
                    explored = true;
                }
            }
            if !explored {
                sides.push(self.find_side_positions(*fence_pos, *fence_facing, &fences));
            }
        }
        sides.len() as i32
    }
}

fn explore(farm: &Vec<Vec<char>>, my_plant: char, pos: Pos, so_far: &mut HashSet<Pos>) {
    so_far.insert(pos);
    let offsets = vec![
        Pos::new(-1, 0),
        Pos::new(1, 0),
        Pos::new(0, -1),
        Pos::new(0, 1),
    ];
    for offset in offsets.into_iter() {
        match farm.grab(pos + offset) {
            Some(plant) if *plant == my_plant && !so_far.contains(&(pos + offset)) => {
                explore(farm, my_plant, pos + offset, so_far)
            }
            _ => (),
        }
    }
}

fn find_garden(farm: &Vec<Vec<char>>, pos: Pos) -> Garden {
    let plant = *(farm
        .grab(pos)
        .expect("call explore_garden with something in the farm"));

    let mut span: HashSet<Pos> = HashSet::new();
    explore(&farm, plant, pos, &mut span);

    Garden { plant, span }
}

fn process(input: &str) -> i32 {
    let farm: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut gardens: Vec<Garden> = Vec::new();

    let cols = farm.len() as i32;
    let rows = farm[0].len() as i32;

    for col in 0..cols {
        for row in 0..rows {
            let pos = Pos::new(col, row);
            let mut explored = false;
            for garden in gardens.iter() {
                if garden.covers(pos) {
                    explored = true;
                }
            }
            if !explored {
                gardens.push(find_garden(&farm, pos));
            }
        }
    }

    gardens
        .iter()
        .map(|garden| garden.perimeter() * garden.sides())
        .sum()
}

fn main() {
    let input = include_str!("../../../day12/input2.txt");
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
    fn magical_grid_stuff() {
        let check = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(check.in_bounds(Pos::new(-1, -1)), false);
        assert_eq!(check.in_bounds(Pos::new(-1, 0)), false);
        assert_eq!(check.in_bounds(Pos::new(0, -1)), false);
        assert_eq!(check.in_bounds(Pos::new(0, 0)), true);
        assert_eq!(check.in_bounds(Pos::new(0, 2)), true);
        assert_eq!(check.in_bounds(Pos::new(0, 3)), false);
        assert_eq!(check.in_bounds(Pos::new(1, 0)), true);
        assert_eq!(check.in_bounds(Pos::new(2, 0)), false);
        assert_eq!(check.grab(Pos::new(1, 2)), Some(&6));
    }

    #[test]
    fn area_perimeter_sides() {
        let input = r#"AA
AB
CC
DC
CC
"#;
        let farm: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let mut gardens: Vec<Garden> = Vec::new();

        let cols = farm.len() as i32;
        let rows = farm[0].len() as i32;

        for col in 0..cols {
            for row in 0..rows {
                let pos = Pos::new(col, row);
                let mut explored = false;
                for garden in gardens.iter() {
                    if garden.covers(pos) {
                        explored = true;
                    }
                }
                if !explored {
                    gardens.push(find_garden(&farm, pos));
                }
            }
        }

        assert_eq!(gardens[0].perimeter(), 8); // A
        assert_eq!(gardens[1].perimeter(), 4); // B
        assert_eq!(gardens[2].perimeter(), 12); // C
        assert_eq!(gardens[3].perimeter(), 4); // D
        assert_eq!(gardens[0].area(), 3); // A
        assert_eq!(gardens[1].area(), 1); // B
        assert_eq!(gardens[2].area(), 5); // C
        assert_eq!(gardens[3].area(), 1); // D
        assert_eq!(gardens[0].sides(), 5); // A
        assert_eq!(gardens[1].sides(), 4); // B
        assert_eq!(gardens[2].sides(), 8); // C
        assert_eq!(gardens[3].sides(), 4); // D
    }

    #[test]
    fn our_specific_example() {
        let input = r#"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
"#;
        let farm: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let mut gardens: Vec<Garden> = Vec::new();

        let cols = farm.len() as i32;
        let rows = farm[0].len() as i32;

        for col in 0..cols {
            for row in 0..rows {
                let pos = Pos::new(col, row);
                let mut explored = false;
                for garden in gardens.iter() {
                    if garden.covers(pos) {
                        explored = true;
                    }
                }
                if !explored {
                    gardens.push(find_garden(&farm, pos));
                }
            }
        }
        assert_eq!(gardens.len(), 3);
        dbg!(&gardens);
        //assert_eq!(gardens[0].area(), )
    }
    #[test]
    fn test() {
        let check = include_str!("../../../day12/check2.txt");
        assert_eq!(process(check), 368)
    }
}
