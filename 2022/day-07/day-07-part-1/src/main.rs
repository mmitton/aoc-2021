#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

use std::collections::BTreeMap;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Files {
    entries: Vec<(String, bool, usize)>,
}

impl Files {
    fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    fn add(&mut self, name: &str, size: usize) {
        let parts: Vec<&str> = name.split('/').collect();

        for i in 0..parts.len() {
            let name = format!("/{}", parts[1..i + 1].join("/"));

            let mut found = false;
            for entry in &mut self.entries {
                if entry.0 == name {
                    entry.2 += size;
                    found = true;
                    break;
                }
            }

            if !found {
                let file = i == parts.len() - 1;
                self.entries.push((name, file, size));
            }
        }
    }
}

fn build_filename(parent: &str, child: &str) -> String {
    if parent == "/" {
        format!("/{child}")
    } else {
        format!("{parent}/{child}")
    }
}

fn main() {
    let lines = {
        let file = File::open(INPUT_FILE).expect("Cannot open input file");
        BufReader::new(file).lines()
    };

    let lines: Vec<String> = lines.flatten().collect();

    let mut files = Files::new();
    let mut cur_dir = "".to_string();
    let mut parents = Vec::new();
    for line in &lines {
        if line == "$ cd /" {
            cur_dir = "/".to_string();
            parents.clear();
        } else if line == "$ cd .." {
            cur_dir = parents.pop().unwrap();
        } else if let Some(line) = line.strip_prefix("$ cd ") {
            parents.push(cur_dir.clone());
            cur_dir = build_filename(&cur_dir, line);
        } else if !line.starts_with("dir ") && line != "$ ls" {
            let (size, name) = line.split_once(' ').unwrap();
            let size: usize = size.parse().unwrap();

            files.add(&build_filename(&cur_dir, name), size);
        }
    }

    files.entries.sort();
    let mut ans = 0;
    for entry in &files.entries {
        println!("{entry:?}");
        if !entry.1 && entry.2 <= 100000 {
            ans += entry.2;
        }
    }

    println!("ans: {ans}");
}
