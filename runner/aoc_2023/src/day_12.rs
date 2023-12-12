#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};
use std::collections::BTreeMap;

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

        write!(f, " {:?}", self.nums)
    }
}

#[derive(Debug)]
struct Search {
    state: State,
    min: usize,
    max: usize,
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
                num_at: BTreeMap::new(),
            });
            search_space.push(Search {
                state: State::Damaged,
                min: *damaged,
                max: *damaged,
                num_at: BTreeMap::new(),
            });
        }
        search_space.push(Search {
            state: State::Operational,
            min: 0,
            max: extra_spaces,
            num_at: BTreeMap::new(),
        });

        fn recurse(
            spaces: &mut [Search],
            pos: usize,
            left: usize,
            possible: &[Vec<usize>; 2],
        ) -> usize {
            if spaces.is_empty() {
                return if left == 0 { 1 } else { 0 };
            }
            if let Some(found) = spaces[0].num_at.get(&pos) {
                return *found;
            }

            let (space, remaining_spaces) = spaces.split_at_mut(1);
            let space = &mut space[0];

            let state_possible = if space.state == State::Operational {
                &possible[0]
            } else {
                &possible[1]
            };
            let min_after = remaining_spaces.iter().map(|s| s.min).sum::<usize>();
            let mut found = 0;
            for len in space.min..=space.max {
                if len > left - min_after {
                    break;
                }
                if state_possible[pos..pos + len].iter().sum::<usize>() == len {
                    found += recurse(remaining_spaces, pos + len, left - len, possible);
                }
            }

            space.num_at.insert(pos, found);

            found
        }

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

        let arrangements = recurse(&mut search_space, 0, self.map.len(), &possible);
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
