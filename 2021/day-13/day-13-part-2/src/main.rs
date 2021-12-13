#![feature(drain_filter)]
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
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Points(Vec<Point>);

impl Points {
    fn get(&self, x: usize, y: usize) -> bool {
        for p in &self.0 {
            if p.x == x && p.y == y {
                return true;
            }
        }

        false
    }

    fn print(&self) {
        let mut width = 0usize;
        let mut height = 0usize;

        for p in &self.0 {
            if p.x + 1 > width {
                width = p.x + 1
            }
            if p.y + 1 > height {
                height = p.y + 1
            }
        }

        for y in 0..height {
            for x in 0..width {
                print!("{}", if self.get(x, y) { '#' } else { ' ' });
            }
            println!();
        }
    }

    fn fold(&mut self, fold: &Fold) {
        for p in &mut self.0 {
            match fold.axis {
                Axis::X if p.x > fold.pos => p.x = (2 * fold.pos) - p.x,
                Axis::Y if p.y > fold.pos => p.y = (2 * fold.pos) - p.y,
                _ => {}
            }
        }

        for i in (1..self.0.len()).rev() {
            for j in (0..i).rev() {
                if self.0[i].x == self.0[j].x && self.0[i].y == self.0[j].y {
                    self.0.remove(i);
                    break;
                }
            }
        }
    }
}

#[derive(Debug)]
enum Axis {
    X,
    Y,
}

#[derive(Debug)]
struct Fold {
    axis: Axis,
    pos: usize,
}

fn load_input(filename: &str) -> Result<(Points, Vec<Fold>), Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();

    let mut points = Points(Vec::new());
    let mut folds = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }

        if line.starts_with("fold along ") {
            let a = line.chars().nth(11).unwrap();
            let axis = match a {
                'x' => Axis::X,
                'y' => Axis::Y,
                _ => panic!("wrong letter: {}", a),
            };
            let pos = line[13..].parse::<usize>().map_err(|e| Error::NAN(e))?;
            folds.push(Fold {
                axis: axis,
                pos: pos,
            });
        } else {
            let parts = line.split(",").collect::<Vec<&str>>();

            let x = parts[0].parse::<usize>().map_err(|e| Error::NAN(e))?;
            let y = parts[1].parse::<usize>().map_err(|e| Error::NAN(e))?;

            points.0.push(Point { x: x, y: y });
        }
    }

    Ok((points, folds))
}

fn main() -> Result<(), Error> {
    let (mut points, folds) = load_input(INPUT_FILE)?;

    if cfg!(debug_assertions) {
        points.print();
        println!("Folds: {:?}", folds);
    }

    for fold in &folds {
        points.fold(fold);
    }

    points.print();

    Ok(())
}
