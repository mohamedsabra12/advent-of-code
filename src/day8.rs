use std::char;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

pub fn solve() {
    let file_name = "input/day8.txt";
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");
    let lines = contents.lines();
    let n = lines.clone().count();
    let mut map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    // print!("N: {}", n);

    for (i, line) in lines.enumerate() {
        let row: Vec<char> = line.chars().collect();
        for (j, ch) in row.iter().enumerate() {
            if *ch != '.' {
                let mut v = map.get(ch).unwrap_or(&vec![]).to_vec();
                v.push((i as i32, j as i32));
                map.insert(*ch, v);
            }
        }
    }

    let mut set: HashSet<(i32, i32)> = HashSet::new();

    for (_, v) in map.iter() {
        for (i, pair) in v.iter().enumerate() {
            for j in i + 1..v.len() {
                let (mut x1, mut y1) = pair;
                let (mut x2, mut y2) = &v[j];

                if x1 == x2 && y1 == y2 {
                    continue;
                }

                let diff_x = x1 - x2;
                let diff_y = y1 - y2;

                set.insert((x1, y1));
                set.insert((x2, y2));

                while is_valid(x1 + diff_x, y1 + diff_y, n as i32) {
                    set.insert((x1 + diff_x, y1 + diff_y));
                    x1 += diff_x;
                    y1 += diff_y;
                }

                while is_valid(x2 - diff_x, y2 - diff_y, n as i32) {
                    set.insert((x2 - diff_x, y2 - diff_y));
                    x2 -= diff_x;
                    y2 -= diff_y;
                }
            }
        }
    }

    println!("Cnt {}", set.len());
}

fn is_valid(i: i32, j: i32, n: i32) -> bool {
    i >= 0 && i < n && j >= 0 && j < n
}
