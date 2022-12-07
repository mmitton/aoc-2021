#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::fmt;

struct Dir {
    name: String,
    files: BTreeMap<String, usize>,
    dirs: BTreeMap<String, Dir>,
}

impl fmt::Debug for Dir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Dir")
            .field("size", &self.size())
            .field("files", &self.files)
            .field("dirs", &self.dirs)
            .finish()
    }
}

impl Dir {
    fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            files: BTreeMap::new(),
            dirs: BTreeMap::new(),
        }
    }

    fn size(&self) -> usize {
        let mut size = 0;
        for dir in self.dirs.values() {
            size += dir.size();
        }
        for file in self.files.values() {
            size += file;
        }
        size
    }

    fn part1(&self) -> usize {
        let mut ans = 0;

        let my_size = self.size();
        if my_size < 100_000 {
            ans += my_size;
        }

        for dir in self.dirs.values() {
            ans += dir.part1();
        }

        ans
    }

    fn find_dir(&mut self, name: &str) -> &mut Dir {
        if !self.dirs.contains_key(name) {
            let child_dir = Dir::new(name);
            self.dirs.insert(name.into(), child_dir);
        }
        self.dirs.get_mut(name).unwrap()
    }

    fn parse(&mut self, lines: &Vec<String>, start: usize) -> usize {
        let mut pos = start;
        while pos < lines.len() {
            let line = &lines[pos];

            if line.is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.split(' ').collect();
            match parts[0] {
                "$" => match parts[1] {
                    "ls" => {
                        pos += 1;
                    }
                    "cd" => match parts[2] {
                        ".." => return pos + 1,
                        _ => {
                            let child_dir = self.find_dir(parts[2]);
                            pos = child_dir.parse(lines, pos + 1);
                        }
                    },
                    _ => unreachable!(),
                },
                "dir" => {
                    // Dir
                    let _child_dir = self.find_dir(parts[1]);
                    pos += 1;
                }
                _ => {
                    // File
                    self.files
                        .insert(parts[1].into(), parts[0].parse().unwrap());
                    pos += 1;
                }
            }
        }

        lines.len()
    }
}

fn main() {
    let lines = {
        let file = File::open(INPUT_FILE).expect("Cannot open input file");
        BufReader::new(file).lines()
    };

    let lines: Vec<String> = lines.flatten().collect();
    let mut dir = Dir::new("/");
    dir.parse(&lines, 0);

    println!("{:#?}", dir);

    println!("{}", dir.part1());
}
