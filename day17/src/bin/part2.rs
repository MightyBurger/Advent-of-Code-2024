struct ProgramParams {
    rega: i64,
    regb: i64,
    regc: i64,
    program: Vec<u8>,
}

//use itertools::Itertools;
fn combo(num: u8, a: i64, b: i64, c: i64) -> i64 {
    match num {
        0..=3 => num as i64,
        4 => a,
        5 => b,
        6 => c,
        7.. => {
            panic!("Invalid combo")
        }
    }
}

fn run(mut a: i64, program: &Vec<u8>) -> Vec<i64> {
    let mut b = 0;
    let mut c = 0;
    let mut ip = 0;

    let mut output: Vec<i64> = Vec::new();

    while ip < program.len() - 1 {
        let opcode = program[ip];
        let operand = program[ip + 1];

        match opcode {
            0 => {
                let num = a;
                let denom = i64::pow(2, combo(operand, a, b, c) as u32);
                a = num / denom;
                ip += 2;
            }
            1 => {
                b = b ^ operand as i64;
                ip += 2;
            }
            2 => {
                b = combo(operand, a, b, c) & 0x7;
                ip += 2;
            }
            3 => {
                if a != 0 {
                    ip = operand as usize;
                } else {
                    ip += 2;
                }
            }
            4 => {
                b = b ^ c;
                ip += 2;
            }
            5 => {
                output.push(combo(operand, a, b, c) & 0x7);
                ip += 2;
            }
            6 => {
                let num = a;
                let denom = i64::pow(2, combo(operand, a, b, c) as u32);
                b = num / denom;
                ip += 2;
            }
            7 => {
                let num = a;
                let denom = i64::pow(2, combo(operand, a, b, c) as u32);
                c = num / denom;
                ip += 2;
            }
            8.. => {
                panic!("Invalid opcode");
            }
        }
    }

    output
}

fn coeff_to_a(coefficients: &Vec<i64>) -> i64 {
    let mut sum = 0;

    for (i, coeff) in coefficients.iter().enumerate() {
        sum += 8i64.pow(i as u32) * coeff;
    }

    sum
}

fn process(program: &Vec<u8>) -> i64 {
    let power = program.len();
    let mut coefficients: Vec<i64> = vec![0; power];
    coefficients[power - 1] = 1;

    dbg!(power);

    for i in (0..power).rev() {
        'find_coeff: loop {
            let a = coeff_to_a(&coefficients);
            let result = run(a, program);
            let mut matches = true;
            for k in i..power {
                if result[k] != program[k] as i64 {
                    matches = false;
                }
            }
            if matches {
                break 'find_coeff;
            }
            coefficients[i] = coefficients[i] + 1;
        }
        println!("Determined c{i} to be {}", coefficients[i]);
        println!("Targeting: {:?}", program);
        println!("   So far: {:?}", run(coeff_to_a(&coefficients), program));
        print!("            ");
        for _ in 0..(3 * i) {
            print!(" ");
        }
        println!("^");
    }

    coeff_to_a(&coefficients)
}

fn main() {
    let program = vec![2, 4, 1, 5, 7, 5, 1, 6, 4, 2, 5, 5, 0, 3, 3, 0];
    let result = process(&program);
    println!("The result is {}", result);
}

// ----------------------------------------------------
// -------------------- Unit Tests --------------------
// ----------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let program = vec![0, 1, 5, 4, 3, 0];
        let result = process(&program);
        dbg!(&result);
        assert_eq!(result, 117440);
    }
}
