use std::fs;

fn main() {
    let input = fs::read_to_string("day1/input.txt").expect("unable to read file");

    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();
    for line in input.lines() {
        let mut tokens = line.split_whitespace();
        list1.push(tokens.next().unwrap().parse().unwrap());
        list2.push(tokens.next().unwrap().parse().unwrap());
    }

    list1.sort();
    list2.sort();

    let distance = list1
        .iter()
        .zip(list2.iter())
        .fold(0, |acc, num| acc + (num.0 - num.1).abs());

    println!("The total distance is {}", distance);
}
