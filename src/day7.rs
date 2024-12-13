use std::fs;

pub fn solve() {
    let file_name = "input/day7.txt";
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");
    let lines = contents.lines();

    let mut count = 0;
    for line in lines {
        let parts: Vec<&str> = line.split(":").collect();
        let sum = parts[0].parse::<i64>().unwrap();
        let numbers: Vec<i64> = parts[1]
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();

        let result = calculate((numbers.len() - 1) as i32, sum, &numbers);

        if result {
            count += sum;
        }
    }

    println!("Count: {}", count);
}

fn calculate(i: i32, rem: i64, numbers: &Vec<i64>) -> bool {
    if i == 0 {
        return numbers[0] == rem;
    }

    let shift = numbers[i as usize].to_string().len() as i64;
    let shift_div: i64 = 10_i64.pow(shift as u32);

    let mut found = false;

    found |= calculate(i - 1, rem - numbers[i as usize], numbers);

    if rem % numbers[i as usize] == 0 {
        found |= calculate(i - 1, rem / numbers[i as usize], numbers);
    }
    if rem % shift_div == numbers[i as usize] {
        found |= calculate(i - 1, rem / shift_div, numbers);
    }

    found
}
