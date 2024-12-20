use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;

pub fn solve() {
    let file_name = "input/day18.txt";
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");
    let mut lines = contents.lines();

    let mut arr = [['.'; 71]; 71];

    for _ in 0..1024 {
        let line = lines.next().unwrap();
        let parts: Vec<&str> = line.split(",").collect();
        let y = parts[0].parse::<usize>().unwrap();
        let x = parts[1].parse::<usize>().unwrap();
        arr[x][y] = '#';
    }

    for line in lines {
        let parts: Vec<&str> = line.split(",").collect();
        let y = parts[0].parse::<usize>().unwrap();
        let x = parts[1].parse::<usize>().unwrap();
        arr[x][y] = '#';

        let path = bfs(arr);
        if path == -1 {
            println!("No path found {},{}", y, x);
            return;
        }
    }
}

fn bfs(arr: [[char; 71]; 71]) -> i32 {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((0, 0));
    visited.insert((0, 0));

    let mut path = 0;

    let dx: [i32; 4] = [-1, 1, 0, 0];
    let dy: [i32; 4] = [0, 0, -1, 1];

    while !queue.is_empty() {
        let s = queue.len();

        for i in 0..s {
            let (x, y) = queue.pop_front().unwrap();

            if x == arr.len() - 1 && y == arr[0].len() - 1 {
                return path;
            }

            for j in 0..4 {
                let nx = x as i32 + dx[j];
                let ny = y as i32 + dy[j];

                if nx < 0
                    || nx >= arr.len() as i32
                    || ny < 0
                    || ny >= arr[0].len() as i32
                    || arr[nx as usize][ny as usize] != '.'
                    || visited.contains(&(nx, ny))
                {
                    continue;
                }

                visited.insert((nx, ny));
                queue.push_back((nx as usize, ny as usize));
            }
        }

        path += 1;
    }

    return -1;
}
