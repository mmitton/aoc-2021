#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NotEnoughBits(usize),
}

struct Binary {
    bits: Vec<bool>,
    pos: usize,
}

impl std::fmt::Debug for Binary {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        for n in self.pos..self.bits.len() {
            write!(fmt, "{}", if self.bits[n] { '1' } else { '0' })?;
        }
        Ok(())
    }
}

impl Binary {
    fn new() -> Self {
        Self {
            bits: Vec::new(),
            pos: 0,
        }
    }

    fn push(&mut self, b: bool) {
        self.bits.push(b);
    }

    fn left(&self) -> usize {
        self.bits.len() - self.pos
    }

    fn remove(&mut self, n: usize) -> Result<Self, Error> {
        if self.pos + n > self.bits.len() {
            return Err(Error::NotEnoughBits(n));
        }

        let mut child = Self {
            bits: vec![false; n],
            pos: 0,
        };
        child
            .bits
            .copy_from_slice(&self.bits[self.pos..self.pos + n]);
        self.pos += n;

        Ok(child)
    }

    fn take(&mut self, n: usize) -> Option<usize> {
        if self.pos + n > self.bits.len() {
            return None;
        }

        let mut res = 0;
        for _ in 0..n {
            res <<= 1;
            res |= if self.bits[self.pos] { 1 } else { 0 };
            self.pos += 1
        }

        Some(res)
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Packet {
    version: usize,
    type_id: usize,
    literal: Option<usize>,
    sub_packets: Vec<Packet>,
}

impl Packet {
    fn new(b: &mut Binary) -> Result<Self, Error> {
        let version = b.take(3).unwrap();
        let type_id = b.take(3).unwrap();
        let mut literal = None;
        let mut sub_packets = Vec::new();

        match type_id {
            4 => {
                // Literal
                let mut last_chunk = false;
                let mut l = 0usize;
                while !last_chunk {
                    last_chunk = b.take(1).unwrap() == 0;
                    l <<= 4;
                    l |= b.take(4).unwrap();
                }
                literal = Some(l);
            }
            _ => {
                let length_type = b.take(1).unwrap();
                if length_type == 0 {
                    // len is total length in bits of the sub packets
                    let len = b.take(15).unwrap();
                    let mut sub_b = b.remove(len).unwrap();
                    while sub_b.left() > 6 {
                        let sub_packet = Self::new(&mut sub_b)?;
                        sub_packets.push(sub_packet);
                    }
                } else {
                    // len is the total number of sub packets
                    let len = b.take(11).unwrap();
                    for _ in 0..len {
                        let sub_packet = Self::new(b)?;
                        sub_packets.push(sub_packet);
                    }
                }
            }
        }

        return Ok(Packet {
            version: version,
            type_id: type_id,
            literal: literal,
            sub_packets: sub_packets,
        });
    }

    fn version_sum(&self) -> usize {
        let mut version_sum = self.version;
        for sub in &self.sub_packets {
            version_sum += sub.version_sum();
        }

        version_sum
    }
}

fn load_input(filename: &str) -> Result<Vec<Packet>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();

    let mut packets = Vec::new();
    let hex = vec![
        vec![false, false, false, false],
        vec![false, false, false, true],
        vec![false, false, true, false],
        vec![false, false, true, true],
        vec![false, true, false, false],
        vec![false, true, false, true],
        vec![false, true, true, false],
        vec![false, true, true, true],
        vec![true, false, false, false],
        vec![true, false, false, true],
        vec![true, false, true, false],
        vec![true, false, true, true],
        vec![true, true, false, false],
        vec![true, true, false, true],
        vec![true, true, true, false],
        vec![true, true, true, true],
    ];

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }

        let mut binary = Binary::new();
        for c in line.chars() {
            let h = match c {
                '0' => &hex[0],
                '1' => &hex[1],
                '2' => &hex[2],
                '3' => &hex[3],
                '4' => &hex[4],
                '5' => &hex[5],
                '6' => &hex[6],
                '7' => &hex[7],
                '8' => &hex[8],
                '9' => &hex[9],
                'A' => &hex[10],
                'B' => &hex[11],
                'C' => &hex[12],
                'D' => &hex[13],
                'E' => &hex[14],
                'F' => &hex[15],
                _ => panic!(),
            };
            for b in h {
                binary.push(*b);
            }
        }

        let packet = Packet::new(&mut binary)?;
        packets.push(packet);
    }

    Ok(packets)
}

fn main() -> Result<(), Error> {
    let packets = load_input(INPUT_FILE)?;

    for packet in &packets {
        if cfg!(debug_assertions) {
            println!("packet: {:?}", packet);
        }
        println!("Version Sum: {:?}", packet.version_sum());
    }

    Ok(())
}
