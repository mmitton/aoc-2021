#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NAN(std::num::ParseIntError),
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Self::NAN(e)
    }
}

fn load_input(filename: &str) -> Result<(usize, Vec<usize>), Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;
    let lines = BufReader::new(f).lines();

    let mut target: usize = 0;
    let mut jugs: Vec<usize> = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" || line.starts_with("#") {
            continue;
        }

        if target == 0 {
            target = line.parse()?;
        } else {
            jugs.push(line.parse()?);
        }
    }

    Ok((target, jugs))
}

fn main() -> Result<(), Error> {
    let (target, jugs) = load_input(INPUT_FILE)?;

    let mut visited: Vec<Vec<usize>> = Vec::new();
    let mut good: Vec<Vec<usize>> = Vec::new();
    let mut explore: Vec<(usize, Vec<usize>)> = Vec::new();

    for i in 0..jugs.len() {
        if jugs[i] <= target {
            let new_jugs = vec![i];
            visited.push(new_jugs.clone());
            explore.push((jugs[i], new_jugs));
        }
    }

    println!("Target: {}", target);
    let mut idx = 0;
    while idx < explore.len() {
        if cfg!(debug_assertions) {
            println!("{}: {:?}", explore[idx].0, explore[idx].1);
        }

        if explore[idx].0 == target {
            if good.len() == 0 || good[0].len() == explore[idx].1.len() {
                good.push(explore[idx].1.clone());
            } else if good[0].len() > explore[idx].1.len() {
                good.clear();
                good.push(explore[idx].1.clone());
            }
        } else {
            if good.len() == 0 || explore[idx].1.len() < good[0].len() {
                for i in 0..jugs.len() {
                    if explore[idx].1.contains(&i) {
                        continue;
                    }

                    if explore[idx].0 + jugs[i] > target {
                        continue;
                    }

                    let mut new_jugs = explore[idx].1.clone();
                    new_jugs.push(i);
                    new_jugs.sort();
                    if visited.contains(&new_jugs) {
                        continue;
                    }

                    visited.push(new_jugs.clone());
                    explore.push((explore[idx].0 + jugs[i], new_jugs));
                }
            }
        }

        idx += 1;
    }

    if cfg!(debug_assertions) {
        println!("Good: {:?}", good);
    }
    println!("Number of good: {}", good.len());

    Ok(())
}
