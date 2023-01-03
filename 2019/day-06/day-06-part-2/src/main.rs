const INPUT_FILE: &str = if cfg!(debug_assertions) {
    "../input-sample-2.txt"
} else {
    "../input.txt"
};

use std::collections::{BTreeMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Object {
    parents: Vec<String>,
    children: Vec<String>,
}

fn main() {
    let lines: Vec<String> = {
        let file = File::open(INPUT_FILE).expect("Cannot open input file");
        BufReader::new(file).lines().flatten().collect()
    };

    let mut objects = BTreeMap::new();
    for line in lines.iter() {
        if line.is_empty() {
            continue;
        }

        let (obj1, obj2) = line.split_once(')').unwrap();
        let obj1 = obj1.to_string();
        let obj2 = obj2.to_string();
        let object1 = objects.entry(obj1.clone()).or_insert(Object {
            parents: Vec::new(),
            children: Vec::new(),
        });
        object1.children.push(obj2.clone());

        let object2 = objects.entry(obj2.clone()).or_insert(Object {
            parents: Vec::new(),
            children: Vec::new(),
        });
        object2.parents.push(obj1.clone());
    }

    let mut work = VecDeque::new();
    for (k, v) in objects.iter() {
        if v.parents.is_empty() {
            work.push_back((k.to_owned(), Vec::new()));
        }
    }
    while let Some((obj, parents)) = work.pop_front() {
        for (k, v) in objects.iter_mut() {
            if v.parents.contains(&obj) {
                v.parents.extend_from_slice(parents.as_slice());
                work.push_back((k.to_owned(), v.parents.clone()));
            }
        }
    }

    let you = objects.get("YOU").unwrap();
    let san = objects.get("SAN").unwrap();
    for (up, parent) in you.parents.iter().enumerate() {
        if let Some(down) = san
            .parents
            .iter()
            .position(|san_parent| san_parent == parent)
        {
            let ans = up + down;
            println!("ans: {ans}");
            break;
        }
    }
}
