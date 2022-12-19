#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

use std::collections::{BTreeMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Map {
    start: (usize, usize),
    end: (usize, usize),
    rows: Vec<Vec<char>>,
}

impl Map {
    fn new() -> Self {
        Self {
            start: (0, 0),
            end: (0, 0),
            rows: Vec::new(),
        }
    }

    fn add_row(&mut self, row: Vec<char>) {
        for (idx, c) in row.iter().enumerate() {
            if *c == 'S' {
                self.start = (idx, self.rows.len());
            }
            if *c == 'E' {
                self.end = (idx, self.rows.len());
            }
        }

        self.rows.push(row);
    }

    fn get_height(&self, x: usize, y: usize) -> u32 {
        let c = self.rows[y][x];
        // println!("{x},{y}  {c}");
        match c {
            'S' => 0,
            'E' => 27,
            _ => c as u32 - 'a' as u32 + 1,
        }
    }
}

fn main() {
    let lines: Vec<String> = {
        let file = File::open(INPUT_FILE).expect("Cannot open input file");
        BufReader::new(file).lines().flatten().collect()
    };

    let mut map = Map::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }

        map.add_row(line.chars().collect());
    }

    let mut ans: usize = !0;

    for x in 0..map.rows[0].len() {
        for y in 0..map.rows.len() {
            if map.get_height(x, y) == 1 {
                let mut work = VecDeque::new();
                let mut seen = BTreeMap::new();
                work.push_front((x, y));
                seen.insert((x, y), 0);
                macro_rules! add {
                    ($old_pos:expr, $pos:expr) => {
                        let steps = *seen.get(&$old_pos).unwrap();
                        let old_height = map.get_height($old_pos.0, $old_pos.1);
                        let new_height = map.get_height($pos.0, $pos.1);
                        #[allow(clippy::map_entry)]
                        if !seen.contains_key(&$pos) {
                            if new_height <= old_height + 1 {
                                if $pos == map.end {
                                    if steps + 1 < ans {
                                        ans = steps + 1;
                                    }
                                    break;
                                }
                                seen.insert($pos, steps + 1);
                                work.push_back($pos);
                            }
                        }
                    };
                }

                while let Some(step) = work.pop_front() {
                    if step.0 > 0 {
                        let new_step = (step.0 - 1, step.1);
                        add!(step, new_step);
                    }
                    if step.0 < map.rows[0].len() - 1 {
                        let new_step = (step.0 + 1, step.1);
                        add!(step, new_step);
                    }
                    if step.1 > 0 {
                        let new_step = (step.0, step.1 - 1);
                        add!(step, new_step);
                    }
                    if step.1 < map.rows.len() - 1 {
                        let new_step = (step.0, step.1 + 1);
                        add!(step, new_step);
                    }
                }
            }
        }
    }

    println!("ans: {ans}");
}
