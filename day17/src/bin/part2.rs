use itertools::Itertools;

fn literal(num: u8) -> &'static str {
    match num {
        0 => "#0",
        1 => "#1",
        2 => "#2",
        3 => "#3",
        4 => "#4",
        5 => "#5",
        6 => "#6",
        7 => "#7",
        8.. => panic!("Invalid literal"),
    }
}

fn combo(num: u8) -> &'static str {
    match num {
        0 => "#0",
        1 => "#1",
        2 => "#2",
        3 => "#3",
        4 => "rA",
        5 => "rB",
        6 => "rC",
        7.. => panic!("Invalid combo literal"),
    }
}

fn main() {
    let program: Vec<u8> = vec![2, 4, 1, 5, 7, 5, 1, 6, 4, 2, 5, 5, 0, 3, 3, 0];

    for (opcode, operand) in program.into_iter().tuples() {
        match opcode {
            0 => println!("rA <= rA / 2 ^ {}", combo(operand)),
            1 => println!("rB <= rB xor {}", literal(operand)),
            2 => println!("rB <= {} mod 8", combo(operand)),
            3 => println!("ip <= {} if A /= 0", literal(operand)),
            4 => println!("rB <= rB xor rC"),
            5 => println!("print {} mod 8", combo(operand)),
            6 => println!("rB <= rA / 2 ^ {}", combo(operand)),
            7 => println!("rC <= rA / 2 ^ {}", combo(operand)),
            8.. => panic!("Invalid opcode"),
        }
    }
}
