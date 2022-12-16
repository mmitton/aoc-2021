const INPUT_FILE: &str = if cfg!(debug_assertions) {
    "../input-sample.txt"
} else {
    "../input.txt"
};

use std::collections::{BTreeMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
struct Room {
    flow_rate: usize,
    exits: Vec<String>,
    paths: BTreeMap<String, usize>,
}

fn get_path(rooms: &BTreeMap<String, Room>, from: &str, to: &str) -> Vec<String> {
    let mut work: VecDeque<Vec<String>> = VecDeque::new();

    work.push_front(vec![from.to_string()]);
    while let Some(path) = work.pop_front() {
        let room = rooms.get(&path[path.len() - 1]).unwrap();

        for exit in &room.exits {
            if path.contains(exit) {
                continue;
            }

            let mut new_path = path.clone();
            new_path.push(exit.clone());

            if exit == to {
                return new_path;
            }
            work.push_back(new_path);
        }
    }

    unreachable!();
}

fn main() {
    let lines: Vec<String> = {
        let file = File::open(INPUT_FILE).expect("Cannot open input file");
        BufReader::new(file).lines().flatten().collect()
    };

    let mut rooms = BTreeMap::new();
    let mut room_names = Vec::new();
    for line in lines {
        let line = line.replace('=', " ").replace([',', ';'], "");
        let parts: Vec<&str> = line.split(' ').collect();

        let mut room = Room {
            flow_rate: parts[5].parse().unwrap(),
            exits: Vec::new(),
            paths: BTreeMap::new(),
        };

        for exit in parts[10..].iter() {
            room.exits.push(exit.to_string());
        }

        rooms.insert(parts[1].to_string(), room);
        room_names.push(parts[1].to_string());

        // println!("{parts:?}");
    }

    println!("Calculating Paths");
    for i in 0..room_names.len() {
        if room_names[i] != "AA" && rooms.get(&room_names[i]).unwrap().flow_rate == 0 {
            continue;
        }

        for j in 0..room_names.len() {
            if i == j {
                continue;
            }

            if rooms.get(&room_names[j]).unwrap().flow_rate == 0 {
                continue;
            }

            let path = get_path(&rooms, &room_names[i], &room_names[j]);

            rooms
                .get_mut(&room_names[i])
                .unwrap()
                .paths
                .insert(room_names[j].clone(), path.len());
        }
    }

    println!("Finding Answer");
    let mut work = VecDeque::new();
    work.push_front((vec!["AA".to_string()], 26, 0));

    let mut solutions = Vec::new();

    while let Some(step) = work.pop_front() {
        let last = step.0[step.0.len() - 1].clone();

        let paths = rooms.get(&last).unwrap().paths.clone();
        for (to, len) in paths {
            if step.0.contains(&to) {
                continue;
            }

            let flow_rate = rooms.get(&to).unwrap().flow_rate;

            if step.1 > len {
                let mut new_step = step.clone();
                new_step.0.push(to.clone());
                new_step.1 -= len;
                new_step.2 += new_step.1 * flow_rate;

                solutions.push(new_step.clone());
                work.push_back(new_step);
            }
        }
    }

    solutions.sort_by_key(|s| -(s.2 as isize));

    let mut ans = 0;
    for i in 0..solutions.len() {
        'search: for j in i + 1..solutions.len() {
            if solutions[i].2 + solutions[j].2 < ans {
                continue;
            }
            for k in 1..solutions[i].0.len() {
                if solutions[j].0.contains(&solutions[i].0[k]) {
                    continue 'search;
                }
            }

            let cur = solutions[i].2 + solutions[j].2;
            if cur > ans {
                ans = cur;
            }
        }
    }
    println!("{}", solutions.len());
    println!("ans: {ans}");
}
