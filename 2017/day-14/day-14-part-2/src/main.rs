#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
}

struct Map {
    blocks: Vec<Vec<bool>>,
    used_blocks: usize,
}

impl Map {
    fn new() -> Self {
        Self {
            blocks: Vec::new(),
            used_blocks: 0,
        }
    }

    fn add(&mut self, hash: &Vec<u8>) {
        let mut row = Vec::with_capacity(128);

        for d in hash {
            self.used_blocks += d.count_ones() as usize;
            row.extend_from_slice(&[
                d & 0b1000_0000 != 0,
                d & 0b0100_0000 != 0,
                d & 0b0010_0000 != 0,
                d & 0b0001_0000 != 0,
                d & 0b0000_1000 != 0,
                d & 0b0000_0100 != 0,
                d & 0b0000_0010 != 0,
                d & 0b0000_0001 != 0,
            ]);
        }

        self.blocks.push(row);
    }

    fn print(&self) {
        for row in &self.blocks {
            for used in row {
                if *used {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn regions(&self) -> usize {
        fn mark_region_used(blocks: &mut Vec<Vec<bool>>, x: usize, y: usize) {
            if !blocks[y][x] {
                return;
            }
            blocks[y][x] = false;
            if x > 0 {
                mark_region_used(blocks, x - 1, y);
            }
            if x < 128 - 1 {
                mark_region_used(blocks, x + 1, y);
            }
            if y > 0 {
                mark_region_used(blocks, x, y - 1);
            }
            if y < blocks.len() - 1 {
                mark_region_used(blocks, x, y + 1);
            }
        }

        let mut regions = 0;
        let mut blocks = self.blocks.clone();
        for y in 0..blocks.len() {
            for x in 0..128 {
                if blocks[y][x] {
                    regions += 1;
                    mark_region_used(&mut blocks, x, y);
                }
            }
        }

        regions
    }
}

struct List {
    numbers: Vec<u8>,
    pos: usize,
    skip: usize,
}

impl List {
    fn new(size: usize) -> Self {
        let mut numbers: Vec<u8> = Vec::new();

        for i in 0..size {
            numbers.push(i as u8);
        }

        Self {
            numbers: numbers,
            pos: 0,
            skip: 0,
        }
    }

    fn process(&mut self, len: u8) {
        for i in 0..len / 2 {
            let idx1 = (self.pos + i as usize) % self.numbers.len();
            let idx2 = (self.pos + (len - i - 1) as usize) % self.numbers.len();
            let (a, b) = (self.numbers[idx1], self.numbers[idx2]);
            self.numbers[idx2] = a;
            self.numbers[idx1] = b;
        }
        self.pos = (self.pos + len as usize + self.skip) % self.numbers.len();
        self.skip += 1;
    }
}

fn load_input(filename: &str) -> Result<Vec<u8>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;
    let mut lines = BufReader::new(f).lines().map(|l| l.unwrap());

    let mut lengths: Vec<u8> = Vec::new();

    for len in lines.next().unwrap().chars() {
        lengths.push(len as u8);
    }

    Ok(lengths)
}

fn new_key(lengths: &Vec<u8>, idx: usize) -> Vec<u8> {
    let mut key = lengths.clone();

    for c in format!("-{}", idx).chars() {
        key.push(c as u8);
    }

    key.extend_from_slice(&[17, 31, 73, 47, 23]);
    key
}

fn get_hash(key: &Vec<u8>) -> Vec<u8> {
    let mut list = List::new(256);

    for _ in 0..64 {
        for len in key {
            list.process(*len);
        }
    }

    let mut hash = Vec::new();
    for block in (0..255).step_by(16) {
        let mut xor = 0u8;
        for idx in block..block + 16 {
            xor ^= list.numbers[idx];
        }
        hash.push(xor);
    }

    hash
}

fn main() -> Result<(), Error> {
    let input = load_input(INPUT_FILE)?;

    let mut map = Map::new();

    for i in 0..128 {
        let key = new_key(&input, i);
        let hash = get_hash(&key);

        map.add(&hash);
    }

    if cfg!(debug_assertions) {
        map.print();
    }

    println!("{}", map.blocks[0].len());
    println!("Used Blocks: {}", map.used_blocks);
    println!("Used Regions: {}", map.regions());

    Ok(())
}
