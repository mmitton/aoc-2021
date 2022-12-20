const INPUT_FILE: &str = if cfg!(debug_assertions) {
    "../input-sample.txt"
} else {
    "../input.txt"
};

use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn run_blueprint(
    blueprint: &[[u16; 4]; 4],
    robots: [u16; 4],
    materials: [u16; 4],
    minutes: u16,
) -> [u16; 4] {
    let mut best: [u16; 4] = materials;

    let mut work = BTreeSet::new();
    let job = (1, materials, robots);
    work.insert(job);

    let mut best_geode = vec![0u16; minutes as usize + 1];

    let mut need_to_build = Vec::new();
    'build_loop: while let Some((minute, materials, robots)) = work.pop_first() {
        // println!("minute:{minute}  materials:{materials:?}  robots:{robots:?}");
        let mut new_materials = materials;
        let mut built = [false; 4];
        for minute in minute..=minutes {
            if built == [true; 4] {
                break;
            }
            let materials = new_materials;
            for (new_material, robot) in new_materials.iter_mut().zip(robots.iter()) {
                *new_material += robot;
            }
            // println!("minute:{minute}  materials:{materials:?}  robots:{robots:?}");
            if new_materials[0] + 2 < best_geode[minute as usize] {
                continue;
            }

            if minute == minutes {
                if new_materials[0] > best[0] {
                    println!("new best: {new_materials:?} {robots:?}");
                    best = new_materials;
                }
                continue 'build_loop;
            }

            need_to_build.clear();
            need_to_build.push(0);

            for build in 0..4 {
                if built[build] {
                    continue;
                }
                let mut can_build = true;
                for (material, needed) in blueprint[build].iter().enumerate().rev() {
                    if *needed > materials[material] {
                        can_build = false;
                        if *needed > materials[material] + robots[material]
                            && !need_to_build.contains(&material)
                        {
                            // println!("need a {material} to build a {build}");
                            need_to_build.push(material);
                        }
                    }
                }

                if can_build {
                    // Build a `build` this minute
                    let mut new_materials = new_materials;
                    for (material, needed) in new_materials.iter_mut().zip(blueprint[build].iter())
                    {
                        *material -= needed;
                    }

                    let mut robots = robots;
                    robots[build] += 1;

                    if best_geode[minute as usize] < materials[0] {
                        println!("new best @ {minute} {materials:?}");
                        best_geode[minute as usize] = materials[0];
                        work.retain(|(_minute, _materials, _robots)| {
                            _materials[0] + 2 >= best_geode[*_minute as usize]
                        });
                    }

                    let new_job = (minute + 1, new_materials, robots);
                    if !work.contains(&new_job) {
                        built[build] = true;
                        work.insert(new_job);
                        continue;
                    }
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

            let mut blueprint: [[u16; 4]; 4] = [[0; 4]; 4];

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
