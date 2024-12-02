fn main() {
    let input = include_str!("../../../day01/input.txt");
    let distance = process(input);
    println!("The result is {}", distance);
}

fn process(input: &str) -> i32 {
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();
    for line in input.lines() {
        let mut tokens = line.split_whitespace();
        list1.push(tokens.next().unwrap().parse().unwrap());
        list2.push(tokens.next().unwrap().parse().unwrap());
    }

    let mut similarity = 0;

    for num in list1.iter() {
        similarity += num * list2.iter().filter(|x| *x == num).count() as i32;
    }

    similarity
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let check = include_str!("../../../day01/check.txt");
        process(check);
        assert_eq!(process(check), 31)
    }
}
