const INPUT_FILE: &str = if cfg!(debug_assertions) {
    "../input-sample.txt"
} else {
    "../input.txt"
};

use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Coord {
    x: isize,
    y: isize,
    z: isize,
}

type Bugs = BTreeSet<Coord>;

fn print_bugs(iter: usize, bugs: &Bugs) {
    println!("After {iter} minute(s)");
    let mut min_z = isize::MAX;
    let mut max_z = isize::MIN;
    for c in bugs.iter() {
        if c.z < min_z {
            min_z = c.z;
        };
        if c.z > max_z {
            max_z = c.z;
        }
    }
    for z in min_z..=max_z {
        println!("Depth {z}");
        for y in 0..5 {
            for x in 0..5 {
                if x == 2 && y == 2 {
                    print!("?");
                } else {
                    print!(
                        "{}",
                        if bugs.contains(&Coord { x, y, z }) {
                            '#'
                        } else {
                            '.'
                        }
                    );
                }
            }
            println!();
        }
    }
    println!();
}

fn process(bugs: &Bugs) {
    if bugs.is_empty() {
        return;
    }

    fn get_neighbors(bugs: &Bugs, c: &Coord) -> usize {
        if c.x == 2 && c.y == 2 {
            return 0;
        }
        let mut num_neighbors = 0;

        macro_rules! check {
            ($coord:expr) => {
                if bugs.contains(&$coord) {
                    num_neighbors += 1;
                }
            };
        }

        // Scan in y+1 direction
        if c.x == 2 && c.y == 1 {
            // Look in
            for x in 0..5 {
                check!(Coord {
                    x,
                    y: 0,
                    z: c.z + 1,
                });
            }
        } else if c.y == 4 {
            // Look up
            check!(Coord {
                x: 2,
                y: 3,
                z: c.z - 1,
            });
        } else {
            check!(Coord {
                x: c.x,
                y: c.y + 1,
                z: c.z,
            });
        }

        // Scan in y-1 direction
        if c.x == 2 && c.y == 3 {
            // Look in
            for x in 0..5 {
                check!(Coord {
                    x,
                    y: 4,
                    z: c.z + 1,
                });
            }
        } else if c.y == 0 {
            // Look up
            check!(Coord {
                x: 2,
                y: 1,
                z: c.z - 1,
            });
        } else {
            check!(Coord {
                x: c.x,
                y: c.y - 1,
                z: c.z,
            });
        }

        // Scan in x+1 direction
        if c.x == 1 && c.y == 2 {
            // Look in
            for y in 0..5 {
                check!(Coord {
                    x: 0,
                    y,
                    z: c.z + 1,
                });
            }
        } else if c.x == 4 {
            // Look up
            check!(Coord {
                x: 3,
                y: 2,
                z: c.z - 1,
            });
        } else {
            check!(Coord {
                x: c.x + 1,
                y: c.y,
                z: c.z,
            });
        }

        // Scan in x-1 direction
        if c.x == 3 && c.y == 2 {
            // Look in
            for y in 0..5 {
                check!(Coord {
                    x: 4,
                    y,
                    z: c.z + 1,
                });
            }
        } else if c.x == 0 {
            // Look up
            check!(Coord {
                x: 1,
                y: 2,
                z: c.z - 1,
            });
        } else {
            check!(Coord {
                x: c.x - 1,
                y: c.y,
                z: c.z,
            });
        }

        num_neighbors
    }

    let mut min_z = isize::MAX;
    let mut max_z = isize::MIN;
    for c in bugs.iter() {
        if c.z < min_z {
            min_z = c.z;
        };
        if c.z > max_z {
            max_z = c.z;
        }
    }

    let mut bugs = bugs.clone();
    print_bugs(0, &bugs);
    let max_iters = if cfg!(debug_assertions) { 10 } else { 200 };
    for iter in 1..=max_iters {
        let mut next_bugs = BTreeSet::new();
        let mut next_min_z = isize::MAX;
        let mut next_max_z = isize::MIN;

        for z in min_z - 1..=max_z + 1 {
            for y in 0..5 {
                for x in 0..5 {
                    let pos = Coord { x, y, z };
                    let num_neighbors = get_neighbors(&bugs, &pos);
                    let is_alive = bugs.contains(&pos);
                    if num_neighbors == 1 || (!is_alive && num_neighbors == 2) {
                        next_bugs.insert(pos);
                        if pos.z < next_min_z {
                            next_min_z = pos.z;
                        }
                        if pos.z > next_max_z {
                            next_max_z = pos.z;
                        }
                    }
                }
            }
        }
        bugs = next_bugs;
        min_z = next_min_z;
        max_z = next_max_z;
        print_bugs(iter, &bugs);
    }

    println!("ans: {}", bugs.len());
}

fn main() {
    let lines: Vec<String> = {
        let file = File::open(INPUT_FILE).expect("Cannot open input file");
        BufReader::new(file).lines().flatten().collect()
    };

    let mut bugs: Bugs = Bugs::new();
    let mut y = 0;
    for line in lines.iter() {
        if line.is_empty() {
            process(&bugs);
            bugs.clear();
            y = 0;
            continue;
        }

        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                bugs.insert(Coord {
                    x: x as isize,
                    y,
                    z: 0,
                });
            }
        }
        y += 1;
    }
    process(&bugs);
}
