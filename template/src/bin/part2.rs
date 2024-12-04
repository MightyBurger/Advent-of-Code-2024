fn process(input: &str) -> i32 {
    println!("{}", input);
    22
}

fn main() {
    let input = include_str!("../../../dayxx/input2.txt");
    let distance = process(input);
    println!("The result is {}", distance);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let check = include_str!("../../../day0x/check2.txt");
        assert_eq!(process(check), 11)
    }
}
