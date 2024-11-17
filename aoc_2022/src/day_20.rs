#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};

pub struct Day20 {
    nums: Vec<isize>,
    mix_count: u8,
    mixed: Vec<usize>,
}

impl Day20 {
    pub fn new() -> Self {
        Self {
            nums: Vec::new(),
            mix_count: 0,
            mixed: Vec::new(),
        }
    }

    fn mix(&mut self) {
        for i in 0..self.nums.len() {
            let pos = self.mixed.iter().position(|n| *n == i).unwrap();
            let num = self.nums[i];
            // self.mixed.remove(pos);

            let new_pos = (pos as isize + num).rem_euclid(self.mixed.len() as isize - 1) as usize;

            if new_pos != pos {
                if pos < new_pos {
                    self.mixed.copy_within(pos + 1..=new_pos, pos);
                } else {
                    self.mixed.copy_within(new_pos..pos, new_pos + 1);
                }
                self.mixed[new_pos] = i;
            }

            // self.nums.insert(new_pos, num);
            // self.mixed.insert(new_pos, i);
        }
        self.mix_count += 1;
        // println!("After mix {} => {:?}", self.mix_count, self.nums);
    }

    fn _print(&self) {
        println!("After {} mixes", self.mix_count);
        for i in self.mixed.iter() {
            print!("{}, ", self.nums[*i]);
        }
        println!();
    }

    fn coords(&self) -> isize {
        let zero_at = self.mixed.iter().position(|v| self.nums[*v] == 0).unwrap();
        let x = self.mixed[(zero_at + 1000) % self.mixed.len()];
        let y = self.mixed[(zero_at + 2000) % self.mixed.len()];
        let z = self.mixed[(zero_at + 3000) % self.mixed.len()];

        self.nums[x] + self.nums[y] + self.nums[z]
    }
}

impl Runner for Day20 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        self.nums
            .extend(lines.iter().map(|l| l.parse::<isize>().unwrap()));
        self.mixed.extend(0..self.nums.len());
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

impl Day20 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        self.mix();
        Ok(self.coords().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        self.nums.iter_mut().for_each(|v| *v *= 811589153);

        for _ in 0..10 {
            self.mix();
        }
        Ok(self.coords().into())
    }
}
