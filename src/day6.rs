use std::collections::HashSet;
use std::fs;

pub fn part_one() {
    let file_name = "input/day6.txt";
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");
    let lines = contents.lines();
    let n = lines.clone().count() as i32;
    let mut arr = [[0; 130]; 130];
    let mut start = (0, 0);


    for (i, line) in lines.enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '.' {
                arr[i][j] = 1;
            } else if c == '#' {
                arr[i][j] = 2;
            } else {
                start = (i as i32, j as i32);
            }
        }
    }

    let dx: [i32; 4] = [-1, 0, 1, 0];
    let dy: [i32; 4] = [0, 1, 0, -1];
    let mut dir: usize = 0;

    let mut i: i32 = start.0;
    let mut j: i32 = start.1;

    let mut sum = 0;
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    while i >= 0 && j >= 0 && i <= n - 1 && j <= n - 1 {
        if arr[i as usize][j as usize] == 2 {
            i -= dx[dir];
            j -= dy[dir];
            dir = (dir + 1) % 4;
            continue;
        }
        if !visited.contains(&(i, j)) {
            visited.insert((i, j));
            sum += 1;
        }

        visited.insert((i, j));

        i += dx[dir];
        j += dy[dir];
    }

    println!("Sum: {}", sum);
}

pub fn part_two() {
    let file_name = "input/day6.txt";
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");
    let lines = contents.lines();
    let n = lines.clone().count() as i32;
    let mut arr = [[0; 130]; 130];
    let mut start = (0, 0);

    for (i, line) in lines.enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '.' {
                arr[i][j] = 1;
            } else if c == '#' {
                arr[i][j] = 2;
            } else {
                start = (i as i32, j as i32);
            }
        }
    }

    let dx: [i32; 4] = [-1, 0, 1, 0];
    let dy: [i32; 4] = [0, 1, 0, -1];
    let mut sum = 0;

    for x in 0..arr.len() {
        for y in 0..arr[0].len() {
            let c = arr[x][y];

            if c != 1 {
                continue;
            }

            arr[x][y] = 2;

            let mut i = start.0;
            let mut j = start.1;

            let mut seen: HashSet<(i32, i32, i32)> = HashSet::new();
            let mut dir: usize = 0;

            let mut is_loop = false;
            while i >= 0 && j >= 0 && i <= n - 1 && j <= n - 1 {
                if arr[i as usize][j as usize] == 2 {
                    i -= dx[dir];
                    j -= dy[dir];
                    dir = (dir + 1) % 4;
                    continue;
                }

                if seen.contains(&(i, j, dir as i32)) {
                    is_loop = true;
                    break;
                }

                seen.insert((i, j, dir as i32));

                i += dx[dir];
                j += dy[dir];
            }

            if is_loop {
                sum += 1;
            }

            arr[x][y] = 1;
        }
    }

    println!("Sum: {}", sum);
}
