fn check_for_xmas(input: &Vec<Vec<char>>, row: usize, col: usize) -> i32 {
    // Eight directions.
    let top_three = row < 3;
    let bottom_three = row > input.len() - 4;
    let left_three = col < 3;
    let right_three = col > input[0].len() - 4;

    let mut all: Vec<String> = Vec::new();

    // right
    if !right_three {
        all.push(
            [
                input[row][col + 0],
                input[row][col + 1],
                input[row][col + 2],
                input[row][col + 3],
            ]
            .iter()
            .collect(),
        );
    }

    // down right
    if !right_three && !bottom_three {
        all.push(
            [
                input[row + 0][col + 0],
                input[row + 1][col + 1],
                input[row + 2][col + 2],
                input[row + 3][col + 3],
            ]
            .iter()
            .collect(),
        );
    }

    // down
    if !bottom_three {
        all.push(
            [
                input[row + 0][col],
                input[row + 1][col],
                input[row + 2][col],
                input[row + 3][col],
            ]
            .iter()
            .collect(),
        );
    }
    // down-left
    if !bottom_three && !left_three {
        all.push(
            [
                input[row + 0][col - 0],
                input[row + 1][col - 1],
                input[row + 2][col - 2],
                input[row + 3][col - 3],
            ]
            .iter()
            .collect(),
        );
    }
    // left
    if !left_three {
        all.push(
            [
                input[row][col - 0],
                input[row][col - 1],
                input[row][col - 2],
                input[row][col - 3],
            ]
            .iter()
            .collect(),
        );
    }

    // left-up
    if !left_three && !top_three {
        all.push(
            [
                input[row - 0][col - 0],
                input[row - 1][col - 1],
                input[row - 2][col - 2],
                input[row - 3][col - 3],
            ]
            .iter()
            .collect(),
        );
    }

    // up
    if !top_three {
        all.push(
            [
                input[row - 0][col],
                input[row - 1][col],
                input[row - 2][col],
                input[row - 3][col],
            ]
            .iter()
            .collect(),
        );
    }

    // up-right
    if !top_three && !right_three {
        all.push(
            [
                input[row - 0][col + 0],
                input[row - 1][col + 1],
                input[row - 2][col + 2],
                input[row - 3][col + 3],
            ]
            .iter()
            .collect(),
        );
    }

    all.iter().filter(|s| *s == "XMAS").count() as i32
}

fn process(input: &str) -> i32 {
    let mut invec: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        invec.push(line.chars().collect());
    }

    let rows = invec.len();
    let cols = invec[0].len();

    let mut sum: i32 = 0;
    for i in 0..rows {
        for k in 0..cols {
            sum += check_for_xmas(&invec, i, k);
        }
    }
    sum
}

fn main() {
    let input = include_str!("../../../day04/input1.txt");
    let distance = process(input);
    println!("The result is {}", distance);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let check = include_str!("../../../day04/check1.txt");
        assert_eq!(process(check), 18)
    }
}
