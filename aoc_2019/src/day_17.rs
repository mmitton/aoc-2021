use std::collections::HashSet;
use std::{cmp::Ordering, fmt::Display};

#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};

use crate::intcode::{IntCode, State};

pub struct Day17 {
    intcode: IntCode<isize>,
}

impl Day17 {
    pub fn new() -> Self {
        Self {
            intcode: IntCode::default(),
        }
    }

    fn get_map(&self) -> (HashSet<Pos>, Pos) {
        let mut y = 0;
        let mut x = 0;
        let mut map = HashSet::new();
        let mut start = Pos(isize::MAX, isize::MAX);
        let mut intcode = self.intcode.clone();
        loop {
            match intcode.run() {
                State::HasOutput(v) => match v as u8 as char {
                    '\n' => {
                        y += 1;
                        x = 0;
                    }
                    '.' => x += 1,
                    '#' => {
                        let _ = map.insert(Pos(x, y));
                        x += 1;
                    }
                    '^' => {
                        map.insert(Pos(x, y));
                        start = Pos(x, y);
                        x += 1;
                    }
                    x => unreachable!("Unexpected map char {x:?}"),
                },
                State::Stopped => break,
                x => unreachable!("Unexpected state: {x:?}"),
            }
        }

        // let max_y = y - 1;
        // let max_x = map.iter().map(|Pos(x, _)| *x).max().unwrap();
        // println!("start: {start:?}");
        // println!("max: {max_x},{max_y}");
        // for y in 0..=max_y {
        //     for x in 0..=max_x {
        //         match map.get(&Pos(x, y)) {
        //             Some(_) => print!("#"),
        //             None => print!(" "),
        //         }
        //     }
        //     println!();
        // }

        (map, start)
    }

    fn get_commands(&self) -> Vec<Command> {
        let (map, start) = self.get_map();

        let mut dir = Dir::North;
        let mut cur = start;
        let mut commands = Vec::new();
        while let Some((command, d, c)) = dir.next_command(&map, cur) {
            commands.push(command);
            dir = d;
            cur = c;
        }

        commands
    }
}

#[derive(Copy, Clone, Debug)]
enum Dir {
    North,
    South,
    East,
    West,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Pos(isize, isize);

impl Dir {
    fn next_command(self, map: &HashSet<Pos>, cur: Pos) -> Option<(Command, Dir, Pos)> {
        let dirs = match self {
            Self::North | Self::South => [Self::East, Self::West],
            Self::East | Self::West => [Self::North, Self::South],
        };

        let mut next_dir_pos = None;
        for dir in dirs {
            let next = dir.next(cur);
            if map.contains(&next) {
                next_dir_pos = Some((dir, next));
                break;
            }
        }

        if let Some((next_dir, mut next)) = next_dir_pos {
            let turn = match (self, next_dir) {
                (Self::North, Self::West) => 'L',
                (Self::North, Self::East) => 'R',
                (Self::East, Self::North) => 'L',
                (Self::East, Self::South) => 'R',
                (Self::South, Self::East) => 'L',
                (Self::South, Self::West) => 'R',
                (Self::West, Self::South) => 'L',
                (Self::West, Self::North) => 'R',
                _ => unreachable!("Invalid turn from {self:?} to {next_dir:?}"),
            };

            let mut dist = 0;
            let mut last = next;
            while map.contains(&next) {
                dist += 1;
                last = next;
                next = next_dir.next(next);
            }

            Some((Command { turn, dist }, next_dir, last))
        } else {
            None
        }
    }

    fn next(self, pos: Pos) -> Pos {
        match self {
            Self::North => Pos(pos.0, pos.1 - 1),
            Self::South => Pos(pos.0, pos.1 + 1),
            Self::East => Pos(pos.0 + 1, pos.1),
            Self::West => Pos(pos.0 - 1, pos.1),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Command {
    turn: char,
    dist: usize,
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.dist == 0 {
            write!(f, "{}", self.turn)
        } else {
            write!(f, "{},{}", self.turn, self.dist)
        }
    }
}

impl Command {
    fn len(&self) -> usize {
        if self.dist == 0 {
            1
        } else if self.dist < 10 {
            3
        } else {
            4
        }
    }
}

impl From<char> for Command {
    fn from(value: char) -> Self {
        Self {
            turn: value,
            dist: 0,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
struct CommandSet {
    commands: Vec<Command>,
}

impl Display for CommandSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, c) in self.commands.iter().enumerate() {
            if i != 0 {
                write!(f, ",{}", c)?;
            } else {
                c.fmt(f)?;
            }
        }
        Ok(())
    }
}

impl PartialOrd for CommandSet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CommandSet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match other.len().cmp(&self.len()) {
            Ordering::Equal => self.commands.cmp(&other.commands),
            x => x,
        }
    }
}

impl CommandSet {
    fn len(&self) -> usize {
        self.commands.iter().map(|c| c.len()).sum::<usize>() + self.commands.len() - 1
    }

