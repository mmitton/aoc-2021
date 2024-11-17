use std::str::FromStr;

#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Copy, Clone, Debug)]
enum Op {
    SwapPosition(usize, usize),
    SwapLetter(char, char),
    RotateSteps(isize),
    RotateLetterPos(char),
    Reverse(usize, usize),
    Move(usize, usize),
}

impl FromStr for Op {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        let op = match parts[0] {
            "swap" => match parts[1] {
                "position" => Op::SwapPosition(parts[2].parse()?, parts[5].parse()?),
                "letter" => Op::SwapLetter(
                    parts[2].chars().nth(0).unwrap(),
                    parts[5].chars().nth(0).unwrap(),
                ),
                _ => unreachable!(),
            },
            "rotate" => match parts[1] {
                "left" => Op::RotateSteps(-parts[2].parse()?),
                "right" => Op::RotateSteps(parts[2].parse()?),
                "based" => Op::RotateLetterPos(parts[6].chars().nth(0).unwrap()),
                _ => unreachable!(),
            },
            "reverse" => Op::Reverse(parts[2].parse()?, parts[4].parse()?),
            "move" => Op::Move(parts[2].parse()?, parts[5].parse()?),
            _ => unreachable!(),
        };
        Ok(op)
    }
}

#[derive(Default)]
pub struct Day21 {
    ops: Vec<Op>,
}

impl Day21 {
    pub fn new() -> Self {
        Self::default()
    }

    fn scramble(&self, password: &str) -> String {
        let mut password: Vec<char> = password.chars().collect();

        for op in self.ops.iter().copied() {
            match op {
                Op::SwapPosition(p0, p1) => {
                    password.swap(p0, p1);
                }
                Op::SwapLetter(c0, c1) => {
                    let mut p0 = usize::MAX;
                    let mut p1 = usize::MAX;
                    for (i, c) in password.iter().copied().enumerate() {
                        if c == c0 {
                            p0 = i;
                        }
                        if c == c1 {
                            p1 = i;
                        }
                    }

                    assert!(p0 != usize::MAX);
                    assert!(p1 != usize::MAX);
                    password.swap(p0, p1);
                }
                Op::RotateSteps(steps) => {
                    if steps < 0 {
                        password.rotate_left((-(steps)) as usize);
                    } else {
                        password.rotate_right(steps as usize);
                    }
                }
                Op::RotateLetterPos(c) => {
                    for i in 0..password.len() {
                        if password[i] == c {
                            password.rotate_right(i);
                            password.rotate_right(1);
                            if i >= 4 {
                                password.rotate_right(1);
                            }
                            break;
                        }
                    }
                }
                Op::Reverse(c0, c1) => {
                    let width = c1 - c0 + 1;

                    for i in 0..width / 2 {
                        password.swap(c0 + i, c1 - i);
                    }
                }
                Op::Move(from, to) => {
                    let c = password.remove(from);
                    password.insert(to, c);
                }
            }
        }

        password.iter().collect::<String>()
    }

    fn unscramble(&self, password: &str) -> String {
        let mut password: Vec<char> = password.chars().collect();

        let mut letter_rotations = vec![usize::MAX; password.len()];
        for i in 0..letter_rotations.len() {
            let mut rotate = i + 1;
            if i >= 4 {
                rotate += 1
            };
            rotate %= password.len();
            letter_rotations[(i + rotate) % password.len()] = rotate;
        }

        for op in self.ops.iter().rev().copied() {
            match op {
                Op::SwapPosition(p0, p1) => {
                    password.swap(p0, p1);
                }
                Op::SwapLetter(c0, c1) => {
                    let mut p0 = usize::MAX;
                    let mut p1 = usize::MAX;
                    for (i, c) in password.iter().copied().enumerate() {
                        if c == c0 {
                            p0 = i;
                        }
                        if c == c1 {
                            p1 = i;
                        }
                    }

                    assert!(p0 != usize::MAX);
                    assert!(p1 != usize::MAX);
                    password.swap(p0, p1);
                }
                Op::RotateSteps(steps) => {
                    if steps < 0 {
                        password.rotate_right((-(steps)) as usize);
                    } else {
                        password.rotate_left(steps as usize);
                    }
                }
                Op::RotateLetterPos(c) => {
                    for i in 0..password.len() {
                        if password[i] == c {
                            password.rotate_left(letter_rotations[i]);
                            break;
                        }
                    }
                }
                Op::Reverse(c0, c1) => {
                    let width = c1 - c0 + 1;

                    for i in 0..width / 2 {
                        password.swap(c0 + i, c1 - i);
                    }
                }
                Op::Move(from, to) => {
                    let c = password.remove(to);
                    password.insert(from, c);
                }
            }
        }

        password.iter().collect::<String>()
    }
}

impl Runner for Day21 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            self.ops.push(line.parse()?);
        }
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

impl Day21 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        let password = if self.ops.len() == 8 {
            "abcde"
        } else {
            "abcdefgh"
        };
        Ok(self.scramble(password).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let password = if self.ops.len() == 8 {
            "decab"
        } else {
            "fbgdceah"
        };
        Ok(self.unscramble(password).into())
    }
}
