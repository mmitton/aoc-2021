#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NAN(std::num::ParseIntError),
}

#[derive(Debug)]
struct Layer {
    num: usize,
    depth: usize,
}

impl Layer {
    fn get_position(&self, t: usize) -> usize {
        if self.depth == 0 {
            return usize::MAX;
        }
        if self.depth == 1 {
            return 0;
        }
        let dist = (2 * self.depth) - 2;
        let mut pos = t % dist;
        if pos >= self.depth {
            pos = dist - pos;
        }

        pos
    }
}

fn load_input(filename: &str) -> Result<Vec<Layer>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();
    let mut layers: Vec<Layer> = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }
        if line.starts_with("#") {
            continue;
        }

        let parts: Vec<&str> = line.split(": ").collect();
        let layer = Layer {
            num: parts[0].parse().map_err(|e| Error::NAN(e))?,
            depth: parts[1].parse().map_err(|e| Error::NAN(e))?,
        };

        layers.push(layer);
    }

    Ok(layers)
}

fn trip_cost(start: usize, layers: &Vec<Layer>) -> (bool, usize) {
    let mut cost = 0;
    let mut caught = false;
    for layer in layers {
        let pos = layer.get_position(start + layer.num);
        if pos == 0 {
            println!("{} {}", layer.num, layer.depth);
            cost += layer.num * layer.depth;
            caught = true;
        }
    }

    (caught, cost)
}

fn main() -> Result<(), Error> {
    let layers = load_input(INPUT_FILE)?;

    if cfg!(debug_assertions) {
        println!("Layers: {:?}", layers);
    }

    let (_, cost) = trip_cost(0, &layers);
    println!("Trip Cost at t[0]: {}", cost);

    Ok(())
}
