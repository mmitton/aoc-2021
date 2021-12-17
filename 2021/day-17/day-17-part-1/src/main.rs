#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NAN(std::num::ParseIntError),
    NoInput,
    NoSolution,
}

fn load_input(filename: &str) -> Result<(isize, isize, isize, isize), Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }

        let line = &line[15..];
        let line = line.replace("..", " ").replace(", y=", " ");
        let parts = line.split(" ").collect::<Vec<&str>>();
        let x0: isize = parts[0].parse().map_err(|e| Error::NAN(e))?;
        let x1: isize = parts[1].parse().map_err(|e| Error::NAN(e))?;
        let y0: isize = parts[2].parse().map_err(|e| Error::NAN(e))?;
        let y1: isize = parts[3].parse().map_err(|e| Error::NAN(e))?;

        return Ok((x0, x1, y0, y1));
    }

    Err(Error::NoInput)
}

fn main() -> Result<(), Error> {
    let (x0, x1, y0, y1) = load_input(INPUT_FILE)?;
    println!("x:{}..{}  y:{}..{}", x0, x1, y0, y1);

    let mut steps = Vec::new();

    for y_initial in (y0..=(-y0)).rev() {
        'x_loop: for x_initial in 0..2 * x1 {
            steps.clear();
            let mut x = 0isize;
            let mut y = 0isize;

            let mut x_vel = x_initial;
            let mut y_vel = y_initial;
            let mut max_height = 0isize;
            loop {
                x += x_vel;
                y += y_vel;

                if y > max_height {
                    max_height = y;
                }

                steps.push((x, y));

                y_vel -= 1;
                if x_vel > 0 {
                    x_vel -= 1;
                } else if x_vel < 0 {
                    x_vel += 1;
                }

                if y < y0 {
                    continue 'x_loop;
                }
                if y <= y1 && y >= y0 && x <= x1 && x >= x0 {
                    println!(
                        "{},{}  max_height: {}  final: {},{}  steps: {:?}",
                        x_initial,
                        y_initial,
                        max_height,
                        x,
                        y,
                        steps.len()
                    );
                    return Ok(());
                }
            }
        }
    }

    Err(Error::NoSolution)
}
