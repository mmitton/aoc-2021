#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

pub struct Day09 {
    nums: Vec<usize>,
}

impl Day09 {
    pub fn new() -> Self {
        Self { nums: Vec::new() }
    }

    fn weakness(&self) -> (usize, usize) {
        let preamble = if self.nums.len() < 500 { 5 } else { 25 };
        'search_loop: for (i, a) in self.nums.iter().enumerate().skip(preamble) {
            for (j, b) in self.nums[..i].iter().enumerate().skip(i - preamble) {
                for c in self.nums[j + 1..i].iter() {
                    if b + c == *a {
                        continue 'search_loop;
                    }
                }
            }

            return (i, *a);
        }

        unreachable!();
    }
}

impl Runner for Day09 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        self.nums
            .extend(lines.iter().map(|l| l.parse::<usize>().unwrap()));
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.weakness().1.into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let (i, num) = self.weakness();

        'inner_search: for j in 0..i {
            let mut sum = self.nums[j];
            for k in j + 1..i {
                sum += self.nums[k];
                if sum > num {
                    continue 'inner_search;
                }
                if sum == num {
                    let sum_numbers = &mut self.nums[j..k + 1];
                    sum_numbers.sort();
                    return Ok((sum_numbers[0] + sum_numbers[sum_numbers.len() - 1]).into());
                }
            }
        }
        Err(Error::Unsolved)
    }
}
