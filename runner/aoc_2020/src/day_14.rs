#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Debug)]
enum Op {
    Mask(Vec<char>),
    Mem(usize, usize),
}

#[derive(Debug)]
struct AddrMask {
    mask: Vec<char>,
}

impl AddrMask {
    fn new(addr: usize, mask: &[char]) -> Self {
        let mut addr_mask = Vec::with_capacity(36);
        for (i, mask) in mask.iter().enumerate() {
            match mask {
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
            addrs.iter_mut().for_each(|addr| *addr <<= 1);
            match self.mask[i] {
                '0' => {}
                '1' => {
                    addrs.iter_mut().for_each(|addr| *addr |= 1);
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

pub struct Day14 {
    ops: Vec<Op>,
}

impl Day14 {
    pub fn new() -> Self {
        Self { ops: Vec::new() }
    }
}

impl Runner for Day14 {
    fn parse(&mut self, path: &str, _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_path(path, LinesOpt::ALL)?;

        for line in lines.iter() {
            if let Some(line) = line.strip_prefix("mask = ") {
                let mask: Vec<char> = line.chars().collect();
                assert!(mask.len() == 36);
                self.ops.push(Op::Mask(mask));
            } else if line.starts_with("mem[") {
                let line = line.replace(']', "");
                let parts = line[4..].split(" = ").collect::<Vec<&str>>();
                let addr = parts[0].parse()?;
                let num = parts[1].parse()?;

                self.ops.push(Op::Mem(addr, num));
            } else {
                return Err(Error::InvalidInput(line.to_string()));
            }
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        let mut mem = HashMap::default();
        let mut and_mask = !0;
        let mut or_mask = 0;

        for op in self.ops.iter() {
            match op {
                Op::Mask(m) => {
                    or_mask = 0usize;
                    and_mask = 0usize;
                    for c in m.iter() {
                        or_mask <<= 1;
                        and_mask <<= 1;
                        match c {
                            'X' => {
                                or_mask |= 0b0;
                                and_mask |= 0b1;
                            }
                            '1' => {
                                or_mask |= 0b1;
                                and_mask |= 0b1;
                            }
                            '0' => {
                                or_mask |= 0b0;
                                and_mask |= 0b0;
                            }
                            _ => unreachable!(),
                        }
                    }
                }
                Op::Mem(addr, num) => {
                    mem.insert(addr, (num | or_mask) & and_mask);
                }
            }
        }

        Ok(mem.values().sum::<usize>().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut mask = vec!['X'; 36];
        let mut mem: HashMap<usize, usize> = HashMap::default();

        for op in self.ops.iter() {
            match op {
                Op::Mask(m) => {
                    mask.copy_from_slice(m);
                }
                Op::Mem(addr, num) => {
                    let addr_mask = AddrMask::new(*addr, &mask);
                    for addr in addr_mask.addresses() {
                        mem.insert(addr, *num);
                    }
                }
            }
        }

        Ok(mem.values().sum::<usize>().into())
    }
}
