#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

use std::collections::BTreeMap;

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NAN(std::num::ParseIntError),
    InvalidInput(String),
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

#[derive(Debug)]
enum Op {
    Mask(Vec<char>),
    Mem(usize, usize),
}

fn load_input(filename: &str) -> Result<Vec<Op>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let reader = BufReader::new(f);
    let lines = reader.lines();
    let mut ops = Vec::new();

    for line in lines {
        let line = line.unwrap();
        let line = line.trim();
        if line == "" || line.starts_with("#") {
            continue;
        }

        if line.starts_with("mask = ") {
            let mask: Vec<char> = line[7..].chars().collect();
            assert!(mask.len() == 36);
            ops.push(Op::Mask(mask));
        } else if line.starts_with("mem[") {
            let line = line.replace("]", "");
            let parts = line[4..].split(" = ").collect::<Vec<&str>>();
            let addr = parts[0].parse()?;
            let num = parts[1].parse()?;

            ops.push(Op::Mem(addr, num));
        } else {
            return Err(Error::InvalidInput(line.to_string()));
        }
    }

    Ok(ops)
}

#[derive(Debug)]
struct AddrMask {
    mask: Vec<char>,
}

impl AddrMask {
    fn new(addr: usize, mask: &Vec<char>) -> Self {
        let mut addr_mask = Vec::with_capacity(36);
        for i in 0..36 {
            match mask[i] {
                'X' => {
                    addr_mask.push('X');
                }
                '1' => addr_mask.push('1'),
                '0' => addr_mask.push(if addr >> (35 - i) & 0b1 == 0b1 {
                    '1'
                } else {
                    '0'
                }),
                _ => unreachable!(),
            }
        }
        assert!(addr_mask.len() == 36);

        Self { mask: addr_mask }
    }

    fn addresses(&self) -> Vec<usize> {
        let mut addrs = Vec::new();
        addrs.push(0);
        for i in 0..36 {
            for j in 0..addrs.len() {
                addrs[j] <<= 1;
            }
            match self.mask[i] {
                '0' => {}
                '1' => {
                    for j in 0..addrs.len() {
                        addrs[j] |= 1;
                    }
                }
                'X' => {
                    for j in 0..addrs.len() {
                        let new_addr = addrs[j] | 1;
                        addrs.push(new_addr);
                    }
                }
                _ => unreachable!(),
            }
        }

        addrs
    }
}

fn main() -> Result<(), Error> {
    let ops = load_input(INPUT_FILE)?;

    // let mut mem = BTreeMap::new();
    let mut mask = vec!['X'; 36];
    let mut mem: BTreeMap<usize, usize> = BTreeMap::new();

    for op in &ops {
        match op {
            Op::Mask(m) => {
                mask.copy_from_slice(&m);
            }
            Op::Mem(addr, num) => {
                let addr_mask = AddrMask::new(*addr, &mask);
                for addr in addr_mask.addresses() {
                    mem.insert(addr, *num);
                }
            }
        }
    }

    let mut sum = 0;
    for (_, num) in mem.iter() {
        sum += num;
    }

    println!("Sum : {}", sum);
    Ok(())
}
