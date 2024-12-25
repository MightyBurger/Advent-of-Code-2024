use itertools::Itertools;

fn process(input: &str) -> usize {
    let mut locks: Vec<[i32; 5]> = Vec::new();
    let mut keys: Vec<[i32; 5]> = Vec::new();

    for schematic in input.split("\n\n") {
        let mut pattern = [0i32; 5];

        for line in schematic.lines().skip(1).take(5) {
            for (i, c) in line.chars().enumerate() {
                if c == '#' {
                    pattern[i] += 1;
                }
            }
        }

        match schematic.chars().next() {
            Some('#') => locks.push(pattern),
            Some('.') => keys.push(pattern),
            Some(c) => panic!("unexpected character {}", c),
            None => panic!("empty schematic"),
        };
    }

    locks
        .into_iter()
        .cartesian_product(keys.into_iter())
        .filter(|(lock, key)| (0..5).all(|i| lock[i] + key[i] <= 5))
        .count()
}

fn main() {
    let input = include_str!("../../../day25/input1.txt");
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
        let check = include_str!("../../../day25/check1.txt");
        assert_eq!(process(check), 3)
    }
}
