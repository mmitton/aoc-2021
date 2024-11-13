#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

pub struct Day14 {
    pairs: HashMap<(char, char), usize>,
    gen: HashMap<(char, char), char>,
    counts: HashMap<char, usize>,
}

impl Day14 {
    pub fn new() -> Self {
        Self {
            pairs: HashMap::default(),
            gen: HashMap::default(),
            counts: HashMap::default(),
        }
    }

    fn generate(&mut self, iters: usize) {
        let mut pairs = HashMap::default();
        for _ in 0..iters {
            pairs.clear();
            for (pair, cnt) in self.pairs.iter() {
                if let Some(c) = self.gen.get(pair) {
                    *pairs.entry((pair.0, *c)).or_default() += cnt;
                    *pairs.entry((*c, pair.1)).or_default() += cnt;
                    *self.counts.entry(*c).or_default() += cnt;
                } else {
                    *pairs.entry(*pair).or_default() += cnt;
                }
            }
            std::mem::swap(&mut pairs, &mut self.pairs);
        }
    }
}

impl Runner for Day14 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        let chars: Vec<char> = lines[0].chars().collect();
        for (i, c) in chars.windows(2).enumerate() {
            if i == 0 {
                self.counts.insert(c[0], 1);
            }
            *self.counts.entry(c[1]).or_default() += 1;
            *self.pairs.entry((c[0], c[1])).or_default() += 1;
        }

        for line in lines[2..].iter() {
            if let Some((from, to)) = line.split_once(" -> ") {
                let mut chars = from.chars();
                let a = chars.next().expect("Invalid rule");
                let b = chars.next().expect("Invalid rule");
                self.gen
                    .insert((a, b), to.chars().next().expect("Invalid rule"));
            }
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        self.generate(10);
        let (mut min, mut max) = (usize::MAX, usize::MIN);
        for c in self.counts.values().copied() {
            min = min.min(c);
            max = max.max(c);
        }
        Ok((max - min).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        self.generate(40);
        let (mut min, mut max) = (usize::MAX, usize::MIN);
        for c in self.counts.values().copied() {
            min = min.min(c);
            max = max.max(c);
        }
        Ok((max - min).into())
    }
}
