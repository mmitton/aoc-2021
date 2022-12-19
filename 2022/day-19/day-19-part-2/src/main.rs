const INPUT_FILE: &str = if cfg!(debug_assertions) {
    "../input-sample.txt"
} else {
    "../input.txt"
};

use std::collections::{BTreeSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Material {
    Geode,
    Obsidian,
    Clay,
    Ore,
}

impl From<Material> for usize {
    fn from(m: Material) -> Self {
        match m {
            Material::Geode => 0,
            Material::Obsidian => 1,
            Material::Clay => 2,
            Material::Ore => 3,
        }
    }
}

fn run_blueprint(
    blueprint: &[[usize; 4]; 4],
    robots: [usize; 4],
    materials: [usize; 4],
    minutes: usize,
) -> [usize; 4] {
    let mut best: [usize; 4] = materials;
    let mut best_geode = 0;

    let mut work = VecDeque::new();
    let mut seen = BTreeSet::new();
    let job = (materials, 1, robots, 24);
    work.push_front(job);
    seen.insert(job);

    while let Some((materials, minute, robots, _best_case)) = work.pop_front() {
        // println!("next work: minute:{minute} materials:{materials:?} robots:{robots:?}");
        'robot_builder: for robot in 0..4 {
            let mut materials = materials;
            for minute in minute..=minutes {
                // println!("checking: robot:{robot} minute:{minute} materials:{materials:?}");
                if minute == minutes {
                    for (material, robot) in materials.iter_mut().zip(robots.iter()) {
                        *material += robot;
                    }
                    if materials[0] > best_geode {
                        best_geode = materials[0];
                        best = materials;
                        print!(
                            "new_best: {best_geode} {materials:?} {robots:?}  {} -> ",
                            work.len()
                        );
                        work.retain(|(_materials, _minute, _robots, best_case)| {
                            *best_case > best_geode
                        });
                        println!("{}", work.len());
                    }
                    break;
                }

                // Can we build a robot now?
                let mut can_build = true;
                for (material, needed) in materials.iter().zip(blueprint[robot].iter()) {
                    if material < needed {
                        can_build = false;
                        break;
                    }
                }

                for (material, robot) in materials.iter_mut().zip(robots.iter()) {
                    *material += robot;
                }

                if can_build {
                    // println!("building robot {robot} in minute {minute}");
                    for (material, needed) in materials.iter_mut().zip(blueprint[robot].iter()) {
                        *material -= *needed;
                    }
                    let mut robots = robots;
                    robots[robot] += 1;

                    let best_case =
                        materials[0] + (minutes - minute) * (robots[0] + (minutes - minute));
                    if best_case > best_geode {
                        let new_job = (materials, minute + 1, robots, best_case);
                        if !seen.contains(&new_job) {
                            work.push_back(new_job);
                            seen.insert(new_job);
                        }
                    }
                    continue 'robot_builder;
                }
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

            let mut blueprint: [[usize; 4]; 4] = [[0; 4]; 4];

            for group in &groups[1..] {
                let group = group.replace(" and ", " ");
                let group: Vec<&str> = group.split(' ').collect();

                let robot = match group[1] {
                    "ore" => Material::Ore,
                    "clay" => Material::Clay,
                    "obsidian" => Material::Obsidian,
                    "geode" => Material::Geode,
                    _ => unreachable!(),
                } as usize;

                for req in group[4..].chunks(2) {
                    let material = match req[1] {
                        "ore" => Material::Ore,
                        "clay" => Material::Clay,
                        "obsidian" => Material::Obsidian,
                        "geode" => Material::Geode,
                        _ => unreachable!(),
                    } as usize;
                    blueprint[robot][material] = req[0].parse().unwrap();
                }
            }

            let best = run_blueprint(&blueprint, [0, 0, 0, 1], [0, 0, 0, 0], 32);
            println!("{idx} {best:?}");
            best[Material::Geode as usize]
        })
        .collect();

    let mut ans = 1;
    for best in &best {
        ans *= *best;
    }
    println!("{best:?}");
    println!("ans: {ans}");
}
