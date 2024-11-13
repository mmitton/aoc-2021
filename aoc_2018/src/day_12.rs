#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Default)]
pub struct Day12 {
    initial_state: Vec<isize>,
    on_states: Vec<u8>,
}

impl Day12 {
    pub fn new() -> Self {
        Self::default()
    }

    fn next_gen(&self, states: &mut [Vec<isize>], cur: usize) {
        let next = 1 - cur;

        macro_rules! get {
            ($i:expr) => {
                if states[cur].contains(&$i) {
                    1
                } else {
                    0
                }
            };
        }

        let min_x = states[cur][0] - 2;
        let max_x = states[cur][states[cur].len() - 1] + 2;

        states[next].clear();

        for i in min_x..=max_x {
            let mut c = 0u8;

            c = c << 1 | get!(i - 2);
            c = c << 1 | get!(i - 1);
            c = c << 1 | get!(i);
            c = c << 1 | get!(i + 1);
            c = c << 1 | get!(i + 2);

            if self.on_states.contains(&c) {
                states[next].push(i);
            }
        }
    }

    fn last(sums: &[isize], window: usize) -> Option<isize> {
        let delta = sums[sums.len() - 1] - sums[sums.len() - 2];
        for i in (1..window).rev() {
            if sums[sums.len() - i] - sums[sums.len() - i - 1] != delta {
                return None;
            }
        }
        Some(delta)
    }

    fn iterate(&self, generations: usize) -> isize {
        let mut states = vec![self.initial_state.clone(), Vec::new()];
        let mut sums = Vec::new();
        let mut cur = 0;
        const WINDOW: usize = 50;
        // println!("0: {}", format_full(&states[cur]));
        for gen in 1..=generations {
            self.next_gen(&mut states, cur);
            cur = 1 - cur;

            sums.push(states[cur].iter().sum());
            if sums.len() > WINDOW {
                if let Some(delta) = Self::last(&sums, WINDOW) {
                    println!(
                        "Going up by {} every generation after {} generations",
                        delta, gen
                    );
                    return states[cur].iter().sum::<isize>()
                        + ((generations - gen) as isize * delta);
                }
            }
        }

        states[cur].iter().sum()
    }
}

impl Runner for Day12 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        fn parse(s: &str) -> Vec<bool> {
            let mut v = Vec::new();
            for c in s.chars() {
                let b = match c {
                    '.' => false,
                    '#' => true,
                    _ => unreachable!(),
                };
                v.push(b);
            }
            v
        }

        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            if let Some(line) = line.strip_prefix("initial state: ") {
                self.initial_state.clear();
                for (i, b) in parse(line).iter().enumerate() {
                    if *b {
                        self.initial_state.push(i as isize)
                    }
                }
            } else if line.ends_with("#") {
                let mut n = 0u8;
                for b in parse(&line[..5]).iter() {
                    n <<= 1;
                    if *b {
                        n |= 1;
                    }
                }
                self.on_states.push(n);
            }
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.iterate(20).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self.iterate(50000000000).into())
    }
}
