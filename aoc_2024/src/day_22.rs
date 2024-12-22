#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, IterPairs, Lines, LinesOpt};

struct Random(usize);

impl Random {
    fn next_number(&mut self) -> usize {
        self.0 ^= (self.0 << 6) & 0xff_ffff;
        self.0 ^= self.0 >> 5;
        self.0 ^= (self.0 << 11) & 0xff_ffff;

        self.0
    }
}

impl Iterator for Random {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.next_number())
    }
}

#[derive(Default)]
pub struct Day22 {
    monkeys: Vec<Random>,
}

impl Day22 {
    pub fn new() -> Self {
        Self::default()
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self
            .monkeys
            .iter_mut()
            .map(|m| m.take(2000).last().unwrap())
            .sum::<usize>()
            .into())
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        let mut diff_combos: HashMap<[i8; 4], isize> = HashMap::default();
        let mut seen = HashSet::default();
        for monkey in self.monkeys.iter_mut() {
            let mut last = (monkey.0 % 10) as i8;

            let mut deltas = [0i8; 4];
            seen.clear();
            for i in 0..2000 {
                let cur = (monkey.next_number() % 10) as i8;
                deltas.copy_within(1..4, 0);
                deltas[3] = cur - last;

                if i >= 4 && seen.insert(deltas) {
                    *diff_combos.entry(deltas).or_default() += cur as isize;
                }
                last = cur;
            }
        }

        Ok(diff_combos.values().max().copied().unwrap().into())
    }
}

impl helper::Runner for Day22 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            self.monkeys.push(Random(line.parse()?));
        }
        Ok(())
    }

    fn run_part(&mut self, part: u8) -> Result<helper::RunOutput, Error> {
        match part {
            1 => self.part1(),
            2 => self.part2(),
            _ => Err(Error::Skipped),
        }
    }
}
