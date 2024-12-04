use regex::Regex;

fn process(input: &str) -> i32 {
    let regex_ignore = Regex::new(r"(?s)don\'t\(\).*?do\(\)|don\'t\(\).*?$").unwrap();
    let regex_extract_mult = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum = 0;

    for enabled in regex_ignore.split(input) {
        for capture in regex_extract_mult.captures_iter(&enabled) {
            let left: i32 = capture[1].parse().expect("failed to parse");
            let right: i32 = capture[2].parse().expect("failed to parse");
            sum += left * right;
        }
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
        let check = include_str!("../../../day03/check2.txt");
        assert_eq!(process(check), 48)
    }
}
