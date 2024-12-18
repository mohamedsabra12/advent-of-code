use std::char;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::i32::MAX;

const DX: [i32; 4] = [0, -1, 0, 1];
const DY: [i32; 4] = [-1, 0, 1, 0];

#[derive(Eq, PartialEq)]
struct Node {
    i: i32,
    j: i32,
    d: i32,
    sum: i32,
    set: HashSet<(i32, i32)>,
}
// Implement Ord for Task

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.sum.cmp(&self.sum)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
pub fn solve() {
    let file_name = "input/day16.txt";
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");
    let mut lines = contents.lines();

    let mut arr: Vec<Vec<char>> = Vec::new();

    let mut start: (i32, i32) = (0, 0);
    let mut end: (i32, i32) = (0, 0);

    for (i, line) in lines.by_ref().enumerate() {
        if line.is_empty() {
            break;
        }

        let chars: Vec<char> = line.chars().collect();

        for (j, c) in chars.iter().enumerate() {
            if *c == 'S' {
                start = (i as i32, j as i32);
            }

            if *c == 'E' {
                end = (i as i32, j as i32);
            }
        }

        arr.push(chars);
    }

    let res = dijkstra(start, end, &mut arr);

    println!("Result: {}", res);
}

fn dijkstra(start: (i32, i32), end: (i32, i32), arr: &mut Vec<Vec<char>>) -> i32 {
    let mut heap = BinaryHeap::new();
    let mut node = Node {
        i: start.0,
        j: start.1,
        d: 2,
        sum: 0,
        set: HashSet::new(),
    };

    node.set.insert((node.i, node.j));

    heap.push(node);

    let mut min_dist = HashMap::new();

    let mut final_set = HashSet::new();

    let mut min_sum = MAX;

    while !heap.is_empty() {
        let node = heap.pop().unwrap();

        let key = format!("{},{},{}", node.i, node.j, node.d);

        if min_dist.contains_key(&key) && min_dist[&key] < node.sum {
            continue;
        }

        min_dist.insert(key, node.sum);

        if node.i == end.0 && node.j == end.1 {
            println!("Found path: {}", node.sum);
            if node.sum <= min_sum {
                min_sum = node.sum;
                for (i, j) in node.set.iter() {
                    final_set.insert((*i, *j));
                }
            } else {
                return final_set.len() as i32;
            }
            continue;
        }

        let dirs = [(node.d + 3) % 4, node.d, (node.d + 1) % 4];

        for dir in dirs {
            let ni = node.i + DX[dir as usize];
            let nj = node.j + DY[dir as usize];

            if ni < 0
                || nj < 0
                || ni >= arr.len() as i32
                || nj >= arr[0].len() as i32
                || arr[ni as usize][nj as usize] == '#'
            {
                continue;
            }

            let cnt = 1 + if dir == node.d { 0 } else { 1000 };

            let mut set = HashSet::new();

            for (i, j) in node.set.iter() {
                set.insert((*i, *j));
            }
            set.insert((ni, nj));
            heap.push(Node {
                i: ni,
                j: nj,
                d: dir,
                sum: node.sum + cnt,
                set,
            });
        }
    }
    final_set.len() as i32
}
