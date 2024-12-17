use std::fs;

pub fn solve() {
    let file_name = "input/day13.txt";
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");
    let mut lines = contents.lines();
    let n = lines.clone().count();
    let mut i = 0;

    let mut sum = 0;
    while i < n {
        let eq_one = lines.next().unwrap();
        let eq_two = lines.next().unwrap();
        let result = lines.next().unwrap();

        let (num1, num2) = parse_numbers(eq_one);
        let (num3, num4) = parse_numbers(eq_two);
        let (coff_a, coff_b) = parse_numbers(result);
        let coff1 = coff_a + 10000000000000;
        let coff2 = coff_b + 10000000000000;

        let det = num1 * num4 - num2 * num3;
        if det == 0 {
            println!("No unique solution exists for the system of equations.");
        } else {
            let a = (coff1 * num4) / det - (coff2 * num3) / det;
            let b = (num1 * coff2) / det - (num2 * coff1) / det;
            if a * num1 + b * num3 == coff1 && a * num2 + b * num4 == coff2 {
                println!("{} {}", coff_a, coff_b);
                println!("a = {}, b = {}", a, b);
                sum += 3 * a + b;
            }
        }

        lines.next();
        i += 4;
    }

    println!("Sum = {}", sum);
}

fn parse_numbers(line: &str) -> (i128, i128) {
    let eq_one = line.split(": ").collect::<Vec<&str>>();
    let numbers = eq_one[1].split(", ").collect::<Vec<&str>>();
    let number_one = numbers[0][2..].parse::<i128>().unwrap();
    let number_two = numbers[1][2..].parse::<i128>().unwrap();

    (number_one, number_two)
}
