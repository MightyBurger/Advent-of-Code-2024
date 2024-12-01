use std::fs;

fn main() {
    //let input = fs::read_to_string("day1/check.txt").expect("unable to read file");
    let input = fs::read_to_string("day1/input.txt").expect("unable to read file");

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

    println!("The total similarity is {}", similarity);
}
