use std::collections::HashMap;
use std::fs;

pub fn solve() {
    let file_name = "input/day19.txt";
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");
    let mut lines = contents.lines();
    let mut patterns: Vec<&str> = lines.next().unwrap().split(',').map(|x| x.trim()).collect();
    println!("Patterns: {:?}", patterns);

    lines.next();

    let mut sum: i64 = 0;

    let mut map = HashMap::new();

    for line in lines {
        // println!("Line: {}", line);
        let count = count_comb(0, &patterns, line, &mut map);

        sum += count;
    }

    println!("Part 2: {}", sum);
}

fn count_comb(i: i32, patterns: &Vec<&str>, line: &str, map: &mut HashMap<String, i64>) -> i64 {
    if i == line.len() as i32 {
        return 1;
    }

    if i > line.len() as i32 {
        return 0;
    }

    let start_i = i as usize;
    let substr = &line[start_i..].to_string();

    if map.contains_key(substr) {
        return map[substr];
    }

    let mut count = 0;
    for pattern in patterns {
        // println!("Pattern: {}", *pattern);
        if substr.starts_with(*pattern) {
            // println!("{} {} {}", i, pattern, line);
            count += count_comb(i + pattern.len() as i32, patterns, line, map);
        }
    }

    map.insert(substr.to_string(), count);
    count
}
