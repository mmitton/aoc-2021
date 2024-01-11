use crate::intcode::{IntCode, State};
#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

pub struct Day15 {
    intcode: IntCode<isize>,
    map: BTreeMap<(isize, isize), u8>,
    oxygen: (isize, isize),
}

#[derive(Copy, Clone, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

const DIRS: [Dir; 4] = [Dir::Up, Dir::Down, Dir::Left, Dir::Right];

impl Dir {
    fn next_pos(&self, pos: (isize, isize)) -> (isize, isize) {
        match self {
            Self::Up => (pos.0, pos.1 - 1),
            Self::Down => (pos.0, pos.1 + 1),
            Self::Left => (pos.0 - 1, pos.1),
            Self::Right => (pos.0 + 1, pos.1),
        }
    }
}

impl Day15 {
    pub fn new() -> Self {
        Self {
            intcode: IntCode::default(),
            map: BTreeMap::new(),
            oxygen: (0, 0),
        }
    }

    pub fn map(&mut self) {
        let mut pos = (0, 0);
        let mut missing = BTreeSet::new();
        missing.insert((0, -1));
        missing.insert((0, 1));
        missing.insert((-1, 0));
        missing.insert((1, 0));
        self.map.insert((0, 0), 1);
        let mut path: VecDeque<Dir> = VecDeque::new();
        loop {
            match self.intcode.run() {
                State::HasOutput(v) => {
                    let next_pos = path.pop_front().unwrap().next_pos(pos);
                    missing.remove(&next_pos);
                    if self.map.insert(next_pos, v as u8).is_none() && v != 0 {
                        for d in DIRS.iter() {
                            let pos = d.next_pos(next_pos);
                            if !self.map.contains_key(&pos) {
                                missing.insert(pos);
                            }
                        }
                    }
                    if v == 2 {
                        self.oxygen = next_pos;
                    }
                    if v != 0 {
                        pos = next_pos;
                    } else {
                        assert!(path.is_empty());
                    }
                }
                State::WaitingForInput(..) => {
                    path = self.find_shortest_path(pos, &missing).into();
                    if path.is_empty() {
                        break;
                    }
                    self.intcode
                        .input
                        .extend(path.iter().map(|d| *d as isize + 1));
                }
                State::Stopped => break,
                x => unreachable!("Unexpected state: {x:?}"),
            }
        }
    }

    fn find_shortest_path(&self, from: (isize, isize), to: &BTreeSet<(isize, isize)>) -> Vec<Dir> {
        let mut seen = BTreeSet::new();
        seen.insert(from);
        let mut work: VecDeque<((isize, isize), Vec<Dir>)> = VecDeque::new();
        work.push_front((from, Vec::new()));
        while let Some((pos, path)) = work.pop_front() {
            for d in DIRS.iter() {
                let pos = d.next_pos(pos);
                if seen.insert(pos) {
                    let mut path = path.clone();
                    path.push(*d);
                    if to.contains(&pos) {
                        return path;
                    }
                    match self.map.get(&pos) {
                        Some(1) | Some(2) => work.push_back((pos, path)),
                        _ => {}
                    }
                }
            }
        }
        Vec::new()
    }

    fn find_longest_path(&self, from: (isize, isize)) -> Vec<Dir> {
        let mut seen = BTreeSet::new();
        seen.insert(from);
        let mut work: VecDeque<((isize, isize), Vec<Dir>)> = VecDeque::new();
        work.push_front((from, Vec::new()));
        while let Some((pos, path)) = work.pop_front() {
            for d in DIRS.iter() {
                let pos = d.next_pos(pos);
                if seen.insert(pos) {
                    let mut path = path.clone();
                    path.push(*d);
                    match self.map.get(&pos) {
                        Some(1) | Some(2) => work.push_back((pos, path)),
                        _ => {}
                    }
                }
            }
            if work.is_empty() {
                return path;
            }
        }
        Vec::new()
    }

    fn _print(&self) {
        let mut min = (isize::MAX, isize::MAX);
        let mut max = (isize::MIN, isize::MIN);
        for pos in self.map.keys() {
            min.0 = min.0.min(pos.0);
            min.1 = min.1.min(pos.1);
            max.0 = max.0.max(pos.0);
            max.1 = max.1.max(pos.1);
        }

        for y in min.1..=max.1 {
            for x in min.0..=max.0 {
                if x == 0 && y == 0 {
                    print!("S");
                } else {
                    match self.map.get(&(x, y)) {
                        Some(0) => print!("#"),
                        Some(1) => print!("."),
                        Some(2) => print!("O"),
                        _ => print!(" "),
                    }
                }
            }
            println!();
        }
    }
}

impl Runner for Day15 {
    fn parse(&mut self, path: &str, _part1: bool) -> Result<(), Error> {
        self.intcode.load(Lines::from_path(path, LinesOpt::RAW)?)
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        self.map();
        let path = self.find_shortest_path((0, 0), &[self.oxygen].iter().copied().collect());
        Ok(path.len().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        self.map();
        Ok(self.find_longest_path(self.oxygen).len().into())
    }
}
