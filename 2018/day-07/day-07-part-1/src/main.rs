#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

use std::collections::BTreeMap;

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

    let mut order = Vec::new();

    while steps.len() > 0 {
        let mut ready = Vec::new();

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

        assert!(ready.len() > 0);
        ready.sort();
        order.push(ready[0]);
        steps.retain(|c| *c != ready[0]);
    }

    println!("Order: {}", order.iter().collect::<String>());
    Ok(())
}
