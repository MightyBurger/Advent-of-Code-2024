fn eliminate_leading_zeros(numstr: &str) -> String {
    let num: u64 = numstr.parse().unwrap();
    format!("{}", num)
}
fn process(input: &str) -> i32 {
    let mut stones: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();

    for _ in 0..25 {
        // In the worst case, the new vec must be twice as long... so let's just try to reduce
        // any memory allocations.
        let mut newstones: Vec<String> = Vec::with_capacity(stones.len() * 2);

        for stone in stones.iter() {
            if stone == "0" {
                newstones.push("1".to_string());
            } else if stone.len() % 2 == 0 {
                newstones.push((&stone[0..stone.len() / 2]).to_string());
                newstones.push(eliminate_leading_zeros(
                    &stone[stone.len() / 2..stone.len()],
                ));
            } else {
                newstones.push(format!("{}", stone.parse::<u64>().unwrap() * 2024))
            }
        }
        stones = newstones;
    }
    stones.len() as i32
}

fn main() {
    let input = include_str!("../../../day11/input1.txt");
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
        let check = include_str!("../../../day11/check1.txt");
        assert_eq!(process(check), 55312)
    }
}
