#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt};

#[derive(Default)]
pub struct Day11 {
    stones: HashMap<usize, usize>,
}

impl Day11 {
    pub fn new() -> Self {
        Self::default()
    }

    fn process_rules(&mut self, cnt: usize) -> usize {
        self.stones.reserve(5000);
        let mut next_stones: HashMap<usize, usize> = HashMap::default();
        next_stones.reserve(5000);
        for _ in 0..cnt {
            for (n, c) in self.stones.drain() {
                match n {
                    0 => *next_stones.entry(1).or_default() += c,
                    n if (n.ilog10() + 1) % 2 == 0 => {
                        let digits = n.ilog10() + 1;
                        let magnatude = 10usize.pow(digits / 2);
                        let left = n / magnatude;
                        let right = n % magnatude;
                        *next_stones.entry(left).or_default() += c;
                        *next_stones.entry(right).or_default() += c;
                    }
                    _ => *next_stones.entry(n * 2024).or_default() += c,
                }
            }
            std::mem::swap(&mut self.stones, &mut next_stones);
        }

        self.stones.iter().fold(0, |count, (_, c)| count + c)
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self.process_rules(25).into())
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self.process_rules(75).into())
    }
}

impl helper::Runner for Day11 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        for n in Lines::from_bufread(file, LinesOpt::RAW)?
            .single_line()?
            .split_whitespace()
        {
            *self.stones.entry(n.parse()?).or_default() += 1;
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
