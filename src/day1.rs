// (c) Meta Platforms, Inc. and affiliates. Confidential and proprietary.

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

pub fn solve() {
    let file_name = "input/day1.txt";
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");
    let lines = contents.lines();
    let mut loc_a = HashSet::new();
    let mut loc_b = HashMap::new();

    for line in lines {
        let loc: Vec<&str> = line.split_whitespace().collect();
        loc_a.insert(loc[0]);
        *loc_b.entry(loc[1]).or_insert(0) += 1;
    }

    let mut sum = 0;

    for loc in loc_a {
        if loc_b.contains_key(loc) {
            let val: i32 = loc.parse().unwrap();
            sum += val * loc_b[loc];
        }
    }

    println!("{}", sum);
}
