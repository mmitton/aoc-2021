const INPUT_FILE: &str = if cfg!(debug_assertions) {
    "../input-sample.txt"
} else {
    "../input.txt"
};

use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn run_blueprint(
    blueprint: &[[u8; 4]; 4],
    robots: [u8; 4],
    materials: [u8; 4],
    minutes: u8,
) -> [u8; 4] {
    let mut best: [u8; 4] = materials;
    let mut best_geode = 0;

    let mut work = BTreeSet::new();
    let job = (1, materials, robots);
    work.insert(job);

    while let Some((minute, materials, robots)) = work.pop_first() {
        let mut new_materials = materials;
        for (new_material, robot) in new_materials.iter_mut().zip(robots.iter()) {
            *new_material += robot;
        }
        if new_materials[0] > best_geode {
            best_geode = new_materials[0];
            println!("new best_geode: {best_geode}");
            work.retain(|(_minute, materials, robots)| materials[0] + robots[0] + 1 >= best_geode);
        } else if new_materials[0] + robots[0] + 1 < best_geode {
            continue;
        }
        if minute == minutes {
            if new_materials[0] > best[0] {
                println!("new best: {new_materials:?} {robots:?}");
                best = new_materials;
            }
            continue;
        }

        // println!("next work: minute:{minute} materials:{materials:?} robots:{robots:?}");
        for robot in 0..4 {
            // Can we build a robot now?
            let mut can_build = true;
            for (material, needed) in materials.iter().zip(blueprint[robot].iter()) {
                if material < needed {
                    can_build = false;
                    break;
                }
            }

            if can_build {
                let mut new_materials = new_materials;
                for (material, needed) in new_materials.iter_mut().zip(blueprint[robot].iter()) {
                    *material -= needed;
                }

                let mut robots = robots;
                robots[robot] += 1;

                // println!("building robot {robot} in minute {minute}  {materials:?} {robots:?}");
                if new_materials[0] + 2 >= best_geode {
                    let new_job = (minute + 1, new_materials, robots);
                    if !work.contains(&new_job) {
                        work.insert(new_job);
                    }
                }
            }
        }

        // println!("building robot {robot} in minute {minute}  {materials:?} {robots:?}");
        if new_materials[0] + 2 >= best_geode {
            let new_job = (minute + 1, new_materials, robots);
            if !work.contains(&new_job) {
                work.insert(new_job);
            }
        }
    }

    best
}

fn main() {
    let lines: Vec<String> = {
        let file = File::open(INPUT_FILE).expect("Cannot open input file");
        BufReader::new(file).lines().flatten().collect()
    };

    let lines = if lines.len() > 3 {
        &lines[0..3]
    } else {
        &lines
    };

    let best: Vec<usize> = lines
        .iter()
        .enumerate()
        .map(|(idx, line)| {
            let line = line.strip_suffix('.').unwrap();
            let line = line.replace(':', ".");
            let groups: Vec<&str> = line.split(". ").collect();

            let mut blueprint: [[u8; 4]; 4] = [[0; 4]; 4];

            for group in &groups[1..] {
                let group = group.replace(" and ", " ");
                let group: Vec<&str> = group.split(' ').collect();

                let robot = match group[1] {
                    "ore" => 3,
                    "clay" => 2,
                    "obsidian" => 1,
                    "geode" => 0,
                    _ => unreachable!(),
                };

                for req in group[4..].chunks(2) {
                    let material = match req[1] {
                        "ore" => 3,
                        "clay" => 2,
                        "obsidian" => 1,
                        "geode" => 0,
                        _ => unreachable!(),
                    };
                    blueprint[robot][material] = req[0].parse().unwrap();
                }
            }

            let best = run_blueprint(&blueprint, [0, 0, 0, 1], [0, 0, 0, 0], 32);
            println!("{idx} {best:?}");
            best[0] as usize
        })
        .collect();

    let mut ans = 1;
    for best in &best {
        ans *= *best;
    }
    println!("{best:?}");
    println!("ans: {ans}");
}
