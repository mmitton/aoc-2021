#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NAN(std::num::ParseIntError),
}

struct List {
    numbers: Vec<usize>,
    pos: usize,
    skip: usize,
}

impl List {
    fn new(size: usize) -> Self {
        let mut numbers: Vec<usize> = Vec::new();

        for i in 0..size {
            numbers.push(i);
        }

        Self {
            numbers: numbers,
            pos: 0,
            skip: 0,
        }
    }

    fn process(&mut self, len: usize) {
        for i in 0..len / 2 {
            let idx1 = (self.pos + i) % self.numbers.len();
            let idx2 = (self.pos + len - i - 1) % self.numbers.len();
            let (a, b) = (self.numbers[idx1], self.numbers[idx2]);
            self.numbers[idx2] = a;
            self.numbers[idx1] = b;
        }
        self.pos = (self.pos + len + self.skip) % self.numbers.len();
        self.skip += 1;
    }
}

fn load_input(filename: &str) -> Result<(usize, Vec<usize>), Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;
    let mut lines = BufReader::new(f).lines().map(|l| l.unwrap());

    let size: usize = lines.next().unwrap().parse().map_err(|e| Error::NAN(e))?;
    let mut lengths: Vec<usize> = Vec::new();

    for len in lines.next().unwrap().split(",") {
        lengths.push(len.parse().map_err(|e| Error::NAN(e))?);
    }

    Ok((size, lengths))
}

fn main() -> Result<(), Error> {
    let (size, lengths) = load_input(INPUT_FILE)?;

    println!("size: {}  lengths: {:?}", size, lengths);

    let mut list = List::new(size);

    for len in &lengths {
        list.process(*len);
        if cfg!(debug_assertions) {
            println!("List: pos:{}  numbers:{:?}", list.pos, list.numbers);
        }
    }

    println!(
        "Answer: {} * {} = {}",
        list.numbers[0],
        list.numbers[1],
        list.numbers[0] * list.numbers[1]
    );

    Ok(())
}