    fn extend(&self, command: Command) -> Option<Self> {
        let mut cs = self.clone();
        cs.commands.push(command);
        if cs.len() <= 20 {
            Some(cs)
        } else {
            None
        }
    }
}

impl TryFrom<&[Command]> for CommandSet {
    type Error = ();

    fn try_from(value: &[Command]) -> Result<Self, Self::Error> {
        let cs = CommandSet {
            commands: value.into(),
        };

        if cs.len() < 20 {
            Ok(cs)
        } else {
            Err(())
        }
    }
}

fn try_encode(
    commands: &[Command],
    a: &CommandSet,
    b: &CommandSet,
    c: &CommandSet,
) -> Option<CommandSet> {
    let mut work = Vec::new();
    work.push((commands, CommandSet::default()));

    fn strip_prefix<'a>(commands: &'a [Command], prefix: &'a [Command]) -> Option<&'a [Command]> {
        if commands.len() < prefix.len() {
            return None;
        }

        for (a, b) in commands.iter().zip(prefix.iter()) {
            if a != b {
                return None;
            }
        }

        Some(&commands[prefix.len()..])
    }

    let parts: &[(Command, &[Command])] = &[
        ('A'.into(), a.commands.as_slice()),
        ('B'.into(), b.commands.as_slice()),
        ('C'.into(), c.commands.as_slice()),
    ];

    while let Some((commands, command_set)) = work.pop() {
        for (c, prefix) in parts.iter() {
            if let Some(commands) = strip_prefix(commands, prefix) {
                if let Some(cs) = command_set.extend(*c) {
                    if commands.is_empty() {
                        return Some(cs);
                    }
                    work.push((commands, cs));
                }
            }
        }
    }

    None
}

impl Runner for Day17 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        self.intcode.load(Lines::from_bufread(file, LinesOpt::RAW)?)
    }

    fn run_part(&mut self, part: u8) -> Result<RunOutput, Error> {
        match part {
            1 => self.part1(),
            2 => self.part2(),
            _ => Err(Error::Skipped),
        }
    }
}

impl Day17 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        let (map, _) = self.get_map();
        let mut alignment = 0;
        for pos in map.iter() {
            if map.contains(&Pos(pos.0 - 1, pos.1))
                && map.contains(&Pos(pos.0 + 1, pos.1))
                && map.contains(&Pos(pos.0, pos.1 - 1))
                && map.contains(&Pos(pos.0, pos.1 + 1))
            {
                alignment += pos.0 * pos.1;
            }
        }
        Ok(alignment.into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let commands = self.get_commands();

        let mut command_sets = Vec::new();
        for i in 0..commands.len() {
            for j in i + 1..commands.len() {
                if let Ok(cs) = CommandSet::try_from(&commands[i..j]) {
                    if !command_sets.contains(&cs) {
                        command_sets.push(cs);
                    }
                }
            }
        }
        command_sets.sort();

        for (i, a) in command_sets.iter().enumerate() {
            for (j, b) in command_sets.iter().enumerate().skip(i + 1) {
                for c in command_sets.iter().skip(j) {
                    if let Some(cs) = try_encode(&commands, a, b, c) {
                        self.intcode[0] = 2;
                        self.intcode.append_ascii(&format!("{cs}\n"));
                        self.intcode.append_ascii(&format!("{a}\n"));
                        self.intcode.append_ascii(&format!("{b}\n"));
                        self.intcode.append_ascii(&format!("{c}\n"));
                        self.intcode.append_ascii("n\n");

                        let mut dust = 0;
                        loop {
                            match self.intcode.run() {
                                State::Stopped => return Ok(dust.into()),
                                State::HasOutput(v) => dust = v,
                                x => unreachable!("Unexpected state: {x:?}"),
                            }
                        }
                    }
                }
            }
        }

        Err(Error::Unsolved)
    }
}
