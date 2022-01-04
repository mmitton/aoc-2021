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

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::IO(e)
    }
}

fn load_input(filename: &str) -> Result<Vec<Vec<(usize, usize)>>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let reader = BufReader::new(f);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    let mut inputs = Vec::new();

    for line in lines {
        if line.contains(",") {
            let mut busses: Vec<(usize, usize)> = Vec::new();
            for (idx, part) in line.split(",").enumerate() {
                if part == "x" {
                    continue;
                }
                busses.push((idx, part.parse()?));
            }
            inputs.push(busses);
        }
    }

    Ok(inputs)
}

fn main() -> Result<(), Error> {
    let inputs = load_input(INPUT_FILE)?;

    for busses in &inputs {
        println!("busses: {:?}", busses);
        let mut max_idx = 0;
        let mut max_bus = 0;
        for bus in busses {
            if bus.1 > max_bus {
                max_bus = bus.1;
                max_idx = bus.0;
            }
        }

        let mut step = max_bus;
        for bus in busses {
            if (bus.0 as isize - max_idx as isize).abs() as usize == bus.1 {
                step *= bus.1;
            }
        }

        'start_loop: for start in (step..usize::MAX).step_by(step) {
            for i in 0..busses.len() {
                let trips = (start + busses[i].1 + busses[i].0 - max_idx - 1) / busses[i].1;
                let arrive_at = trips * busses[i].1;
                if arrive_at != start + busses[i].0 - max_idx {
                    continue 'start_loop;
                }
            }

            let start = start - max_idx;
            println!("earliest timestamp: {}", start);
            break;
        }
    }
    /*
    for bus in &busses {
        let trips = (min_time + bus - 1) / bus;
        let arrive_at = trips * bus;
        let wait_time = arrive_at - min_time;
        if wait_time < min_wait_time {
            min_wait_time = wait_time;
            min_bus = *bus;
        }
    }

    println!(
        "Bus {}, Wait Time {}...  Answer: {}",
        min_bus,
        min_wait_time,
        min_bus * min_wait_time
    );
    */
    Ok(())
}
