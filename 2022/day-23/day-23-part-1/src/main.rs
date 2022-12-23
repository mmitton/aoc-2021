const INPUT_FILE: &str = if cfg!(debug_assertions) {
    "../input-sample.txt"
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

    println!();
    println!("Initial");
    dump_elves(&cur_locations);

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
    for round in 0..10 {
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

                    println!(
                        "Can elf {} move {}: {}",
                        elf_idx + 1,
                        rule.name,
                        if nope { "nope" } else { "yup!" }
                    );
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
            } else {
                println!("Elf {} chooses not to move", elf_idx + 1);
            }
        }

        // Move any elves who do not conflict with another elf
        cur_locations.clear();
        for (elf_idx, elf) in elves.iter_mut().enumerate() {
            if let Some(target) = elf.target.take() {
                if !matches!(duplicates.get(&target), Some(true)) {
                    elf.pos = target;
                    println!("Moving elf {} to {:?}", elf_idx + 1, target);
                } else {
                    println!("NOT Moving elf {} to {:?}", elf_idx + 1, target);
                }
            }
            cur_locations.insert(elf.pos);
        }

        let rule = rules.remove(0);
        rules.push(rule);
        println!();
        println!("After round {}", round + 1);
        dump_elves(&cur_locations);
    }

    println!();
    println!("Final");
    let ans = dump_elves(&cur_locations);

    println!("ans = {ans}");
}
