#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt};

struct Scanner {
    instructions: Vec<Instruction>,
}

impl Scanner {
    fn scan<F>(&self, mem: &str, mut callback: F)
    where
        F: FnMut(&str, Vec<usize>),
    {
        let chars: Vec<char> = mem.chars().collect();
        let mut pos = 0;

        while pos < chars.len() {
            'inst_search: for inst in self.instructions.iter() {
                for (i, c) in inst.name_chars.iter().enumerate() {
                    if let Some(mc) = chars.get(pos + i) {
                        if mc != c {
                            continue 'inst_search;
                        }
                    }
                }
                let mut tmp_pos = pos + inst.name_chars.len();

                // consume the '('
                if Some('(') != chars.get(tmp_pos).copied() {
                    continue 'inst_search;
                }
                tmp_pos += 1;

                // consume the args
                let mut args: Vec<usize> = Vec::new();
                let mut arg: Option<usize> = None;
                loop {
                    if let Some(c) = chars.get(tmp_pos) {
                        match *c {
                            c if c.is_ascii_digit() => {
                                let v = (c as u8 - b'0') as usize;
                                match &mut arg {
                                    Some(arg) => *arg = *arg * 10 + v,
                                    None => arg = Some(v),
                                }
                            }
                            ',' if args.len() < inst.args && arg.is_some() => {
                                args.push(arg.take().unwrap())
                            }
                            ')' if args.len() + 1 == inst.args && arg.is_some() => {
                                args.push(arg.unwrap());
                                break;
                            }
                            ')' if args.len() == inst.args && arg.is_none() => break,
                            _ => continue 'inst_search,
                        }
                    } else {
                        continue 'inst_search;
                    }
                    tmp_pos += 1;
                }

                callback(inst.name, args);
                pos = tmp_pos;
                continue 'inst_search;
            }
            pos += 1;
        }
    }
}

struct Instruction {
    name: &'static str,
    name_chars: Vec<char>,
    args: usize,
}

impl Instruction {
    fn new(name: &'static str, args: usize) -> Self {
        let name_chars = name.chars().collect();
        Self {
            name,
            name_chars,
            args,
        }
    }
}

#[derive(Default)]
pub struct Day03 {
    memory: Vec<String>,
}

impl Day03 {
    pub fn new() -> Self {
        Self::default()
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        let scanner = Scanner {
            instructions: vec![Instruction::new("mul", 2)],
        };

        let mut ans = 0;
        for line in self.memory.iter() {
            scanner.scan(line, |name, args| {
                if name == "mul" {
                    ans += args[0] * args[1];
                } else {
                    unreachable!()
                }
            })
        }
        Ok(ans.into())
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        let scanner = Scanner {
            instructions: vec![
                Instruction::new("mul", 2),
                Instruction::new("do", 0),
                Instruction::new("don't", 0),
            ],
        };

        let mut do_mul = true;
        let mut ans = 0;
        for line in self.memory.iter() {
            scanner.scan(line, |name, args| match name {
                "mul" if do_mul => ans += args[0] * args[1],
                "do" => do_mul = true,
                "don't" => do_mul = false,
                _ => {}
            })
        }

        Ok(ans.into())
    }
}

impl helper::Runner for Day03 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        for line in Lines::from_bufread(file, LinesOpt::RAW)?.iter() {
            self.memory.push(line.into());
        }
        Ok(())
    }

    fn run_part(&mut self, part: u8) -> Result<helper::RunOutput, Error> {
        match part {
            1 => self.part1(),
            2 => self.part2(),
            _ => Err(Error::Skipped),
        }
    }
}
