const INPUT_FILE: &str = if cfg!(debug_assertions) {
    "../input-sample.txt"
} else {
    "../input.txt"
};

use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Entity {
    pos: (isize, isize),
    delta: (isize, isize),
    c: char,
}

fn print(minute: usize, max_x: usize, max_y: usize, entities: &[Entity]) {
    println!("\nAfter Minute {minute}");
    for y in 0..max_y as isize {
        for x in 0..max_x as isize {
            let mut c = '.';
            let mut cnt = 0;
            for entity in entities {
                if entity.pos == (x, y) {
                    if entity.delta == (0, 0) {
                        c = '#';
                    } else if c != '#' {
                        cnt += 1;
                        c = entity.c;
                    }
                }
            }

            if cnt > 1 {
                c = (cnt as u8 + '0' as u8) as char;
            }

            print!("{c}");
        }
        println!();
    }
}

fn move_entities(max_x: usize, max_y: usize, entities: &mut [Entity]) -> BTreeSet<(isize, isize)> {
    let max_x = max_x as isize;
    let max_y = max_y as isize;

    let mut entities_at = BTreeSet::new();
    for entity in entities {
        if entity.delta != (0, 0) {
            entity.pos.0 += entity.delta.0;
            entity.pos.1 += entity.delta.1;

            if entity.pos.0 == 0 {
                entity.pos.0 = max_x - 2;
            }
            if entity.pos.0 == max_x - 1 {
                entity.pos.0 = 1;
            }
            if entity.pos.1 == 0 {
                entity.pos.1 = max_y - 2;
            }
            if entity.pos.1 == max_y - 1 {
                entity.pos.1 = 1;
            }
        }
        entities_at.insert(entity.pos);
    }

    entities_at
}

fn main() {
    let lines: Vec<String> = {
        let file = File::open(INPUT_FILE).expect("Cannot open input file");
        BufReader::new(file).lines().flatten().collect()
    };

    let mut entities: Vec<Entity> = Vec::new();

    let max_y = lines.len();
    let max_x = lines[0].len();

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = (x as isize, y as isize);
            let delta = match c {
                '.' => None,
                '#' => Some((0, 0)),
                '<' => Some((-1, 0)),
                '>' => Some((1, 0)),
                '^' => Some((0, -1)),
                'v' => Some((0, 1)),
                _ => unreachable!(),
            };

            if let Some(delta) = delta {
                entities.push(Entity { pos, delta, c });
            }
        }
    }

    let mut positions: Vec<(isize, isize)> = vec![(1, 0)];

    print(0, max_x, max_y, &entities);
    let mut minute = 0;
    loop {
        minute += 1;

        println!("{} positions at minute {minute}", positions.len());

        let entities_at = move_entities(max_x, max_y, &mut entities);
        // print(minute, max_x, max_y, &entities);

        let mut new_positions = Vec::new();
        for position in positions {
            macro_rules! new_pos {
                ($dx:expr, $dy:expr) => {{
                    if position.1 + $dy >= 0 {
                        let new_pos = (position.0 + $dx, position.1 + $dy);
                        if new_pos == (max_x as isize - 2, max_y as isize - 1) {
                            println!("Ans: {minute}");
                            return;
                        }
                        if !entities_at.contains(&new_pos) && !new_positions.contains(&new_pos) {
                            new_positions.push(new_pos);
                        }
                    }
                }};
            }

            new_pos!(0, 0);
            new_pos!(-1, 0);
            new_pos!(1, 0);
            new_pos!(0, -1);
            new_pos!(0, 1);
        }

        positions = new_positions;
    }
}
