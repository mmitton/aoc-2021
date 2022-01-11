#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
}

fn load_input(filename: &str) -> Result<(Vec<char>, BTreeMap<char, Vec<char>>), Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();

    let mut steps = Vec::new();
    let mut requirements = BTreeMap::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim().to_string();
        if line == "" || line.starts_with("#") {
            continue;
        }

        let chars: Vec<char> = line.chars().collect();

        let to = chars["Step T must be finished before step X".len() - 1];
        let from = chars["Step T".len() - 1];

        if !steps.contains(&from) {
            steps.push(from);
        }
        if !steps.contains(&to) {
            steps.push(to);
        }

        if !requirements.contains_key(&to) {
            requirements.insert(to, Vec::new());
        }
        requirements.get_mut(&to).unwrap().push(from);
    }

    for step in &steps {
        if !requirements.contains_key(step) {
            requirements.insert(*step, Vec::new());
        }
    }

    Ok((steps, requirements))
}

fn main() -> Result<(), Error> {
    let (mut steps, requirements) = load_input(INPUT_FILE)?;

    let (num_workers, time_delta) = if cfg!(debug_assertions) {
        (2, 0)
    } else {
        (5, 60)
    };

    let mut order = Vec::new();
    let mut in_progress: BTreeSet<char> = BTreeSet::new();

    let mut workers: Vec<Option<(usize, char)>> = vec![None; num_workers];
    let mut t = 0;

    while steps.len() > 0 || in_progress.len() > 0 {
        let mut ready = Vec::new();

        // Find workers who complete first
        let mut min_t = usize::MAX;
        for i in 0..workers.len() {
            if let Some((t, _)) = workers[i] {
                if t < min_t {
                    min_t = t;
                }
            }
        }
        if min_t != usize::MAX {
            let mut completed = Vec::new();
            for i in 0..workers.len() {
                let mut clear = false;
                if let Some((t, c)) = workers[i] {
                    if t == min_t {
                        clear = true;
                        in_progress.remove(&c);
                        completed.push(c);
                    }
                }
                if clear {
                    workers[i] = None;
                }
            }
            completed.sort();
            order.extend_from_slice(&completed);
            t = min_t + 1;
        }
        println!("t={}", t);

        for i in 0..steps.len() {
            let requirements = requirements.get(&steps[i]).unwrap();
            let mut ok = true;
            for c in requirements {
                if !order.contains(c) {
                    ok = false;
                    break;
                }
            }

            if ok {
                ready.push(steps[i]);
            }
        }

        if ready.len() > 0 {
            ready.sort();
            println!("ready: {:?}  workers:{:?}", ready, workers);

            let mut ready_idx = 0;
            for i in 0..workers.len() {
                if workers[i].is_none() {
                    let c = ready[ready_idx];
                    workers[i] = Some((t + (c as u8 - 'A' as u8) as usize + time_delta, c));
                    println!("workers[{}]: {:?}", i, workers[i]);
                    in_progress.insert(c);
                    steps.retain(|sc| *sc != c);
                    ready_idx += 1;
                    if ready_idx == ready.len() {
                        break;
                    }
                }
            }
        }
    }

    println!("Order: {}", order.iter().collect::<String>());
    Ok(())
}
