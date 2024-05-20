#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};
use std::cmp::Ordering;

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
}

impl std::cmp::Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl std::cmp::PartialOrd for Packet {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        match (self, rhs) {
            (Self::Num(lhs), Self::Num(rhs)) => {
                if lhs == rhs {
                    None
                } else {
                    lhs.partial_cmp(rhs)
                }
            }
            (Self::Num(_), Self::List(_)) => {
                let lhs = Packet::List(vec![self.clone()]);
                lhs.partial_cmp(rhs)
            }
            (Self::List(_), Self::Num(_)) => {
                let rhs = Packet::List(vec![rhs.clone()]);
                self.partial_cmp(&rhs)
            }
            (Self::List(lhs), Self::List(rhs)) => {
                for (lhs, rhs) in lhs.iter().zip(rhs.iter()) {
                    match lhs.partial_cmp(rhs) {
                        None => {}
                        Some(o) => return Some(o),
                    }
                }

                match lhs.len().cmp(&rhs.len()) {
                    Ordering::Equal => None,
                    Ordering::Less => Some(Ordering::Less),
                    Ordering::Greater => Some(Ordering::Greater),
                }
            }
        }
    }
}

pub struct Day13 {
    packets: Vec<Packet>,
}

impl Day13 {
    pub fn new() -> Self {
        Self {
            packets: Vec::new(),
        }
    }
}

impl Runner for Day13 {
    fn parse(&mut self, path: &str, _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_path(path, LinesOpt::REMOVE_EMPTY)?;
        self.packets.extend(lines.iter().map(Packet::new));
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self
            .packets
            .chunks(2)
            .enumerate()
            .fold(0, |acc, (idx, packets)| {
                if packets[0].partial_cmp(&packets[1]) == Some(Ordering::Less) {
                    acc + idx + 1
                } else {
                    acc
                }
            })
            .into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let d2 = Packet::new("[[2]]");
        let d6 = Packet::new("[[6]]");
        self.packets.push(d2.clone());
        self.packets.push(d6.clone());
        self.packets.sort();

        let d2_idx = self
            .packets
            .iter()
            .position(|packet| packet == &d2)
            .unwrap()
            + 1;
        let d6_idx = self
            .packets
            .iter()
            .position(|packet| packet == &d6)
            .unwrap()
            + 1;
        println!("d2_idx:{d2_idx}  d6_idx:{d6_idx}");
        Ok((d2_idx * d6_idx).into())
    }
}
