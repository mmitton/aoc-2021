#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Debug, Eq, PartialEq)]
enum Packet {
    Num(u32),
    List(Vec<Packet>),
}

impl Packet {
    fn new(line: &str) -> Packet {
        fn decode(chars: &[char], s: usize) -> (Packet, usize) {
            match chars[s] {
                '[' => {
                    if chars[s + 1] == ']' {
                        return (Packet::List(Vec::new()), s + 2);
                    }
                    let mut list = Vec::new();
                    let mut p = s + 1;
                    while p < chars.len() {
                        let (elem, e) = decode(chars, p);
                        list.push(elem);
                        p = e;
                        match chars[p] {
                            ',' => p += 1,
                            ']' => return (Packet::List(list), p + 1),
                            _ => unreachable!("{}", chars[p]),
                        }
                    }
                    unreachable!();
                }
                '0'..='9' => {
                    let mut num: u32 = chars[s] as u32 - '0' as u32;
                    let mut s = s + 1;
                    while chars[s] >= '0' && chars[s] <= '9' {
                        num *= 10;
                        num += chars[s] as u32 - '0' as u32;
                        s += 1;
                    }
                    (Packet::Num(num), s)
                }
                _ => unreachable!(),
            }
        }

        let chars: Vec<char> = line.chars().collect();
        let (packet, end) = decode(&chars, 0);
        if end != chars.len() {
            panic!("Did not decode entire packet");
        }
        packet
    }

    fn ordered(&self, rhs: &Packet) -> Option<bool> {
        match (self, rhs) {
            (Self::Num(lhs), Self::Num(rhs)) => {
                if lhs == rhs {
                    None
                } else {
                    Some(lhs < rhs)
                }
            }
            (Self::Num(_), Self::List(_)) => {
                let lhs = Packet::List(vec![self.clone()]);
                lhs.ordered(rhs)
            }
            (Self::List(_), Self::Num(_)) => {
                let rhs = Packet::List(vec![rhs.clone()]);
                self.ordered(&rhs)
            }
            (Self::List(lhs), Self::List(rhs)) => {
                for (lhs, rhs) in lhs.iter().zip(rhs.iter()) {
                    match lhs.ordered(rhs) {
                        None => {}
                        Some(o) => return Some(o),
                    }
                }

                match lhs.len().cmp(&rhs.len()) {
                    Ordering::Equal => None,
                    Ordering::Less => Some(true),
                    Ordering::Greater => Some(false),
                }
            }
        }
    }
}

fn main() {
    let lines: Vec<String> = {
        let file = File::open(INPUT_FILE).expect("Cannot open input file");
        BufReader::new(file).lines().flatten().collect()
    };

    let d2 = Packet::new("[[2]]");
    let d6 = Packet::new("[[6]]");
    let mut packets = Vec::new();
    packets.push(d2.clone());
    packets.push(d6.clone());
    for line in lines {
        if line.is_empty() {
            continue;
        }
        packets.push(Packet::new(&line));
    }

    packets.sort_by(|a, b| match a.ordered(b) {
        None => unreachable!(),
        Some(true) => Ordering::Less,
        Some(false) => Ordering::Greater,
    });

    let mut d2_idx = 0;
    let mut d6_idx = 0;
    for (idx, packet) in packets.iter().enumerate() {
        if packet == &d2 {
            d2_idx = idx + 1;
        }
        if packet == &d6 {
            d6_idx = idx + 1;
        }
        println!("{} {packet:?}", idx + 1);
    }

    println!("{d2_idx} * {d6_idx} = {}", d2_idx * d6_idx);
}
