use std::collections::BTreeMap;
use std::fs;

use regex::Regex;
pub fn solve() {
    let file_name = "input/day3.txt";
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");
    let lines = contents.lines();
    let pattern = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let do_string = "do()";
    let dont_string = "don't()";
    let mut sum = 0;

    let mut prog: String = String::new();
    for line in lines {
        prog.push_str(line);
    }

    let do_occurrences: Vec<usize> = prog.match_indices(do_string).map(|entry| entry.0).collect();
    let do_not_occurrences: Vec<usize> = prog
        .match_indices(dont_string)
        .map(|entry| entry.0)
        .collect();
    let mut tree_map = BTreeMap::new();
    for occur in do_occurrences {
        tree_map.insert(occur, 1);
    }
    for occur in do_not_occurrences {
        tree_map.insert(occur, 0);
    }
    let matches: Vec<(usize, &str)> = pattern
        .find_iter(&prog)
        .map(|mat| (mat.start(), mat.as_str()))
        .collect();
    for tuple in matches {
        let idx = tuple.0;
        let matched_str: &str = tuple.1;
        let input = matched_str.replace("mul(", "").replace(")", "");
        let numbers: Vec<&str> = input.split(',').collect();
        let num1: i32 = numbers[0].parse().expect("Failed to parse num1");
        let num2: i32 = numbers[1].parse().expect("Failed to parse num2");
        let floor: Option<(&usize, &i32)> = tree_map.range(..idx).next_back();
        match floor {
            Some((_, &value)) => {
                if value == 1 {
                    sum += num1 * num2;
                }
            }
            None => {
                sum += num1 * num2;
            }
        }
    }
    println!("Sum: {}", sum);
}
