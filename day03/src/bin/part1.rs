use regex::Regex;

fn process(input: &str) -> i32 {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum = 0;
    for capture in regex.captures_iter(input) {
        let left: i32 = capture[1].parse().expect("failed to parse");
        let right: i32 = capture[2].parse().expect("failed to parse");
        sum += left * right;
    }
    sum
}

fn main() {
    let input = include_str!("../../../day03/input.txt");
    println!("The result is {}", process(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let check = include_str!("../../../day03/check1.txt");
        assert_eq!(process(check), 161)
    }
}
