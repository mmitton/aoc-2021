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
    x: i32,
    y: i32,
    z: i32,
    r: i32,
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

fn num_bots(nanobots: &Vec<NanoBot>, x: i32, y: i32, z: i32) -> usize {
    let mut bots = 0;

    for nanobot in nanobots {
        let dist = (nanobot.x - x).abs() + (nanobot.y - y).abs() + (nanobot.z - z).abs();
        if dist <= nanobot.r {
            bots += 1;
        }
    }

    bots
}

fn main() -> Result<(), Error> {
    let nanobots = load_input(INPUT_FILE)?;

    let mut x0 = i32::MAX;
    let mut x1 = i32::MIN;
    let mut y0 = i32::MAX;
    let mut y1 = i32::MIN;
    let mut z0 = i32::MAX;
    let mut z1 = i32::MIN;

    for nanobot in &nanobots {
        if nanobot.x < x0 {
            x0 = nanobot.x;
        }
        if nanobot.x > x1 {
            x1 = nanobot.x;
        }
        if nanobot.y < y0 {
            y0 = nanobot.y;
        }
        if nanobot.y > y1 {
            y1 = nanobot.y;
        }
        if nanobot.z < z0 {
            z0 = nanobot.z;
        }
        if nanobot.z > z1 {
            z1 = nanobot.z;
        }
    }

    println!("{},{},{} => {},{},{}", x0, y0, z0, x1, y1, z1);
    println!("{},{},{}", x1 - x0, y1 - y0, z1 - z0);
    let x_step = (x1 - x0) / 100;
    let y_step = (y1 - y0) / 100;
    let z_step = (z1 - z0) / 100;
    let step = if x_step <= y_step && x_step <= z_step {
        x_step
    } else if y_step <= x_step && y_step <= z_step {
        y_step
    } else {
        z_step
    } as usize;

    use std::collections::VecDeque;
    let mut queue = VecDeque::new();
    queue.push_front((step, x0, y0, z0, x1, y1, z1));

    let mut best = 0;
    loop {
        let mut results = Vec::new();
        let mut cur_step = 0;

        let mut best_coord: Option<(i32, i32, i32)> = None;
        let mut best_dist = i32::MAX;

        while queue.len() > 0 {
            let (step, x0, y0, z0, x1, y1, z1) = queue.pop_front().unwrap();
            cur_step = step;

            for z in (z0..=z1).step_by(step) {
                for y in (y0..=y1).step_by(step) {
                    for x in (x0..=x1).step_by(step) {
                        let num = num_bots(&nanobots, x, y, z);
                        if num >= best {
                            if num > best {
                                best = num;
                                println!("new best: {}", best);
                                results.clear();
                                best_coord = None;
                                best_dist = i32::MAX;
                            }
                            if step == 1 {
                                let dist = x.abs() + y.abs() + z.abs();
                                if dist < best_dist {
                                    println!("New best coord: {:?} @ {}", (x, y, z), dist);
                                    best_coord = Some((x, y, z));
                                    best_dist = dist;
                                }
                            } else {
                                results.push((x, y, z));
                            }
                        }
                    }
                }
            }
        }

        if let Some(best_coord) = best_coord {
            println!("Done  {:?}  {}", best_coord, best_dist);
            return Ok(());
        }

        let mut new_step = cur_step / 10;
        if new_step == 0 {
            new_step = 1;
        }
        println!(
            "best: {}  cur_step:{}  {}  next step:{}",
            best,
            cur_step,
            results.len(),
            new_step,
        );
        assert!(results.len() > 0);

        for result in &results {
            let (x, y, z) = *result;
            let x0 = x - (cur_step / 2) as i32;
            let x1 = x + (cur_step / 2) as i32;
            let y0 = y - (cur_step / 2) as i32;
            let y1 = y + (cur_step / 2) as i32;
            let z0 = z - (cur_step / 2) as i32;
            let z1 = z + (cur_step / 2) as i32;
            queue.push_back((new_step, x0, y0, z0, x1, y1, z1));
        }
    }
}
