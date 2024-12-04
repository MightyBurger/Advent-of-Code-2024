fn check_for_xmas(input: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    let cross1: String = [
        input[row - 1][col - 1],
        input[row][col],
        input[row + 1][col + 1],
    ]
    .iter()
    .collect();

    let cross2: String = [
        input[row + 1][col - 1],
        input[row][col],
        input[row - 1][col + 1],
    ]
    .iter()
    .collect();

    (cross1 == "MAS" || cross1 == "SAM") && (cross2 == "MAS" || cross2 == "SAM")
}

fn process(input: &str) -> i32 {
    let mut invec: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        invec.push(line.chars().collect());
    }

    let rows = invec.len();
    let cols = invec[0].len();

    let mut sum: i32 = 0;
    for i in 1..rows - 1 {
        for k in 1..cols - 1 {
            if check_for_xmas(&invec, i, k) {
                sum += 1;
            }
        }
    }
    sum
}

fn main() {
    let input = include_str!("../../../day04/input2.txt");
    let distance = process(input);
    println!("The result is {}", distance);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let check = include_str!("../../../day04/check2.txt");
        assert_eq!(process(check), 9)
    }
}
