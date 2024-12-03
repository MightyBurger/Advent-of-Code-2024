#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum IncreasingResult {
    Increasing,
    Decreasing,
    Neither,
}

fn increasing(input: &Vec<i32>) -> IncreasingResult {
    let mut all_increasing = true;
    for window in input.windows(2) {
        if window[0] > window[1] {
            all_increasing = false;
        }
    }

    let mut all_decreasing = true;
    for window in input.windows(2) {
        if window[0] < window[1] {
            all_decreasing = false;
        }
    }

    match (all_increasing, all_decreasing) {
        (true, false) => IncreasingResult::Increasing,
        (false, true) => IncreasingResult::Decreasing,
        _ => IncreasingResult::Neither,
    }
}

fn numbers_close(input: &Vec<i32>) -> bool {
    let mut result = true;
    for window in input.windows(2) {
        let diff = (window[0] - window[1]).abs();
        if !(1..=3).contains(&diff) {
            result = false;
        }
    }
    result
}

fn safe_undamped(input: &Vec<i32>) -> bool {
    let safe_from_incr = match increasing(&input) {
        IncreasingResult::Neither => false,
        _ => true,
    };
    let safe_from_close = numbers_close(&input);

    safe_from_close && safe_from_incr
}

fn process(input: &str) -> i32 {
    let mut safe_count = 0;
    for line in input.lines() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let mut is_safe = false;
        for i in 0..numbers.len() {
            let mut numbers_smaller = numbers.clone();
            numbers_smaller.remove(i);
            if safe_undamped(&numbers_smaller) {
                is_safe = true;
            }
        }
        if is_safe {
            safe_count += 1;
        }
    }
    safe_count
}

fn main() {
    let input = include_str!("../../../day02/input.txt");
    println!("The result is {}", process(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let check = include_str!("../../../day02/check.txt");
        assert_eq!(process(check), 4)
    }

    #[test]
    fn test_increasing() {
        assert_eq!(increasing(&vec![0, 1, 2, 3]), IncreasingResult::Increasing);
        assert_eq!(increasing(&vec![3, 2, 1, 0]), IncreasingResult::Decreasing);
        assert_eq!(increasing(&vec![0, 2, 1, 3]), IncreasingResult::Neither);
    }

    #[test]
    fn test_numbers_close() {
        assert_eq!(numbers_close(&vec![0, 1, 3, 6]), true);
        assert_eq!(numbers_close(&vec![0, 1, 3, 7]), false);
        assert_eq!(numbers_close(&vec![0, 0, 3, 6]), false);
    }
}
