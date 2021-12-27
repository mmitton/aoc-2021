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
    fn from(e: std::num::ParseIntError) -> Error {
        Error::NAN(e)
    }
}

struct Component(usize, usize);

impl std::fmt::Debug for Component {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{}/{}", self.0, self.1)
    }
}

fn load_input(filename: &str) -> Result<Vec<Component>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;
    let lines = BufReader::new(f).lines();

    let mut components: Vec<Component> = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" || line.starts_with("#") {
            continue;
        }

        let parts: Vec<&str> = line.split("/").collect();
        let n1 = parts[0].parse()?;
        let n2 = parts[1].parse()?;
        if n1 < n2 {
            components.push(Component(n1, n2));
        } else {
            components.push(Component(n2, n1));
        }
    }

    Ok(components)
}

fn main() -> Result<(), Error> {
    let components = load_input(INPUT_FILE)?;

    let mut bridges: Vec<(Vec<usize>, usize)> = Vec::new();

    let mut best = 0;
    for i in 0..components.len() {
        if components[i].0 == 0 {
            bridges.push((vec![i], components[i].1));
        } else if components[i].1 == 0 {
            bridges.push((vec![i], components[i].0));
        }
    }

    let mut bi = 0;
    let mut max_length = 0;
    while bi < bridges.len() {
        if bridges[bi].0.len() > max_length {
            max_length = bridges[bi].0.len();
        }
        let next_number = bridges[bi].1;
        for i in 0..components.len() {
            if bridges[bi].0.contains(&i) {
                continue;
            }
            if components[i].0 == next_number {
                let mut next_bridge = bridges[bi].0.clone();
                next_bridge.push(i);
                bridges.push((next_bridge, components[i].1));
            } else if components[i].1 == next_number {
                let mut next_bridge = bridges[bi].0.clone();
                next_bridge.push(i);
                bridges.push((next_bridge, components[i].0));
            }
        }

        bi += 1;
    }

    let print_debug = true; // if cfg!(debug_assertions) { true } else { false };
    for bridge in bridges {
        let mut points = 0;

        if bridge.0.len() == max_length {
            if print_debug {
                print!("- ");
            }
            for idx in &bridge.0 {
                points += components[*idx].0 + components[*idx].1;
                if print_debug {
                    print!("{:?}  ", components[*idx]);
                }
            }
            if print_debug {
                println!("points:{}", points);
            }
            if points > best {
                best = points;
            }
        }
    }
    println!("Best: {}", best);

    Ok(())
}
