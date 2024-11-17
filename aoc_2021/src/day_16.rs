#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

pub struct Day16 {
    bit_buffer: BitBuffer,
}

#[derive(Debug)]
struct Packet {
    version: usize,
    len: usize,
    payload: Payload,
}

impl Packet {
    fn new(bit_buffer: &mut BitBuffer) -> Self {
        let version = bit_buffer.bits(3);
        let (payload, len) = Payload::new(bit_buffer);
        Self {
            version,
            len: len + 3,
            payload,
        }
    }

    fn eval(&self) -> (usize, usize) {
        let payload_eval = self.payload.eval();
        (payload_eval.0, payload_eval.1 + self.version)
    }
}

#[derive(Debug)]
enum Payload {
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Min(Vec<Packet>),
    Max(Vec<Packet>),
    Literal(usize),
    GreaterThan(Vec<Packet>),
    LessThan(Vec<Packet>),
    EqualTo(Vec<Packet>),
}

impl Payload {
    fn new(bit_buffer: &mut BitBuffer) -> (Self, usize) {
        fn subpackets(bit_buffer: &mut BitBuffer) -> (Vec<Packet>, usize) {
            let mut bit_count = None;
            let mut packet_count = None;
            let l = bit_buffer.bits(1);
            let mut len = 1;
            if l == 0 {
                bit_count = Some(bit_buffer.bits(15) + 16);
                len += 15;
            } else {
                packet_count = Some(bit_buffer.bits(11));
                len += 11;
            }
            let mut subpackets = Vec::new();
            loop {
                let subpacket = Packet::new(bit_buffer);
                // println!("Subpacket: {subpacket:?}");
                len += subpacket.len;
                subpackets.push(subpacket);
                match (bit_count, packet_count) {
                    (Some(bit_count), None) => {
                        // println!("** {bit_count} {len}");
                        if bit_count == len {
                            break;
                        }
                    }
                    (None, Some(packet_count)) => {
                        if subpackets.len() == packet_count {
                            break;
                        }
                    }
                    _ => unreachable!(),
                }
            }
            (subpackets, len)
        }
        match bit_buffer.bits(3) {
            0 => {
                let (subpackets, len) = subpackets(bit_buffer);
                (Self::Sum(subpackets), len + 3)
            }
            1 => {
                let (subpackets, len) = subpackets(bit_buffer);
                (Self::Product(subpackets), len + 3)
            }
            2 => {
                let (subpackets, len) = subpackets(bit_buffer);
                (Self::Min(subpackets), len + 3)
            }
            3 => {
                let (subpackets, len) = subpackets(bit_buffer);
                (Self::Max(subpackets), len + 3)
            }
            4 => {
                let mut literal = 0;
                let mut len = 0;
                loop {
                    let v = bit_buffer.bits(5);
                    literal = (literal << 4) | (v & 0b1111);
                    len += 5;
                    if v & 0b10000 == 0 {
                        break;
                    }
                }
                (Self::Literal(literal), len + 3)
            }
            5 => {
                let (subpackets, len) = subpackets(bit_buffer);
                (Self::GreaterThan(subpackets), len + 3)
            }
            6 => {
                let (subpackets, len) = subpackets(bit_buffer);
                (Self::LessThan(subpackets), len + 3)
            }
            7 => {
                let (subpackets, len) = subpackets(bit_buffer);
                (Self::EqualTo(subpackets), len + 3)
            }
            _ => unreachable!(),
        }
    }

    fn eval(&self) -> (usize, usize) {
        match self {
            Self::Sum(subpackets) => {
                let mut v = 0;
                let mut version_sum = 0;
                for subpacket in subpackets.iter() {
                    let eval = subpacket.eval();
                    v += eval.0;
                    version_sum += eval.1
                }
                (v, version_sum)
            }
            Self::Product(subpackets) => {
                let mut v = 1;
                let mut version_sum = 0;
                for subpacket in subpackets.iter() {
                    let eval = subpacket.eval();
                    v *= eval.0;
                    version_sum += eval.1
                }
                (v, version_sum)
            }
            Self::Min(subpackets) => {
                let mut v = usize::MAX;
                let mut version_sum = 0;
                for subpacket in subpackets.iter() {
                    let eval = subpacket.eval();
                    v = v.min(eval.0);
                    version_sum += eval.1
                }
                (v, version_sum)
            }
            Self::Max(subpackets) => {
                let mut v = usize::MIN;
                let mut version_sum = 0;
                for subpacket in subpackets.iter() {
                    let eval = subpacket.eval();
                    v = v.max(eval.0);
                    version_sum += eval.1
                }
                (v, version_sum)
            }
            Self::Literal(literal) => (*literal, 0),
            Self::LessThan(subpackets) => {
                let sp1 = subpackets[0].eval();
                let sp2 = subpackets[1].eval();
                (if sp1.0 < sp2.0 { 1 } else { 0 }, sp1.1 + sp2.1)
            }
            Self::GreaterThan(subpackets) => {
                let sp1 = subpackets[0].eval();
                let sp2 = subpackets[1].eval();
                (if sp1.0 > sp2.0 { 1 } else { 0 }, sp1.1 + sp2.1)
            }
            Self::EqualTo(subpackets) => {
                let sp1 = subpackets[0].eval();
                let sp2 = subpackets[1].eval();
                (if sp1.0 == sp2.0 { 1 } else { 0 }, sp1.1 + sp2.1)
            }
        }
    }
}

#[derive(Default)]
struct BitBuffer {
    input: Vec<usize>,
    bit_buffer: usize,
    bits_avail: usize,
}

impl Day16 {
    pub fn new() -> Self {
        Self {
            bit_buffer: BitBuffer::default(),
        }
    }
}

impl BitBuffer {
    fn bits(&mut self, num: usize) -> usize {
        while self.bits_avail < num {
            self.bit_buffer <<= 4;
            self.bit_buffer |= self.input.pop().unwrap();
            self.bits_avail += 4;
        }
        let bits = self.bit_buffer >> (self.bits_avail - num);
        self.bits_avail -= num;
        self.bit_buffer &= !(!0 << self.bits_avail);
        bits
    }
}

impl Runner for Day16 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        assert_eq!(lines.len(), 1);
        self.bit_buffer.input.extend(
            lines[0]
                .chars()
                .map(|c| match c {
                    '0'..='9' => (c as u8 - b'0') as usize,
                    'A'..='F' => (c as u8 - b'A' + 10) as usize,
                    _ => unreachable!(),
                })
                .rev(),
        );

        Ok(())
    }

    fn run_part(&mut self, part: u8) -> Result<RunOutput, Error> {
        match part {
            1 => self.part1(),
            2 => self.part2(),
            _ => Err(Error::Skipped),
        }
    }
}

impl Day16 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        let packet = Packet::new(&mut self.bit_buffer);
        Ok(packet.eval().1.into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let packet = Packet::new(&mut self.bit_buffer);
        Ok(packet.eval().0.into())
    }
}
