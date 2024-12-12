use std::fs;

pub fn solve() {
    let file_name = "input/day2.txt";
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");
    let lines = contents.lines();

    let mut sum = 0;
    for line in lines {
        let line_vec: Vec<&str> = line.split_whitespace().collect();
        let line_nums: Vec<i32> = line_vec.into_iter().map(|x| x.parse().unwrap()).collect();
        println!("{:?}", line_nums);

        if check_row(&line_nums) {
            sum += 1;
        } else {
            for idx in 0..line_nums.len() {
                let new_arr: Vec<i32> = line_nums
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i != idx)
                    .map(|(_, x)| x)
                    .cloned()
                    .collect();

                println!("{:?}", new_arr);
                if check_row(&new_arr) {
                    sum += 1;
                    break;
                }
            }
        }
    }

    println!("{}", sum);
}

fn check_row(row: &[i32]) -> bool {
    let diff_val = row[1] - row[0];
    let diff_dir = diff_val > 0;

    for i in 1..row.len() {
        let diff_val = row[i] - row[i - 1];
        let cur_diff_dir = diff_val > 0;

        if cur_diff_dir != diff_dir || diff_val.abs() < 1 || diff_val.abs() > 3 {
            return false;
        }
    }

    true
}
