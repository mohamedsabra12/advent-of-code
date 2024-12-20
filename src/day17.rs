use std::fs;

pub fn solve() {
    let file_name = "input/day17.txt";
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");
    let mut lines = contents.lines();

    let line_one = lines.next().unwrap();
    let line_two = lines.next().unwrap();
    let line_three = lines.next().unwrap();

    let mut A = get_register_value(line_one);
    let mut B = get_register_value(line_two);
    let mut C = get_register_value(line_three);

    lines.next(); // skip the empty line

    let program: Vec<usize> = lines.next().unwrap().split(": ").collect::<Vec<&str>>()[1]
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    let mut instruction_pointer: usize = 0;
    let mut result = vec![];

    while instruction_pointer < program.len() {
        let opcode = program[instruction_pointer];
        let operand = program[instruction_pointer + 1];

        match opcode {
            0 => {
                A = A / 2_usize.pow(get_comp_operand(operand, A, B, C) as u32);
            }
            1 => B ^= operand,
            2 => B = get_comp_operand(operand, A, B, C) % 8,
            3 => {
                if A != 0 {
                    instruction_pointer = operand;
                    continue;
                }
            }
            4 => {
                B ^= C;
            }
            5 => result.push(get_comp_operand(operand, A, B, C) % 8),
            6 => {
                B = A / 2_usize.pow(get_comp_operand(operand, A, B, C) as u32);
            }
            7 => {
                C = A / 2_usize.pow(get_comp_operand(operand, A, B, C) as u32);
            }
            _ => {
                println!("Invalid opcode: {}", opcode);
            }
        }

        instruction_pointer += 2;
    }

    let res: String = result
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",");
    println!("Result: {}", res);
}

fn get_register_value(line: &str) -> usize {
    let parts: Vec<&str> = line.split(": ").collect();
    parts[1].parse::<usize>().unwrap()
}

fn get_comp_operand(operand: usize, A: usize, B: usize, C: usize) -> usize {
    if operand <= 3 {
        operand
    } else if operand == 4 {
        A
    } else if operand == 5 {
        B
    } else {
        C
    }
}
