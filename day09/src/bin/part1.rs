#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Block {
    Data(u64),
    Free,
}
use itertools::Itertools;
fn process(input: &str) -> u64 {
    let pad_a_zero = input.trim().to_string() + "0";
    let mut disk: Vec<Block> = Vec::new();

    for (id, (file_len, free_space)) in pad_a_zero.chars().tuples().enumerate() {
        let file_len = file_len.to_digit(10).unwrap();
        let free_space = free_space.to_digit(10).unwrap();

        for _ in 0..file_len {
            disk.push(Block::Data(id as u64));
        }
        for _ in 0..free_space {
            disk.push(Block::Free);
        }
    }

    let mut left: usize = 0;
    let mut right: usize = disk.len() - 1;

    while left < right {
        if matches!(disk[left], Block::Data(_)) {
            left += 1;
            continue;
        } else if disk[right] == Block::Free {
            right -= 1;
            continue;
        }
        disk.swap(left, right);
    }

    disk.iter()
        .enumerate()
        .map(|(i, block)| match block {
            Block::Free => 0,
            Block::Data(id) => i as u64 * id,
        })
        .sum()
}

fn main() {
    let input = include_str!("../../../day09/input1.txt");
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
        let check = include_str!("../../../day09/check1.txt");
        assert_eq!(process(check), 1928)
    }
}
