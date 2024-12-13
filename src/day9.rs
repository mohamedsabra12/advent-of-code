use std::fs;

pub fn solve() {
    let file_name = "input/day9.txt";
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");
    let lines = contents.lines();
    let mut prog: String = String::new();
    for line in lines {
        prog.push_str(line);
    }

    let mut id = 0;
    let mut idx = 0;
    let numbers: Vec<i64> = prog
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect();

    let mut freq = Vec::new();

    for (i, num) in numbers.iter().enumerate() {
        if i % 2 == 0 {
            freq.push((idx, *num, id));
            id += 1;
        }

        idx += num;
    }

    println!("{:?}", freq);

    let mut i = 0;
    let mut j = freq.len() - 1;
    idx = 0;

    let mut sum: i64 = 0;
    while i <= j {
        println!("{}, {}, {}", i, j, idx);
        let start = freq[i];
        let end = freq[j];

        if idx < start.0 + start.1 && idx >= start.0 {
            sum += idx * start.2;

            if idx + 1 == start.0 + start.1 {
                i += 1;
            }
        } else {
            sum += idx * end.2;
            freq[j].1 -= 1;

            if freq[j].1 == 0 {
                j -= 1;
            }
        }

        idx += 1;
    }

    println!("Sum: {}", sum);
}

pub fn part_two() {
    let file_name = "input/day9.txt";
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");
    let lines = contents.lines();
    let mut prog: String = String::new();
    for line in lines {
        prog.push_str(line);
    }

    let mut id = 0;
    let mut idx = 0;
    let numbers: Vec<i64> = prog
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect();

    let mut freq = Vec::new();
    let mut spaces = Vec::new();

    for (i, num) in numbers.iter().enumerate() {
        if i % 2 == 0 {
            freq.push((idx, *num, id));
            id += 1;
        } else {
            spaces.push((idx, *num));
        }

        idx += num;
    }


    for i in (0..(freq.len())).rev() {

        let (start, num, id) = freq[i];
        let mut j = 0;
        while j < spaces.len() {
            let (space_idx, space_capacity) = spaces[j];
            if space_capacity >= num && start > space_idx {
                spaces[j].1 -= num;
                spaces[j].0 += num;
                freq[i] = (space_idx, num, id);
                break;
            }
            j += 1
        }
    }

    let mut sum = 0;

    for item in freq {
        let (idx, num, id) = item;

        for i in 0..num {
            sum += (idx + i) * id;
        }
    }

    println!("{}", sum);
}
