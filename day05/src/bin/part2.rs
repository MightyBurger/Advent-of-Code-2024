struct Rule {
    left: i32,
    right: i32,
}

fn pagesort(manual: &mut Vec<i32>, rules: &Vec<Rule>) -> bool {
    let mut modified = false;
    for rule in rules.iter() {
        let left_pos = manual.iter().position(|page| *page == rule.left);
        let right_pos = manual.iter().position(|page| *page == rule.right);
        match (left_pos, right_pos) {
            (Some(left_pos), Some(right_pos)) if right_pos < left_pos => {
                let temp = manual[left_pos];
                manual[left_pos] = manual[right_pos];
                manual[right_pos] = temp;
                modified = true;
            }
            _ => (),
        }
    }
    if modified {
        // Other rules may have been broken. Check again.
        pagesort(manual, rules);
    }
    modified
}

fn process(input: &str) -> i32 {
    let input1 = input.split("\r\n\r\n").nth(0).unwrap();
    let input2 = input.split("\r\n\r\n").nth(1).unwrap();

    let mut rules: Vec<Rule> = Vec::new();
    for line in input1.lines() {
        let left: i32 = line.split('|').nth(0).unwrap().parse().unwrap();
        let right: i32 = line.split('|').nth(1).unwrap().parse().unwrap();
        rules.push(Rule { left, right });
    }
    let mut manuals: Vec<Vec<i32>> = input2
        .lines()
        .map(|line| {
            line.split(',')
                .map(|token| token.parse::<i32>().expect("could not parse"))
                .collect()
        })
        .collect();

    let mut sum: i32 = 0;
    for manual in manuals.iter_mut() {
        if pagesort(manual, &rules) {
            sum += manual[manual.len() / 2];
        }
    }
    sum
}

fn main() {
    let input = include_str!("../../../day05/input2.txt");
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
        let check = include_str!("../../../day05/check2.txt");
        assert_eq!(process(check), 123)
    }
}
