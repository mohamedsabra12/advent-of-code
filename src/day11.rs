use std::collections::HashMap;
use std::fs;

pub fn solve() {
    let file_name = "input/day11.txt";
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");
    let mut nums: Vec<i64> = contents
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .map(|num| num.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut result = 0;

    let mut map: HashMap<(i64, i64), i64> = HashMap::new();

    for num in nums {
        print!("{} ", num);
        result += count(num, 75, &mut map);
    }

    println!("Result: {}", result);
}

fn count(num: i64, lvl: i64, map: &mut HashMap<(i64, i64), i64>) -> i64 {
    if lvl == 0 {
        return 1;
    }

    if map.contains_key(&(num, lvl)) {
        return *map.get(&(num, lvl)).unwrap();
    }

    let num_string = num.to_string();
    let num_length = num_string.len();

    let res;
    if num == 0 {
        res = count(1, lvl - 1, map);
    } else if num_length % 2 == 0 {
        let first_half = &num_string[0..num_length / 2];
        let second_half = &num_string[num_length / 2..];

        let first_num = first_half.parse::<i64>().unwrap();
        let second_num = second_half.parse::<i64>().unwrap();

        res = count(first_num, lvl - 1, map) + count(second_num, lvl - 1, map);
    } else {
        res = count(num * 2024, lvl - 1, map);
    }

    map.insert((num, lvl), res);
    return res;
}
