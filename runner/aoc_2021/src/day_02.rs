#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

enum Command {
    Forward(usize),
    Down(usize),
    Up(usize),
}

pub struct Day02 {
    commands: Vec<Command>,
}

impl Day02 {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }
}

impl Runner for Day02 {
    fn parse(&mut self, path: &str, _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_path(path, LinesOpt::ALL)?;
        self.commands.extend(lines.iter().map(|l| {
            let (cmd, num) = l.split_once(' ').unwrap();
            let num: usize = num.parse().unwrap();
            match cmd {
                "forward" => Command::Forward(num),
                "up" => Command::Up(num),
                "down" => Command::Down(num),
                _ => unreachable!(),
            }
        }));
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        let (mut x, mut y) = (0, 0);
        self.commands.iter().for_each(|cmd| match cmd {
            Command::Forward(n) => x += n,
            Command::Up(n) => y -= n,
            Command::Down(n) => y += n,
        });
        Ok((x * y).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let (mut x, mut y, mut aim) = (0, 0, 0);
        self.commands.iter().for_each(|cmd| match cmd {
            Command::Forward(n) => {
                x += n;
                y += aim * n;
            }
            Command::Up(n) => aim -= n,
            Command::Down(n) => aim += n,
        });
        Ok((x * y).into())
    }
}
