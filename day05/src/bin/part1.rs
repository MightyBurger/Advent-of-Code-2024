struct Rule {
    left: i32,
    right: i32,
}

fn follows_rules(manual: &[i32], rules: &[Rule]) -> bool {
    let mut follows_rules = true;
    for rule in rules.iter() {
        let left_pos = manual.iter().position(|page| *page == rule.left);
        let right_pos = manual.iter().position(|page| *page == rule.right);
        match (left_pos, right_pos) {
            (Some(left_pos), Some(right_pos)) if right_pos < left_pos => {
                follows_rules = false;
            }
            _ => (),
        }
    }
    follows_rules
}

fn process(input: &str) -> i32 {
    let input1 = input.split("\r\n\r\n").nth(0).unwrap();
    let input2 = input.split("\r\n\r\n").nth(1).unwrap();

    let rules: Vec<Rule> = input1
        .lines()
        .map(|line| {
            let left: i32 = line.split('|').nth(0).unwrap().parse().unwrap();
            let right: i32 = line.split('|').nth(1).unwrap().parse().unwrap();
            Rule { left, right }
        })
        .collect();

    let manuals: Vec<Vec<i32>> = input2
        .lines()
        .map(|line| {
            line.split(',')
                .map(|token| token.parse::<i32>().expect("could not parse"))
                .collect()
        })
        .collect();

    manuals
        .iter()
        .filter(|manual| follows_rules(manual, &rules))
        .map(|manual| manual[manual.len() / 2])
        .sum()
}

fn main() {
    let input = include_str!("../../../day05/input1.txt");
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
        let check = include_str!("../../../day05/check1.txt");
        assert_eq!(process(check), 143)
    }
}
