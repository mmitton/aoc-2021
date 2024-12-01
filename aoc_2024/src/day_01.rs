#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt};

#[derive(Default)]
pub struct Day01 {
    lists: Vec<Vec<isize>>,
}

impl Day01 {
    pub fn new() -> Self {
        Self::default()
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        self.lists[0].sort();
        self.lists[1].sort();
        Ok(self.lists[0]
            .iter()
            .zip(self.lists[1].iter())
            .map(|(a, b)| (a - b).abs())
            .sum::<isize>()
            .into())
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        let mut counts: HashMap<isize, isize> = HashMap::default();
        for n in self.lists[1].iter().copied() {
            *counts.entry(n).or_default() += 1;
        }
        Ok(self.lists[0]
            .iter()
            .copied()
            .map(|n| n * counts.get(&n).copied().unwrap_or(0))
            .sum::<isize>()
            .into())
    }
}

impl helper::Runner for Day01 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        self.lists.push(Vec::new());
        self.lists.push(Vec::new());
        for line in lines.iter() {
            let nums: Vec<&str> = line.split_whitespace().collect();
            if nums.len() == 2 {
                self.lists[0].push(nums[0].parse()?);
                self.lists[1].push(nums[1].parse()?);
            } else {
                return Err(Error::InvalidInput(line.into()));
            }
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
