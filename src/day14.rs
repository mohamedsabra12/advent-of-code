use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::{self};

pub fn solve() {
    let file_name = "input/day14.txt";
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");
    let lines = contents.lines();

    let m = 101;
    let n = 103;

    let mut arr = [['.'; 101]; 103];

    let mut vec: Vec<(i32, i32, i32, i32)> = Vec::new();

    let mut file = File::create("output.txt").unwrap();

    let mut set: HashSet<(i32, i32)> = HashSet::new();

    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let pos: &str = parts[0].split("=").collect::<Vec<&str>>()[1];
        let pos_nums = pos.split(",").collect::<Vec<&str>>();
        let y = pos_nums[0].parse::<i32>().unwrap();
        let x = pos_nums[1].parse::<i32>().unwrap();
        let dir = parts[1].split("=").collect::<Vec<&str>>()[1];
        let dir_nums = dir.split(",").collect::<Vec<&str>>();
        let dir_y = dir_nums[0].parse::<i32>().unwrap();
        let dir_x = dir_nums[1].parse::<i32>().unwrap();

        vec.push((x, y, dir_x, dir_y));
    }

    let mut count = 0;

    for i in 0..10000 {
        set = HashSet::new();
        for (x, y, dir_x, dir_y) in &mut vec {
            let new_x = (*x + n + *dir_x) % n;
            let new_y = (*y + m + *dir_y) % m;

            arr[*x as usize][*y as usize] = '.';
            arr[new_x as usize][new_y as usize] = 'X';

            *x = new_x;
            *y = new_y;

            set.insert((new_x, new_y));
        }

        if set.len() == vec.len() {
            count += 1;
            println!("{}", i + 1);
            for i in 0..n {
                for j in 0..m {
                    write!(file, "{}", arr[i as usize][j as usize]).unwrap();
                }
                writeln!(file).unwrap();
            }

            writeln!(file).unwrap();
        }
    }

    println!("count {}", count);
}

fn count(startx: i32, endx: i32, starty: i32, endy: i32, arr: [[i32; 101]; 103]) -> i32 {
    let mut sum = 0;
    for i in startx..endx {
        for j in starty..endy {
            print!("{},{} ", i, j);
            sum += arr[i as usize][j as usize];
        }
        println!();
    }
    sum
}
