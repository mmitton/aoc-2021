use std::collections::BTreeMap;

#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
}

struct Directions(Vec<char>);

impl Directions {
    fn delivered(&self) -> BTreeMap<(isize, isize), usize> {
        let mut delivered = BTreeMap::new();

        let mut x = [0isize; 2];
        let mut y = [0isize; 2];
        delivered.insert((0, 0), 2);

        for (i, c) in self.0.iter().enumerate() {
            match c {
                '^' => y[i % 2] -= 1,
                'v' => y[i % 2] += 1,
                '<' => x[i % 2] -= 1,
                '>' => x[i % 2] += 1,
                _ => panic!("Unknown direction: {}", c),
            }

            let cnt = *delivered.get(&(x[i % 2], y[i % 2])).unwrap_or(&0) + 1;
            delivered.insert((x[i % 2], y[i % 2]), cnt);
        }

        delivered
    }
}

fn load_input(filename: &str) -> Result<Vec<Directions>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();

    let mut inputs = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }

        inputs.push(Directions(line.chars().collect()));
    }

    Ok(inputs)
}

fn main() -> Result<(), Error> {
    let inputs = load_input(INPUT_FILE)?;

    for input in &inputs {
        let delivered = input.delivered();

        println!("Delivered: {}", delivered.len())
    }

    Ok(())
}
