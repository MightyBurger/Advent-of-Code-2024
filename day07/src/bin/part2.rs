#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Equation {
    test: i64,
    operands: Vec<i64>,
}
fn parse_input(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(":");
            let test: i64 = split.next().unwrap().parse().unwrap();
            let operands: Vec<i64> = split
                .next()
                .unwrap()
                .split_whitespace()
                .filter_map(|token| token.parse().ok())
                .collect();
            Equation { test, operands }
        })
        .collect()
}

use itertools::Itertools;
fn valid(eq: &Equation) -> bool {
    let seq_len = eq.operands.len() - 1;
    let operators = [
        |a: i64, b: i64| a + b,
        |a: i64, b: i64| a * b,
        |a: i64, b: i64| format!("{a}{b}").parse().unwrap(),
    ];
    for number in (0..seq_len)
        .map(|_| operators.iter())
        .multi_cartesian_product() // iterates over [+, +, +], [+, +, *], [+, +, ||], [+, *, +] etc.
        .map(|op_sequence| {
            op_sequence
                .iter()
                .zip(eq.operands.iter().skip(1))
                .fold(eq.operands[0], |acc, (f, num)| f(acc, *num))
        })
    {
        if number == eq.test {
            return true;
        }
    }
    false
}
fn process(input: &str) -> i64 {
    let eqs = parse_input(input);

    eqs.iter().filter(|eq| valid(eq)).map(|eq| eq.test).sum()
}

fn main() {
    let input = include_str!("../../../day07/input2.txt");
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
        let check = include_str!("../../../day07/check2.txt");
        assert_eq!(process(check), 11387)
    }

    #[test]
    fn parsetest() {
        let check = r"1: 2 3 4
5: 6 7";
        assert_eq!(
            parse_input(check),
            vec![
                Equation {
                    test: 1,
                    operands: vec!(2, 3, 4)
                },
                Equation {
                    test: 5,
                    operands: vec!(6, 7)
                }
            ]
        );
    }

    #[test]
    fn validtest() {
        assert_eq!(
            valid(&Equation {
                test: 182, // 4*5+6*7
                operands: vec![4, 5, 6, 7],
            }),
            true
        );
    }
}
