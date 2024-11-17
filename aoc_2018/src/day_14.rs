#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Default)]
pub struct Day14 {
    input: usize,
}

impl Day14 {
    pub fn new() -> Self {
        Self::default()
    }

    fn forward(&self) -> usize {
        let mut scores = vec![3, 7];
        let mut elves = [0, 1];

        for _ in 0..self.input + 10 {
            let new_score = scores[elves[0]] + scores[elves[1]];
            if new_score > 9 {
                scores.push(new_score / 10);
                scores.push(new_score % 10);
            } else {
                scores.push(new_score);
            }

            for i in 0..elves.len() {
                elves[i] = (elves[i] + 1 + scores[elves[i]]) % scores.len();
            }
        }

        let mut answer = 0;
        for i in 0..10 {
            answer = (answer * 10) + scores[self.input + i];
        }

        answer
    }

    fn reverse(&self) -> usize {
        let mut scores = vec![3, 7];
        let mut elves = [0, 1];
        let mut target = Vec::new();
        let mut t = self.input;
        while t > 0 {
            target.insert(0, t % 10);
            t /= 10;
        }

        loop {
            let new_score = scores[elves[0]] + scores[elves[1]];
            if new_score > 9 {
                scores.push(new_score / 10);
                scores.push(new_score % 10);
            } else {
                scores.push(new_score);
            }

            if scores.len() > target.len() + 1 {
                if scores[scores.len() - target.len()..].eq(&target) {
                    return scores.len() - target.len();
                } else if scores[scores.len() - target.len() - 1..scores.len() - 1].eq(&target) {
                    return scores.len() - target.len() - 1;
                }
            }

            for i in 0..elves.len() {
                elves[i] = (elves[i] + 1 + scores[elves[i]]) % scores.len();
            }
        }
    }
}

impl Runner for Day14 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        assert_eq!(lines.len(), 1);
        self.input = lines[0].parse()?;
        Ok(())
    }

    fn run_part(&mut self, part: u8) -> Result<RunOutput, Error> {
        match part {
            1 => self.part1(),
            2 => self.part2(),
            _ => Err(Error::Skipped),
        }
    }
}

impl Day14 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(format!("{:010}", self.forward()).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self.reverse().into())
    }
}
