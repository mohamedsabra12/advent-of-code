use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
pub fn solve() {
    let file_name = "input/day5.txt";
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");
    let mut lines = contents.lines();
    let mut arr: [[Option<bool>; 100]; 100] = [[Some(false); 100]; 100];

    for i in 0..100 {
        arr[i][i] = Some(true);
    }

    for line in &mut lines {
        if line.is_empty() {
            break;
        }
        let split: Vec<&str> = line.split('|').collect();
        let x: usize = split[0].parse().unwrap();
        let y: usize = split[1].parse().unwrap();
        arr[x][y] = Some(true);
    }

    let mut sum: i32 = 0;
    for line in lines {
        let split: Vec<&str> = line.split(",").collect();
        let mut map: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut valid = true;

        for i in 0..split.len() {
            let x: usize = split[i].parse().unwrap();
            map.entry(x).or_insert_with(Vec::new);
        }

        for i in 0..split.len() {
            let x: usize = split[i].parse().unwrap();
            for j in (i + 1)..split.len() {
                let y: usize = split[j].parse().unwrap();
                if is_there_a_away(y, x, &mut arr) {
                    map.entry(y).or_insert_with(Vec::new).push(x);
                    valid = false;
                } else if is_there_a_away(x, y, &mut arr) {
                    map.entry(x).or_insert_with(Vec::new).push(y);
                }
            }
        }

        let mut visited = HashSet::new();
        let mut result = Vec::new();
        for x in map.keys() {
            dfs_sorted(*x, &map, &mut visited, &mut result);
        }

        if !valid {
            let mid: usize = result[result.len() / 2];
            sum += mid as i32;
        }
    }

    println!("{}", sum);
}

fn is_there_a_away(i: usize, j: usize, arr: &mut [[Option<bool>; 100]; 100]) -> bool {
    if i == j {
        return true;
    }

    if let Some(val) = arr[i][j] {
        return val;
    }

    for k in 0..100 {
        if is_there_a_away(i, k, arr) && is_there_a_away(k, j, arr) {
            arr[i][j] = Some(true);
            return true;
        }
    }

    arr[i][j] = Some(false);
    false
}

fn dfs_sorted(
    node: usize,
    graph: &HashMap<usize, Vec<usize>>,
    visited: &mut HashSet<usize>,
    result: &mut Vec<usize>,
) {
    if visited.contains(&node) {
        return;
    }

    visited.insert(node);

    for child in graph.get(&node).unwrap() {
        dfs_sorted(*child, graph, visited, result);
    }

    result.insert(0, node);
}
