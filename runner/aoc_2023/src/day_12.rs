#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug)]
pub enum RunnerError {}

impl From<RunnerError> for Error {
    fn from(e: RunnerError) -> Self {
        Self::Runner(format!("{e:?}"))
    }
}

struct Springs {
    map: Vec<State>,
    nums: Vec<usize>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum State {
    Operational,
    Damaged,
    Unknown,
}

impl State {
    fn matches(self, other: State) -> bool {
        self == other || self == State::Unknown
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Operational => '.',
                Self::Damaged => '#',
                Self::Unknown => '?',
            }
        )
    }
}

impl From<char> for State {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Display for Springs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for state in self.map.iter() {
            write!(f, "{state}")?;
        }

        write!(f, " ")?;
        for num in self.nums.iter() {
            write!(f, "{num},")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Search {
    state: State,
    min: usize,
    max: usize,
    ok_at: BTreeSet<(usize, usize)>,
    num_at: BTreeMap<usize, usize>,
}

impl Springs {
    fn num_arrangements(&self) -> usize {
        let min_len = self.nums.iter().map(|n| n + 1).sum::<usize>() - 1;
        let extra_spaces = self.map.len() - min_len;

        let mut search_space = Vec::new();
        for (i, damaged) in self.nums.iter().enumerate() {
            search_space.push(Search {
                state: State::Operational,
                min: if i == 0 { 0 } else { 1 },
                max: if i == 0 { 0 } else { 1 } + extra_spaces,
                ok_at: BTreeSet::new(),
                num_at: BTreeMap::new(),
            });
            search_space.push(Search {
                state: State::Damaged,
                min: *damaged,
                max: *damaged,
                ok_at: BTreeSet::new(),
                num_at: BTreeMap::new(),
            });
        }
        search_space.push(Search {
            state: State::Operational,
            min: 0,
            max: extra_spaces,
            ok_at: BTreeSet::new(),
            num_at: BTreeMap::new(),
        });

        let possible: [Vec<usize>; 2] = [
            self.map
                .iter()
                .map(|s| if s.matches(State::Operational) { 1 } else { 0 })
                .collect(),
            self.map
                .iter()
                .map(|s| if s.matches(State::Damaged) { 1 } else { 0 })
                .collect(),
        ];

        let mut min_start = 0;
        for i in 0..search_space.len() {
            let min_after: usize = search_space.iter().skip(i + 1).map(|s| s.min).sum();
            for start in min_start..=self.map.len() - min_after {
                for len in search_space[i].min..=search_space[i].max {
                    if start + len > self.map.len() {
                        break;
                    }
                    let possible = if search_space[i].state == State::Operational {
                        &possible[0]
                    } else {
                        &possible[1]
                    };
                    if possible[start..start + len].iter().sum::<usize>() == len {
                        search_space[i].ok_at.insert((start, len));
                    }
                }
            }
            min_start += search_space[i].min;
        }

        fn recurse(spaces: &mut [Search], pos: usize, left: usize) -> usize {
            if spaces.is_empty() {
                return if left == 0 { 1 } else { 0 };
            }
            if let Some(found) = spaces[0].num_at.get(&pos) {
                return *found;
            }

            let mut found = 0;

            let lengths = spaces[0]
                .ok_at
                .iter()
                .filter_map(|(p, len)| {
                    if *p == pos && *len <= left {
                        Some(*len)
                    } else {
                        None
                    }
                })
                .collect::<Vec<usize>>();
            for len in lengths {
                found += recurse(&mut spaces[1..], pos + len, left - len);
            }

            spaces[0].num_at.insert(pos, found);

            found
        }

        // println!("{:?}", search_space[0]);

        let arrangements = recurse(&mut search_space, 0, self.map.len());
        println!("{self} => {arrangements}");
        arrangements
    }
}

pub struct Day12 {
    springs: Vec<Springs>,
}

impl Day12 {
    pub fn new() -> Self {
        Self {
            springs: Vec::new(),
        }
    }
}

impl Runner for Day12 {
    fn parse(&mut self, path: &str, _part1: bool) -> Result<(), Error> {
        for line in Lines::from_path(path, LinesOpt::RAW)?.iter() {
            let (map, nums) = line.split_once(' ').unwrap();
            let map: Vec<State> = map.chars().map(|c| c.into()).collect();
            let nums: Vec<usize> = nums.split(',').map(|n| n.parse().unwrap()).collect();

            self.springs.push(Springs { map, nums });
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self
            .springs
            .iter()
            .map(|springs| springs.num_arrangements())
            .sum::<usize>()
            .into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        self.springs.iter_mut().for_each(|s| {
            let map = s.map.clone();
            let nums = s.nums.clone();
            for _ in 0..4 {
                s.map.push(State::Unknown);
                s.map.extend(map.iter());
                s.nums.extend(nums.iter());
            }
        });

        Ok(self
            .springs
            .iter()
            .map(|springs| springs.num_arrangements())
            .sum::<usize>()
            .into())
    }
}
