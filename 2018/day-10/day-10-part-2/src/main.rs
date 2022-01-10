#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NAN(std::num::ParseIntError),
    NoSolution,
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Error {
        Error::NAN(e)
    }
}

struct Point {
    x: isize,
    y: isize,
    dx: isize,
    dy: isize,
}

fn load_input(filename: &str) -> Result<Vec<Point>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();
    let mut points = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim().to_string();
        if line == "" || line.starts_with("#") {
            continue;
        }

        let line = line.replace("<", ",");
        let line = line.replace(">", ",");
        let parts: Vec<&str> = line.split(",").collect();
        let x: isize = parts[1].trim().parse()?;
        let y: isize = parts[2].trim().parse()?;
        let dx: isize = parts[4].trim().parse()?;
        let dy: isize = parts[5].trim().parse()?;
        points.push(Point {
            x: x,
            y: y,
            dx: dx,
            dy: dy,
        });
    }

    Ok(points)
}

fn get_area(points: &Vec<Point>, t: isize) -> (isize, isize, isize, isize) {
    let mut x0 = isize::MAX;
    let mut x1 = isize::MIN;
    let mut y0 = isize::MAX;
    let mut y1 = isize::MIN;

    for p in points {
        let x = p.x + (p.dx * t);
        let y = p.y + (p.dy * t);
        if x < x0 {
            x0 = x;
        }
        if x > x1 {
            x1 = x;
        }
        if y < y0 {
            y0 = y;
        }
        if y > y1 {
            y1 = y;
        }
    }

    (x0, y0, x1, y1)
}

fn print_message(points: &Vec<Point>, t: isize) {
    use std::collections::BTreeSet;

    let mut x0 = isize::MAX;
    let mut x1 = isize::MIN;
    let mut y0 = isize::MAX;
    let mut y1 = isize::MIN;
    let mut new_points = BTreeSet::new();

    for p in points {
        let x = p.x + (p.dx * t);
        let y = p.y + (p.dy * t);
        if x < x0 {
            x0 = x;
        }
        if x > x1 {
            x1 = x;
        }
        if y < y0 {
            y0 = y;
        }
        if y > y1 {
            y1 = y;
        }
        new_points.insert((x, y));
    }

    for y in y0..=y1 {
        for x in x0..=x1 {
            if new_points.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn main() -> Result<(), Error> {
    let points = load_input(INPUT_FILE)?;

    let mut area = isize::MAX;
    for t in 0..isize::MAX {
        let (x0, y0, x1, y1) = get_area(&points, t);
        let t_area = (x1 - x0) * (y1 - y0);
        if t_area > area {
            print_message(&points, t - 1);
            println!("Took {} seconds", t - 1);
            return Ok(());
        } else {
            area = t_area;
        }
    }

    Err(Error::NoSolution)
}
