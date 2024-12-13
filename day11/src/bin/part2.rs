fn split_in_half(stone: u64) -> (u64, u64) {
    let div = identify_number_of_digits(stone) / 2; // expected to be divisible by 2
    let div = u32::pow(10, div) as u64;

    (stone / div, stone % div)
}

fn identify_number_of_digits(stone: u64) -> u32 {
    match stone {
        0 => 1,
        _ => stone.ilog10() + 1,
    }
}

// returns how many stones this stone split into
fn stoned_out_man(steps: i32, stone: u64) -> u64 {
    if steps == 0 {
        1
    } else {
        if stone == 0 {
            stoned_out_man(steps - 1, 1)
        } else if identify_number_of_digits(stone) % 2 == 0 {
            let (left, right) = split_in_half(stone);
            stoned_out_man(steps - 1, left) + stoned_out_man(steps - 1, right)
        } else {
            stoned_out_man(steps - 1, stone * 2024)
        }
    }
}

fn process(input: &str, iterations: i32) -> u64 {
    let stones: Vec<u64> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    stones
        .into_iter()
        .map(|stone| stoned_out_man(iterations, stone))
        .sum()
}

fn main() {
    let input = include_str!("../../../day11/input1.txt");
    let distance = process(input, 75);
    println!("The result is {}", distance);
}

// ----------------------------------------------------
// -------------------- Unit Tests --------------------
// ----------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_in_half() {
        assert_eq!(split_in_half(1002), (10, 2));
        assert_eq!(split_in_half(10), (1, 0));
        assert_eq!(split_in_half(123456), (123, 456));
        assert_eq!(split_in_half(12345678), (1234, 5678));
    }

    #[test]
    fn test_num_digits() {
        assert_eq!(identify_number_of_digits(1024), 4);
        assert_eq!(identify_number_of_digits(0), 1);
        assert_eq!(identify_number_of_digits(1), 1);
        assert_eq!(identify_number_of_digits(10000), 5);
    }

    #[test]
    fn test() {
        let check = include_str!("../../../day11/check1.txt");
        assert_eq!(process(check, 25), 55312);
    }
}
