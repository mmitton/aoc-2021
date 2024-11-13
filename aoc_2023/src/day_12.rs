#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};

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
    Working,
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
                Self::Working => '.',
                Self::Damaged => '#',
                Self::Unknown => '?',
            }
        )
    }
}

impl From<char> for State {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Working,
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
    min_after: usize,
    num_at: Vec<usize>,
    masks: Vec<u128>,
}

impl Springs {
    fn num_arrangements(&self) -> usize {
        let min_len = self.nums.iter().map(|n| n + 1).sum::<usize>() - 1;
        let extra_spaces = self.map.len() - min_len;

        let mut search_space = Vec::new();
        for (i, damaged) in self.nums.iter().enumerate() {
            search_space.push(Search {
                state: State::Working,
                min: if i == 0 { 0 } else { 1 },
                max: if i == 0 { 0 } else { 1 } + extra_spaces,
                min_after: 0,
                num_at: vec![usize::MAX; self.map.len() + 1],
                masks: Vec::new(),
            });
            search_space.push(Search {
                state: State::Damaged,
                min: *damaged,
                max: *damaged,
                min_after: 0,
                num_at: vec![usize::MAX; self.map.len() + 1],
                masks: Vec::new(),
            });
        }
        search_space.push(Search {
            state: State::Working,
            min: 0,
            max: extra_spaces,
            min_after: 0,
            num_at: vec![usize::MAX; self.map.len() + 1],
            masks: Vec::new(),
        });

        for i in 0..search_space.len() {
            let min_after = search_space
                .iter()
                .skip(i + 1)
                .map(|s| s.min)
                .sum::<usize>();
            search_space[i].min_after = min_after;
            for len in search_space[i].min..=search_space[i].max {
                search_space[i].masks.push(!(u128::MAX << len));
            }
        }

        fn recurse(
            spaces: &mut [Search],
            pos: usize,
            left: usize,
            working_mask: u128,
            damaged_mask: u128,
        ) -> usize {
            if spaces.is_empty() {
                return if left == 0 { 1 } else { 0 };
            }
            let (space, remaining_spaces) = spaces.split_first_mut().unwrap();
            if space.num_at[pos] != usize::MAX {
                return space.num_at[pos];
            }

            let test_mask = if space.state == State::Working {
                working_mask >> pos
            } else {
                damaged_mask >> pos
            };

            let mut found = 0;
            for (idx, mask) in space.masks.iter().enumerate() {
                let len = space.min + idx;
                if len > left - space.min_after {
                    break;
                }
                if mask & test_mask == *mask {
                    found += recurse(
                        remaining_spaces,
                        pos + len,
                        left - len,
                        working_mask,
                        damaged_mask,
                    );
                }
            }

            space.num_at[pos] = found;

            found
        }

        let working_mask = self.map.iter().enumerate().fold(0, |acc, (i, s)| {
            acc | if s.matches(State::Working) { 1 << i } else { 0 }
        });
        let damaged_mask = self.map.iter().enumerate().fold(0, |acc, (i, s)| {
            acc | if s.matches(State::Damaged) { 1 << i } else { 0 }
        });

        let arrangements = recurse(
            &mut search_space,
            0,
            self.map.len(),
            working_mask,
            damaged_mask,
        );
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
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        for line in Lines::from_bufread(file, LinesOpt::RAW)?.iter() {
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
