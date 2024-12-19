struct Input {
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

fn process(input: Input) -> String {
    let mut a = input.rega;
    let mut b = input.regb;
    let mut c = input.regc;
    let program = input.program;
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

    let too_long = output
        .iter()
        .map(|num| format!("{num},"))
        .collect::<String>();
    let mut iter = too_long.chars();

    iter.next_back();

    iter.collect()
}

fn main() {
    let user_in: i64 = std::env::args().nth(1).unwrap().parse().unwrap();
    let input = Input {
        rega: user_in,
        regb: 0,
        regc: 0,
        program: vec![2, 4, 1, 5, 7, 5, 1, 6, 4, 2, 5, 5, 0, 3, 3, 0],
    };
    let result = process(input);
    println!("The result is {}", result);
    println!("It should be  {}", "2,4,1,5,7,5,1,6,4,2,5,5,0,3,3,0");
    if result == String::from("2,4,1,5,7,5,1,6,4,2,5,5,0,3,3,0") {
        println!("SUCCESS!! {}", user_in);
    } else {
        println!("Not quite.");
    }
}
