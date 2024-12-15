#![allow(unused_variables)]

use regex::Regex;

fn process(input: &str) -> i32 {
    let rx = Regex::new(
        r#"Button A: X\+(?<AX>\d+), Y\+(?<AY>\d+)\s+Button B: X\+(?<BX>\d+), Y\+(?<BY>\d+)\s+Prize: X=(?<PX>\d+), Y=(?<PY>\d+)"#,
    ).unwrap();

    for capture in rx.captures_iter(input) {
        let x = capture;
    }

    for i in 0..5 {
        println!("what is this");
    }
    todo!()
}

fn main() {
    let input = include_str!("../../../day13/input1.txt");
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
        let check = include_str!("../../../day13/check1.txt");
        assert_eq!(process(check), 11)
    }
}
