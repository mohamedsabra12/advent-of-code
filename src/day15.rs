use std::char;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

pub fn solve() {
    let file_name = "input/day15.txt";
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");
    let mut lines = contents.lines();

    let mut arr: Vec<Vec<char>> = Vec::new();

    let mut map: HashMap<char, (i32, i32)> = HashMap::new();
    map.insert('<', (0, -1));
    map.insert('^', (-1, 0));
    map.insert('>', (0, 1));
    map.insert('v', (1, 0));

    let mut robot: (i32, i32) = (0, 0);
    for (i, line) in lines.by_ref().enumerate() {
        if line.is_empty() {
            break;
        }

        let chars: Vec<char> = line.chars().collect();

        let mut new_chars = Vec::new();

        for (j, c) in chars.iter().enumerate() {
            if *c == '@' {
                new_chars.push('@');
                robot = (i as i32, (new_chars.len() - 1) as i32);
                new_chars.push('.');
            } else if *c == '#' {
                new_chars.push('#');
                new_chars.push('#');
            } else if *c == '.' {
                new_chars.push('.');
                new_chars.push('.');
            } else {
                new_chars.push('[');
                new_chars.push(']');
            }
        }

        arr.push(new_chars);
    }

    for line in &mut lines {
        for char in line.chars() {
            let dir = map.get(&char).unwrap();
            let (start_x, start_y) = robot;

            if char == '>' || char == '<' {
                let can_move = move_robot_horizontal(start_x, start_y, dir.1, &mut arr);

                if can_move {
                    arr[start_x as usize][start_y as usize] = '.';
                    robot = (start_x, start_y + dir.1);
                    arr[robot.0 as usize][robot.1 as usize] = '@';
                }
            } else {
                let new_x = start_x + dir.0;
                let mut can_move = true;

                let val = arr[new_x as usize][start_y as usize];

                if val == '[' {
                    can_move = can_move_robot_vertical(new_x, start_y, dir.0, &arr);
                    if can_move {
                        move_robot_vertical(new_x, start_y, dir.0, &mut arr);
                    }
                } else if val == ']' {
                    can_move = can_move_robot_vertical(new_x, start_y - 1, dir.0, &arr);
                    if can_move {
                        move_robot_vertical(new_x, start_y - 1, dir.0, &mut arr);
                    }
                } else if val == '#' {
                    can_move = false;
                }

                if can_move {
                    arr[start_x as usize][start_y as usize] = '.';
                    robot = (new_x, start_y);
                    arr[robot.0 as usize][robot.1 as usize] = '@';
                }
            }
        }
    }

    let mut sum = 0;
    for i in 0..arr.len() {
        for j in 0..arr[i].len() {
            if arr[i][j] == '[' {
                sum += (100 * i + j);
            }
        }
    }

    println!("Sum: {}", sum);
}

fn can_move_robot_vertical(i: i32, j: i32, dir_x: i32, arr: &Vec<Vec<char>>) -> bool {
    let new_i = i + dir_x;

    if arr[new_i as usize][j as usize] == '.' && arr[new_i as usize][(j + 1) as usize] == '.' {
        return true;
    }

    if arr[new_i as usize][j as usize] == '#' || arr[new_i as usize][(j + 1) as usize] == '#' {
        return false;
    }

    let mut can_move = true;
    for k in (j - 1)..(j + 2) {
        if arr[new_i as usize][k as usize] == '[' {
            can_move &= can_move_robot_vertical(new_i, k, dir_x, arr);
        }
    }

    can_move
}

fn move_robot_vertical(i: i32, j: i32, dir_x: i32, arr: &mut Vec<Vec<char>>) {
    let new_i = i + dir_x;

    if arr[new_i as usize][j as usize] == '.' && arr[new_i as usize][(j + 1) as usize] == '.' {
        arr[i as usize][j as usize] = '.';
        arr[i as usize][(j + 1) as usize] = '.';

        arr[new_i as usize][j as usize] = '[';
        arr[new_i as usize][(j + 1) as usize] = ']';
        return;
    }

    for k in (j - 1)..(j + 2) {
        if arr[new_i as usize][k as usize] == '[' {
            move_robot_vertical(new_i, k, dir_x, arr);
        }
    }

    arr[new_i as usize][j as usize] = '[';
    arr[new_i as usize][(j + 1) as usize] = ']';

    arr[i as usize][j as usize] = '.';
    arr[i as usize][(j + 1) as usize] = '.';
}

fn move_robot_horizontal(i: i32, j: i32, dir_y: i32, arr: &mut Vec<Vec<char>>) -> bool {
    let mut new_j = j;

    let mut can_move = true;
    while new_j >= 0 || new_j < arr[0].len() as i32 {
        if arr[i as usize][new_j as usize] == '#' {
            can_move = false;
            break;
        }

        if arr[i as usize][new_j as usize] == '.' {
            can_move = true;
            break;
        }

        new_j += dir_y;
    }

    if !can_move {
        return false;
    }

    while new_j != j {
        arr[i as usize][new_j as usize] = arr[i as usize][(new_j - dir_y) as usize];
        new_j -= dir_y;
    }

    true
}
