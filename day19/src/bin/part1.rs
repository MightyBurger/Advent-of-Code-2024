fn can_make_work(towels: &[&str], pattern: &str) -> bool {
    for towel in towels.into_iter() {
        if pattern.len() < towel.len() {
            continue;
        } else if *towel == pattern {
            return true;
        } else if *towel == &pattern[0..towel.len()] && pattern.len() > towel.len() {
            let remaining_pattern = &pattern[towel.len()..];
            if can_make_work(towels, remaining_pattern) {
                return true;
            }
        }
    }
    false
}

fn process(input: &str) -> i32 {
    let (input1, input2) = input.split_once("\n\n").unwrap();
    let towels: Vec<&str> = input1.split(",").map(|token| token.trim()).collect();

    input2
        .lines()
        .filter(|pattern| pattern.len() > 0)
        .filter(|pattern| can_make_work(&towels, pattern))
        .count() as i32
}

fn main() {
    let input = include_str!("../../../day19/input1.txt");

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
        let check = include_str!("../../../day19/check1.txt");
        assert_eq!(process(check), 6)
    }
}
