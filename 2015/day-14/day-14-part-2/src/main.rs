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
    score: usize,
}

impl Reindeer {
    fn distance_after(&self, s: usize) -> usize {
        let seconds_per_iter = self.speed_sec + self.cooldown;
        let iters = s / seconds_per_iter;
        let leftover = s % seconds_per_iter;

        let mut dist = iters * (self.speed * self.speed_sec);
        if leftover > self.speed_sec {
            dist += self.speed * self.speed_sec;
        } else {
            dist += self.speed * leftover;
        }

        dist
    }
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
            score: 0,
        };

        reindeers.push(reindeer);
    }

    Ok(reindeers)
}

fn main() -> Result<(), Error> {
    let mut reindeers = load_input(INPUT_FILE)?;
    const TARGET: usize = 2503;

    for s in 1..=TARGET {
        let mut best_idx = Vec::new();
        let mut best: usize = 0;
        for idx in 0..reindeers.len() {
            let dist = reindeers[idx].distance_after(s);

            if dist > best {
                best = dist;
                best_idx.clear();
                best_idx.push(idx);
            } else if dist == best {
                best_idx.push(idx);
            }
        }

        for idx in &best_idx {
            reindeers[*idx].score += 1;
        }
    }

    let mut best = 0usize;
    for reindeer in &reindeers {
        println!("{} : {}", reindeer.name, reindeer.score);
        if reindeer.score > best {
            best = reindeer.score;
        }
    }
    println!("best: {}", best);

    Ok(())
}
