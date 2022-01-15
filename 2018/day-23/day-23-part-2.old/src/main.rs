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

#[derive(Debug, Copy, Clone)]
struct Point(isize, isize, isize);

impl Point {
    fn dist(&self, other: &Point) -> isize {
        (self.0 - other.0).abs() + (self.1 - other.1).abs() + (self.2 - other.2).abs()
    }
}

#[derive(Debug)]
struct NanoBot {
    p: Point,
    r: isize,
}

#[derive(Debug)]
struct Octahedron {
    vertices: [Point; 6],
    faces: [[Point; 3]; 8],
    nanobots: Vec<usize>,
}

impl Octahedron {
    fn new(id: usize, nanobot: &NanoBot) -> Self {
        let vertices = [
            Point(nanobot.p.0 - nanobot.r, nanobot.p.1, nanobot.p.2),
            Point(nanobot.p.0, nanobot.p.1 - nanobot.r, nanobot.p.2),
            Point(nanobot.p.0, nanobot.p.1, nanobot.p.2 - nanobot.r),
            Point(nanobot.p.0, nanobot.p.1 + nanobot.r, nanobot.p.2),
            Point(nanobot.p.0, nanobot.p.1, nanobot.p.2 + nanobot.r),
            Point(nanobot.p.0 + nanobot.r, nanobot.p.1, nanobot.p.2),
        ];
        let faces = [
            [vertices[0], vertices[1], vertices[2]],
            [vertices[0], vertices[2], vertices[3]],
            [vertices[0], vertices[3], vertices[4]],
            [vertices[0], vertices[4], vertices[1]],
            [vertices[5], vertices[1], vertices[2]],
            [vertices[5], vertices[2], vertices[3]],
            [vertices[5], vertices[3], vertices[4]],
            [vertices[5], vertices[4], vertices[1]],
        ];
        Self {
            vertices: vertices,
            faces: faces,
            nanobots: vec![id],
        }
    }
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
            p: Point(x, y, z),
            r: r,
        });
    }

    Ok(nanobots)
}

fn main() -> Result<(), Error> {
    let nanobots = load_input(INPUT_FILE)?;

    let mut octahedrons = Vec::new();
    for (i, nanobot) in nanobots.iter().enumerate() {
        let octahedron = Octahedron::new(i, nanobot);
        octahedrons.push(octahedron);
    }

    for octahedron in &octahedrons {
        println!("{:?}", octahedron);
    }

    Ok(())
}
