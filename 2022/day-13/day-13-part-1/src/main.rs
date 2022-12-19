#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Debug)]
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

        println!("{line}");
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

    let mut ans = 0;
    for i in (0..lines.len()).step_by(3) {
        let idx = (i / 3) + 1;
        let p1 = Packet::new(&lines[i]);
        let p2 = Packet::new(&lines[i + 1]);

        let ordered = match p1.ordered(&p2) {
            Some(true) => true,
            Some(false) => false,
            None => unimplemented!(),
        };

        println!("Pair {idx} : {ordered}");
        if ordered {
            ans += idx;
        }
    }

    println!("ans: {ans}");
}
