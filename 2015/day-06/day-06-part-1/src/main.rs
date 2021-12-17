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
enum OnOff {
    On,
    Off,
    Toggle,
}

#[derive(Debug)]
struct Instruction {
    on_off: OnOff,
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
}

fn load_input(filename: &str) -> Result<Vec<Instruction>, Error> {
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

        let (on_off, line) = if line.starts_with("turn on") {
            (OnOff::On, &line[8..])
        } else if line.starts_with("turn off") {
            (OnOff::Off, &line[9..])
        } else {
            (OnOff::Toggle, &line[7..])
        };

        let line = line.replace(" through ", " ");
        let line = line.replace(",", " ");
        let parts: Vec<&str> = line.split(" ").collect();

        inputs.push(Instruction {
            on_off: on_off,
            x1: parts[0].parse().map_err(|e| Error::NAN(e))?,
            y1: parts[1].parse().map_err(|e| Error::NAN(e))?,
            x2: parts[2].parse().map_err(|e| Error::NAN(e))?,
            y2: parts[3].parse().map_err(|e| Error::NAN(e))?,
        });
    }

    Ok(inputs)
}

fn main() -> Result<(), Error> {
    let inputs = load_input(INPUT_FILE)?;

    let mut grid = [false; 1000 * 1000];
    for input in &inputs {
        println!("input: {:?}", input);
        for y in input.y1..=input.y2 {
            for x in input.x1..=input.x2 {
                let idx = ((y * 1000) + x) as usize;
                match input.on_off {
                    OnOff::On => grid[idx] = true,
                    OnOff::Off => grid[idx] = false,
                    OnOff::Toggle => grid[idx] = !grid[idx],
                }
            }
        }
    }

    let mut total = 0;
    for light in &grid {
        if *light {
            total += 1;
        }
    }

    println!("Total On: {}", total);
    Ok(())
}
