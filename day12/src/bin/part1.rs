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
        .map(|garden| garden.perimeter() * garden.area())
        .sum()
}

fn main() {
    let input = include_str!("../../../day12/input1.txt");
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
    fn area_perimeter() {
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

        dbg!(&gardens);
        assert_eq!(gardens[0].perimeter(), 8); // A
        assert_eq!(gardens[1].perimeter(), 4); // B
        assert_eq!(gardens[2].perimeter(), 12); // C
        assert_eq!(gardens[3].perimeter(), 4); // D
        assert_eq!(gardens[0].area(), 3); // A
        assert_eq!(gardens[1].area(), 1); // B
        assert_eq!(gardens[2].area(), 5); // C
        assert_eq!(gardens[3].area(), 1); // D
    }

    #[test]
    fn test() {
        let check = include_str!("../../../day12/check1.txt");
        assert_eq!(process(check), 1930)
    }
}
