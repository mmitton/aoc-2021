use std::{collections::VecDeque, str::FromStr};

#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Default, Debug)]
struct Bot {
    input: Vec<usize>,
    low_to: To,
    high_to: To,
}

#[derive(Default, Debug)]
enum To {
    Bot(usize),
    Output(usize),
    #[default]
    None,
}

impl FromStr for To {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() != 2 {
            Err(Error::InvalidInput(s.into()))
        } else {
            match parts[0] {
                "bot" => Ok(Self::Bot(parts[1].parse()?)),
                "output" => Ok(Self::Output(parts[1].parse()?)),
                _ => Err(Error::InvalidInput(s.into())),
            }
        }
    }
}

#[derive(Default)]
pub struct Day10 {
    bots: Vec<Bot>,
    outputs: Vec<usize>,
}

impl Day10 {
    pub fn new() -> Self {
        Self::default()
    }

    fn ensure_to(&mut self, to: &To) {
        match to {
            To::Bot(bot) => self.ensure_bot(*bot),
            To::Output(output) => self.ensure_output(*output),
            To::None => unreachable!(),
        }
    }

    fn ensure_bot(&mut self, bot: usize) {
        while self.bots.len() < bot + 1 {
            self.bots.push(Bot::default());
        }
    }

    fn ensure_output(&mut self, output: usize) {
        while self.outputs.len() < output + 1 {
            self.outputs.push(0);
        }
    }

    fn process<F>(&mut self, abort: F) -> usize
    where
        F: Fn(usize, usize) -> bool,
    {
        let mut to_process = VecDeque::new();
        for (i, bot) in self.bots.iter().enumerate() {
            if bot.input.len() == 2 {
                to_process.push_back(i);
            }
        }

        while let Some(idx) = to_process.pop_front() {
            assert_eq!(self.bots[idx].input.len(), 2);

            let (mut a, mut b) = (self.bots[idx].input[0], self.bots[idx].input[1]);
            if a > b {
                (a, b) = (b, a);
            }
            self.bots[idx].input.clear();

            if abort(a, b) {
                return idx;
            }

            macro_rules! send {
                ($to:expr, $val:expr) => {
                    match $to {
                        To::Bot(idx) => {
                            self.bots[idx].input.push($val);
                            if self.bots[idx].input.len() == 2 {
                                to_process.push_back(idx);
                            }
                        }
                        To::Output(idx) => self.outputs[idx] = $val,
                        To::None => unreachable!(),
                    }
                };
            }

            send!(self.bots[idx].low_to, a);
            send!(self.bots[idx].high_to, b);
        }

        0
    }
}

impl Runner for Day10 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            if let Some(rest) = line.strip_prefix("value ") {
                if let Some((val, to)) = rest.split_once(" goes to bot ") {
                    let val: usize = val.parse()?;
                    let to: usize = to.parse()?;
                    self.ensure_bot(to);
                    self.bots[to].input.push(val);
                } else {
                    return Err(Error::InvalidInput(line.into()));
                }
            } else if let Some(rest) = line.strip_prefix("bot ") {
                if let Some((bot, rest)) = rest.split_once(" gives low to ") {
                    let bot: usize = bot.parse()?;
                    self.ensure_bot(bot);
                    if let Some((low, high)) = rest.split_once(" and high to ") {
                        let low: To = low.parse()?;
                        let high: To = high.parse()?;

                        self.ensure_to(&low);
                        self.ensure_to(&high);
                        self.bots[bot].low_to = low;
                        self.bots[bot].high_to = high;
                    } else {
                        return Err(Error::InvalidInput(format!("1 {line:?}")));
                    }
                } else {
                    return Err(Error::InvalidInput(format!("2 {line:?}")));
                }
            } else {
                return Err(Error::InvalidInput(format!("3 {line:?}")));
            }
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

impl Day10 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.process(|a, b| a == 17 && b == 61).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        self.process(|_, _| false);
        Ok(self.outputs[0..3].iter().product::<usize>().into())
    }
}
