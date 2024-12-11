fn process(input: &str) -> i32 {
    todo!()
}

fn main() {
    let input = include_str!("../../../dayxx/input1.txt");
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
        let check = include_str!("../../../dayxx/check1.txt");
        assert_eq!(process(check), 11)
    }
}
