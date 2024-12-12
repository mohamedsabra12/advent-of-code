use std::fs;

const DX: [i32; 8] = [1, -1, 0, 0, 1, 1, -1, -1];
const DY: [i32; 8] = [0, 0, 1, -1, 1, -1, 1, -1];

pub fn part_one() {
    let file_name = "input/day4.txt";
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");
    let lines: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut sum = 0;
    let word: Vec<char> = "XMAS".chars().collect();

    for i in 0..lines.len() {
        for j in 0..lines[0].len() {
            for k in 0..8 {
                let mut sx = i as i32;
                let mut sy = j as i32;
                let mut found = true;

                for idx in 0..4 {
                    if sx < 0 || sy < 0 || sx >= lines.len() as i32 || sy >= lines[0].len() as i32 {
                        found = false;
                        break;
                    }

                    if word[idx as usize] != lines[sx as usize][sy as usize] {
                        found = false;
                        break;
                    }

                    sx += DX[k];
                    sy += DY[k];
                }

                if found {
                    sum += 1;
                }
            }
        }
    }
    println!("Sum: {}", sum);
}
pub fn part_two() {
    let file_name = "input/day4.txt";
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");
    let lines: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut sum = 0;

    for i in 0..lines.len() {
        for j in 0..lines[0].len() {
            if lines[i][j] == 'A'
                && i as i32 > 0
                && j as i32 > 0
                && i + 1 < lines.len()
                && j + 1 < lines[0].len()
            {
                let left = lines[i - 1][j - 1] == 'M' && lines[i + 1][j + 1] == 'S'
                    || lines[i - 1][j - 1] == 'S' && lines[i + 1][j + 1] == 'M';
                let right = lines[i - 1][j + 1] == 'M' && lines[i + 1][j - 1] == 'S'
                    || lines[i - 1][j + 1] == 'S' && lines[i + 1][j - 1] == 'M';

                if left && right {
                    sum += 1;
                }
            }
        }
    }
    println!("Sum: {}", sum);
}
