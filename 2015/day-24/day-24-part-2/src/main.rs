#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

use std::collections::VecDeque;

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NAN(std::num::ParseIntError),
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Error {
        Error::NAN(e)
    }
}

fn load_input(filename: &str) -> Result<Vec<usize>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();
    let mut packages = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }

        packages.push(line.parse()?);
    }

    Ok(packages)
}

fn find_solutions(packages: &Vec<usize>, target: usize) -> Vec<(usize, [Vec<usize>; 4])> {
    // Find all possible groups
    let mut groups = Vec::new();
    let mut stack = VecDeque::new();
    for i in 0..packages.len() {
        stack.push_back((packages[i], vec![i], vec![packages[i]], 1 << i, packages[i]));
    }

    while stack.len() > 0 {
        let group = stack.pop_front().unwrap();
        if group.0 == target {
            println!("{:?}", group.2);
            groups.push((group.2, group.3, group.4));
        } else {
            // Build out group
            let last = group.1[group.1.len() - 1];
            for i in last + 1..packages.len() {
                if group.0 + packages[i] <= target {
                    let mut new_group = group.clone();
                    new_group.0 += packages[i];
                    new_group.1.push(i);
                    new_group.2.push(packages[i]);
                    new_group.3 |= 1 << i;
                    new_group.4 *= packages[i];
                    stack.push_back(new_group);
                }
            }
        }
    }

    groups.sort_by(|a, b| {
        if a.0.len() != b.0.len() {
            return a.0.len().cmp(&b.0.len());
        }
        for i in 0..a.0.len() {
            if a.0[i] != b.0[i] {
                return b.0[i].cmp(&a.0[i]);
            }
        }
        std::cmp::Ordering::Less
    });

    println!("Found {} groups that add up to {}", groups.len(), target);

    let mut solutions = Vec::new();

    let mut best_len = None;
    let mut best_qe = usize::MAX;
    for i in 0..groups.len() {
        // Consider groups[i] for group 0, find a group for 1 and a group for 2
        if let Some(best_len) = best_len {
            if best_len < groups[i].0.len() {
                continue;
            }
        }
        if best_qe < groups[i].2 {
            continue;
        }

        println!("i:{}  best_len:{:?}  best_qe:{}", i, best_len, best_qe);
        for j in 0..groups.len() {
            if i == j || groups[i].1 & groups[j].1 != 0 {
                continue;
            }
            for k in j + 1..groups.len() {
                if i == k
                    || j == k
                    || groups[i].1 & groups[k].1 != 0
                    || groups[j].1 & groups[k].1 != 0
                {
                    continue;
                }
                for l in k + 1..groups.len() {
                    if i == l
                        || j == l
                        || k == l
                        || groups[i].1 & groups[l].1 != 0
                        || groups[j].1 & groups[l].1 != 0
                        || groups[k].1 & groups[l].1 != 0
                    {
                        continue;
                    }

                    solutions.push((
                        groups[i].2,
                        [
                            groups[i].0.clone(),
                            groups[j].0.clone(),
                            groups[k].0.clone(),
                            groups[l].0.clone(),
                        ],
                    ));
                    if best_len.is_none() {
                        best_len = Some(groups[i].0.len());
                    } else {
                        let old_best = best_len.unwrap();
                        best_len = Some(if old_best < groups[i].0.len() {
                            old_best
                        } else {
                            groups[i].0.len()
                        });
                    }
                    if groups[i].2 < best_qe {
                        best_qe = groups[i].2;
                    }
                }
            }
        }
    }

    solutions
}

fn main() -> Result<(), Error> {
    let mut packages = load_input(INPUT_FILE)?;
    packages.sort_by_key(|p| -(*p as isize));

    let mut sum = 0;
    for p in &packages {
        sum += *p;
    }
    let target = sum / 4;
    assert!(target * 4 == sum);
    println!("Packages: {:?}  target:{}", packages, target);

    let mut best = usize::MAX;
    let solutions = find_solutions(&packages, target);
    println!(
        "{:<30} {:<30} {:<30} {:<30}",
        "Group 1;", "Group 2;", "Group 3", "Group 4"
    );
    for solution in &solutions {
        if solution.0 < best {
            best = solution.0;
        }
        println!(
            "{:<30} {:<30} {:<30} {:<30}",
            format!("{:?}  QE={}", solution.1[0], solution.0),
            format!("{:?}", solution.1[1]),
            format!("{:?}", solution.1[2]),
            format!("{:?}", solution.1[3])
        );
    }

    println!("Answer: {}", best);

    Ok(())
}
