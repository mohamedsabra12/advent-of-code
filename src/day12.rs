use std::collections::HashSet;
use std::fs;
pub fn solve() {
    let file_name = "input/day12.txt";
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");
    let lines = contents.lines();

    let mut arr: Vec<Vec<char>> = Vec::new();

    for line in lines {
        let chars = line.to_string().chars().collect::<Vec<char>>();
        arr.push(chars);
    }

    let mut result = 0;

    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    for i in 0..arr.len() {
        for j in 0..arr[i].len() {
            if !visited.contains(&(i as i32, j as i32)) {
                let mut cells: HashSet<(i32, i32)> = HashSet::new();
                count_cells(
                    i as i64,
                    j as i64,
                    arr[i][j],
                    &mut arr,
                    &mut visited,
                    &mut cells,
                );
                result += calc_all(&mut arr, cells);
            }
        }
    }
    println!("Result: {}", result);
}

const DX: [i64; 4] = [0, 0, 1, -1];
const DY: [i64; 4] = [1, -1, 0, 0];

fn count_cells(
    i: i64,
    j: i64,
    c: char,
    arr: &mut Vec<Vec<char>>,

    visited: &mut HashSet<(i32, i32)>,
    cells: &mut HashSet<(i32, i32)>,
) {
    if i < 0
        || j < 0
        || i >= arr.len() as i64
        || j >= arr[0].len() as i64
        || arr[i as usize][j as usize] != c
    {
        return;
    }

    if visited.contains(&(i as i32, j as i32)) {
        return;
    }

    visited.insert((i as i32, j as i32));
    cells.insert((i as i32, j as i32));

    for k in 0..4 {
        count_cells(i + DX[k], j + DY[k], c, arr, visited, cells);
    }
}

fn calc_all(arr: &mut Vec<Vec<char>>, cells: HashSet<(i32, i32)>) -> i32 {
    let mut result = 0;
    let mut count = 0;
    for i in 0..arr.len() {
        let mut above_values = Vec::new();
        let mut below_values = Vec::new();

        for j in 0..arr[i].len() {
            if !cells.contains(&(i as i32, j as i32)) {
                continue;
            }
            count += 1;
            if i == 0 || arr[i - 1][j] != arr[i][j] {
                above_values.push(j);
            }

            if i + 1 == arr.len() || arr[i + 1][j] != arr[i][j] {
                below_values.push(j);
            }
        }

        result += sum_values(above_values, below_values);
    }

    for j in 0..arr[0].len() {
        let mut above_values = Vec::new();
        let mut below_values = Vec::new();

        for i in 0..arr.len() {
            if !cells.contains(&(i as i32, j as i32)) {
                continue;
            }

            if j == 0 || arr[i][j - 1] != arr[i][j] {
                above_values.push(i);
            }

            if j + 1 == arr[0].len() || arr[i][j + 1] != arr[i][j] {
                below_values.push(i);
            }
        }

        result += sum_values(above_values, below_values);
    }

    result * count
}

fn sum_values(above_values: Vec<usize>, below_values: Vec<usize>) -> i32 {
    let mut result = 0;
    let mut up_cnt = 0;
    let mut down_cnt = 0;

    for i in 0..above_values.len() {
        if i == 0 {
            up_cnt += 1;
            continue;
        }
        if above_values[i] - above_values[i - 1] != 1 {
            up_cnt += 1;
        }
    }

    for i in 0..below_values.len() {
        if i == 0 {
            down_cnt += 1;
            continue;
        }
        if below_values[i] - below_values[i - 1] != 1 {
            down_cnt += 1;
        }
    }

    result += up_cnt;
    result += down_cnt;

    result
}
