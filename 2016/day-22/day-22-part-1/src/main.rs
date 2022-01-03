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

struct Node {
    x: isize,
    y: isize,
    size: usize,
    used: usize,
    avail: usize,
}

impl std::fmt::Debug for Node {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            fmt,
            "/dev/grid/node-x{}-y{}\t{}T\t{}T\t{}T",
            self.x, self.y, self.size, self.used, self.avail
        )
    }
}

impl TryFrom<&str> for Node {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut s = s.to_string();
        for _ in 0..5 {
            s = s.replace("  ", " ");
        }

        fn parse_size(s: &str) -> Result<usize, Error> {
            assert!(&s[s.len() - 1..] == "T");
            Ok(s[..s.len() - 1].parse()?)
        }

        let parts = s.split(" ").collect::<Vec<&str>>();
        let name = parts[0].split("-").collect::<Vec<&str>>();
        let x = name[1][1..].parse()?;
        let y = name[2][1..].parse()?;
        let size = parse_size(parts[1])?;
        let used = parse_size(parts[2])?;
        let avail = parse_size(parts[3])?;
        Ok(Self {
            x: x,
            y: y,
            size: size,
            used: used,
            avail: avail,
        })
    }
}

fn load_input(filename: &str) -> Result<Vec<Node>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();
    let mut nodes = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }

        if line.starts_with("/dev/grid/") {
            nodes.push(line.try_into()?);
        }
    }

    Ok(nodes)
}

fn main() -> Result<(), Error> {
    let nodes = load_input(INPUT_FILE)?;

    let mut viable = 0;
    for i in 0..nodes.len() {
        for j in 0..nodes.len() {
            if i == j || nodes[i].used == 0 {
                continue;
            }
            if nodes[i].used <= nodes[j].avail {
                viable += 1;
                if cfg!(debug_assertions) {
                    println!(
                        "Node {},{} is viable with {},{}",
                        nodes[i].x, nodes[i].y, nodes[j].x, nodes[j].y
                    );
                }
            }
        }
    }

    println!("Viable Pairs: {}", viable);

    Ok(())
}
