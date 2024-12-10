fn process(input: &str) -> i32 {
    println!("{}", input);
    11
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
        assert_eq!(process(check), 11)
    }
}
