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

#[derive(Debug)]
struct NanoBot {
    x: isize,
    y: isize,
    z: isize,
    r: isize,
}

fn load_input(filename: &str) -> Result<Vec<NanoBot>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();
    let mut nanobots = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim().to_string();
        if line == "" || line.starts_with("#") {
            continue;
        }

        let parts: Vec<&str> = line.split(">, r=").collect();
        let r = parts[1].parse()?;
        let parts: Vec<&str> = parts[0][5..].split(",").collect();
        let x = parts[0].parse()?;
        let y = parts[1].parse()?;
        let z = parts[2].parse()?;

        nanobots.push(NanoBot {
            x: x,
            y: y,
            z: z,
            r: r,
        });
    }

    Ok(nanobots)
}

fn main() -> Result<(), Error> {
    let nanobots = load_input(INPUT_FILE)?;

    let mut largest_r = 0;
    let mut largest_idx = 0;

    for (idx, nanobot) in nanobots.iter().enumerate() {
        if cfg!(debug_assertions) {
            println!("{}: {:?}", idx, nanobot);
        }
        if nanobot.r > largest_r {
            largest_r = nanobot.r;
            largest_idx = idx;
        }
    }

    println!(
        "Largest NanoBot Range of {} is {:?}",
        largest_r, nanobots[largest_idx]
    );

    let mut answer = 0;
    for idx in 0..nanobots.len() {
        let mut r = (nanobots[idx].x - nanobots[largest_idx].x).abs();
        r += (nanobots[idx].y - nanobots[largest_idx].y).abs();
        r += (nanobots[idx].z - nanobots[largest_idx].z).abs();

        if r <= largest_r {
            answer += 1;
        }
    }

    println!("{} nanobots are in range", answer);
    Ok(())
}
