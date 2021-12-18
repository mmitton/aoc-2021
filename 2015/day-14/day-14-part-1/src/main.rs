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
struct Reindeer {
    name: String,
    speed: usize,
    speed_sec: usize,
    cooldown: usize,
}

fn load_input(filename: &str) -> Result<Vec<Reindeer>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();

    let mut reindeers = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }

        let line = line.replace(" can fly ", " ");
        let line = line.replace(" km/s for ", " ");
        let line = line.replace(" seconds, but then must rest for ", " ");
        let line = line.replace(" seconds.", "");

        let parts: Vec<&str> = line.split(" ").collect();
        let reindeer = Reindeer {
            name: parts[0].to_string(),
            speed: parts[1].parse().map_err(|e| Error::NAN(e))?,
            speed_sec: parts[2].parse().map_err(|e| Error::NAN(e))?,
            cooldown: parts[3].parse().map_err(|e| Error::NAN(e))?,
        };

        reindeers.push(reindeer);
    }

    Ok(reindeers)
}

fn main() -> Result<(), Error> {
    let reindeers = load_input(INPUT_FILE)?;
    const TARGET: usize = 2503;

    let mut best: usize = 0;
    for reindeer in &reindeers {
        let seconds_per_iter = reindeer.speed_sec + reindeer.cooldown;
        let iters = TARGET / seconds_per_iter;
        let leftover = TARGET % seconds_per_iter;

        let mut dist = iters * (reindeer.speed * reindeer.speed_sec);
        if leftover > reindeer.speed_sec {
            dist += reindeer.speed * reindeer.speed_sec;
        } else {
            dist += reindeer.speed * leftover;
        }
        if dist > best {
            best = dist;
        }

        println!("reindeer: {:?}  dist: {}", reindeer, dist);
    }

    println!("best: {}", best);

    Ok(())
}
