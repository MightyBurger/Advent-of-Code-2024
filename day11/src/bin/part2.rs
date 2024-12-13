fn split_in_half(stone: &str) -> (String, String) {
    (
        stone[0..stone.len() / 2].to_string(),
        eliminate_leading_zeros(&stone[stone.len() / 2..stone.len()]),
    )
}
fn eliminate_leading_zeros(numstr: &str) -> String {
    let num: u64 = numstr.parse().unwrap();
    format!("{}", num)
}

fn stoned_out_man(steps: i32, stone: &str) -> u64 {
    if steps == 0 {
        1
    } else {
        if stone == "0" {
            stoned_out_man(steps - 1, "1")
        } else if stone.len() % 2 == 0 {
            let (left, right) = split_in_half(stone);
            stoned_out_man(steps - 1, &left) + stoned_out_man(steps - 1, &right)
        } else {
            stoned_out_man(
                steps - 1,
                &format!("{}", stone.parse::<u64>().unwrap() * 2024),
            )
        }
    }
}

fn process(input: &str) -> u64 {
    let stones: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();
    stones.iter().map(|stone| stoned_out_man(35, stone)).sum()
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
