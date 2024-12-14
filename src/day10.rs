use std::fs;

pub fn solve() {
    let file_name = "input/day10.txt";
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");
    let lines = contents.lines();
    let mut arr = Vec::new();

    for line in lines {
        let line_vec: Vec<i32> = line.chars().map(|c| (c as i32) - ('0' as i32)).collect();
        arr.push(line_vec);
    }

    let mut count = 0;
    for i in 0..arr.len() {
        for j in 0..arr[0].len() {
            if arr[i][j] == 0 {
                count_path(i as i32, j as i32, &arr, &mut count, -1);
            }
        }
    }

    println!("Count: {count}")
}

const DX: [i32; 4] = [0, 0, 1, -1];
const DY: [i32; 4] = [1, -1, 0, 0];

fn count_path(i: i32, j: i32, arr: &Vec<Vec<i32>>, count: &mut i32, prev_val: i32) {
    if i < 0
        || j < 0
        || i >= arr.len() as i32
        || j >= arr[0].len() as i32
        || arr[i as usize][j as usize] != prev_val + 1
    {
        return;
    }

    if arr[i as usize][j as usize] == 9 {
        // println!("{} {}", i, j);

        *count += 1;
    }

    for k in 0..4 {
        let new_i = i + DX[k];
        let new_j = j + DY[k];

        count_path(new_i, new_j, arr, count, arr[i as usize][j as usize]);
    }
}
