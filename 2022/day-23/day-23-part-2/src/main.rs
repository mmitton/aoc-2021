const INPUT_FILE: &str = if cfg!(debug_assertions) {
    "../input-sample2.txt"
} else {
    "../input.txt"
};

use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

type Pos = (isize, isize);
struct Elf {
    pos: Pos,
    target: Option<Pos>,
}

fn dump_elves(locations: &BTreeSet<Pos>) -> usize {
    let mut min = (isize::MAX, isize::MAX);
    let mut max = (isize::MIN, isize::MIN);

    for elf in locations.iter() {
        if elf.0 < min.0 {
            min.0 = elf.0;
        }
        if elf.0 > max.0 {
            max.0 = elf.0;
        }
        if elf.1 < min.1 {
            min.1 = elf.1;
        }
        if elf.1 > max.1 {
            max.1 = elf.1;
        }
    }

    let mut blank = 0;
    for y in min.1..=max.1 {
        for x in min.0..=max.0 {
            if locations.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
                blank += 1;
            }
        }
        println!();
    }

    blank
}

fn main() {
    let lines: Vec<String> = {
        let file = File::open(INPUT_FILE).expect("Cannot open input file");
        BufReader::new(file).lines().flatten().collect()
    };

    let mut cur_locations: BTreeSet<Pos> = BTreeSet::new();
    let mut elves: Vec<Elf> = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                let elf = Elf {
                    pos: (x as isize, y as isize),
                    target: None,
                };
                cur_locations.insert(elf.pos);
                elves.push(elf);
            }
        }
    }

    if cfg!(debug_assertions) {
        println!();
        println!("Initial");
        dump_elves(&cur_locations);
    }

    struct Rule {
        name: &'static str,
        target: Pos,
        check: [Pos; 3],
    }

    let mut rules: Vec<Rule> = vec![
        Rule {
            name: "North",
            target: (0, -1),
            check: [(-1, -1), (0, -1), (1, -1)],
        },
        Rule {
            name: "South",
            target: (0, 1),
            check: [(-1, 1), (0, 1), (1, 1)],
        },
        Rule {
            name: "West",
            target: (-1, 0),
            check: [(-1, -1), (-1, 0), (-1, 1)],
        },
        Rule {
            name: "East",
            target: (1, 0),
            check: [(1, -1), (1, 0), (1, 1)],
        },
    ];

    let mut duplicates: BTreeMap<Pos, bool> = BTreeMap::new();
    let mut round = 0;
    loop {
        round += 1;
        // Let elves decide where to move
        duplicates.clear();
        for (elf_idx, elf) in elves.iter_mut().enumerate() {
            // Check to see if there are *any* neighbors
            let has_neighbor = cur_locations.contains(&(elf.pos.0 - 1, elf.pos.1 - 1))
                | cur_locations.contains(&(elf.pos.0 - 1, elf.pos.1))
                | cur_locations.contains(&(elf.pos.0 - 1, elf.pos.1 + 1))
                | cur_locations.contains(&(elf.pos.0, elf.pos.1 + 1))
                | cur_locations.contains(&(elf.pos.0 + 1, elf.pos.1 + 1))
                | cur_locations.contains(&(elf.pos.0 + 1, elf.pos.1))
                | cur_locations.contains(&(elf.pos.0 + 1, elf.pos.1 - 1))
                | cur_locations.contains(&(elf.pos.0, elf.pos.1 - 1));
            if has_neighbor {
                for rule in &rules {
                    let mut nope = false;
                    for check in rule.check {
                        let neighbor = (elf.pos.0 + check.0, elf.pos.1 + check.1);
                        if cur_locations.contains(&neighbor) {
                            nope = true;
                            break;
                        }
                    }

                    if cfg!(debug_assertions) {
                        println!(
                            "Can elf {} move {}: {}",
                            elf_idx + 1,
                            rule.name,
                            if nope { "nope" } else { "yup!" }
                        );
                    }
                    if !nope {
                        let move_to = (elf.pos.0 + rule.target.0, elf.pos.1 + rule.target.1);
                        elf.target = Some(move_to);
                        duplicates
                            .entry(move_to)
                            .and_modify(|dup| *dup = true)
                            .or_insert(false);
                        break;
                    }
                }
            } else if cfg!(debug_assertions) {
                println!("Elf {} chooses not to move", elf_idx + 1);
            }
        }

        // Move any elves who do not conflict with another elf
        cur_locations.clear();
        let mut num_moved = 0;
        for (elf_idx, elf) in elves.iter_mut().enumerate() {
            if let Some(target) = elf.target.take() {
                if !matches!(duplicates.get(&target), Some(true)) {
                    elf.pos = target;
                    if cfg!(debug_assertions) {
                        println!("Moving elf {} to {:?}", elf_idx + 1, target);
                    }
                    num_moved += 1;
                } else if cfg!(debug_assertions) {
                    println!("NOT Moving elf {} to {:?}", elf_idx + 1, target);
                }
            }
            cur_locations.insert(elf.pos);
        }

        let rule = rules.remove(0);
        rules.push(rule);
        if cfg!(debug_assertions) {
            println!();
            println!("After round {round}");
            dump_elves(&cur_locations);
        }

        if num_moved == 0 {
            println!("No elves moved in round {round}");
            break;
        }
    }
}
