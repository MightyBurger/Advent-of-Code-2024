use std::collections::HashMap;
fn how_many_combinations<'a>(
    towels: &[&str],
    pattern: &'a str,
    cache: &mut HashMap<&'a str, u64>,
) -> u64 {
    if let Some(result) = cache.get(pattern) {
        return *result;
    }
    let mut result = 0;
    for towel in towels.into_iter() {
        if pattern.len() < towel.len() {
            continue;
        } else if *towel == pattern {
            result += 1;
        } else if *towel == &pattern[0..towel.len()] && pattern.len() > towel.len() {
            let remaining_pattern = &pattern[towel.len()..];
            result += how_many_combinations(towels, remaining_pattern, cache);
        }
    }
    cache.insert(pattern, result);
    result
}

fn process(input: &str) -> u64 {
    let (input1, input2) = input.split_once("\n\n").unwrap();
    let towels: Vec<&str> = input1.split(",").map(|token| token.trim()).collect();

    input2
        .lines()
        .filter(|pattern| pattern.len() > 0)
        .map(|pattern| how_many_combinations(&towels, pattern, &mut HashMap::new()))
        .sum()
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
        assert_eq!(process(check), 16)
    }
}
