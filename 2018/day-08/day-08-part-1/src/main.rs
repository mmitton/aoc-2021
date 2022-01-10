#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

use std::collections::VecDeque;

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NAN(std::num::ParseIntError),
    MissingData,
    ExtraData(VecDeque<usize>),
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Error {
        Error::NAN(e)
    }
}

struct Node {
    children: Vec<Node>,
    meta: Vec<usize>,
}

impl Node {
    fn from_vecdeque(buf: &mut VecDeque<usize>) -> Result<Node, Error> {
        let num_children = buf.pop_front().ok_or(Error::MissingData)?;
        let num_meta = buf.pop_front().ok_or(Error::MissingData)?;

        let mut children = Vec::new();
        let mut meta = Vec::new();

        for _ in 0..num_children {
            children.push(Node::from_vecdeque(buf)?);
        }

        for _ in 0..num_meta {
            meta.push(buf.pop_front().ok_or(Error::MissingData)?);
        }

        Ok(Self {
            children: children,
            meta: meta,
        })
    }

    fn meta_sum(&self) -> usize {
        let mut meta_sum = 0;

        for meta in &self.meta {
            meta_sum += *meta;
        }
        for child in &self.children {
            meta_sum += child.meta_sum();
        }

        meta_sum
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
        let line = line.trim().to_string();
        if line == "" || line.starts_with("#") {
            continue;
        }

        let mut buf: VecDeque<usize> = VecDeque::new();
        for num in line.split(" ") {
            buf.push_back(num.parse()?);
        }

        nodes.push(Node::from_vecdeque(&mut buf)?);
        if buf.len() > 0 {
            return Err(Error::ExtraData(buf));
        }
    }

    Ok(nodes)
}

fn main() -> Result<(), Error> {
    let nodes = load_input(INPUT_FILE)?;

    for node in &nodes {
        println!("Meta Sum: {}", node.meta_sum());
    }

    Ok(())
}
