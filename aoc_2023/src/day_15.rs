#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};

pub struct Day15 {
    steps: Vec<Vec<char>>,
}

impl Day15 {
    pub fn new() -> Self {
        Self { steps: Vec::new() }
    }

    fn hash(chars: &[char]) -> u8 {
        chars
            .iter()
            .fold(0, |hash, &c| hash.wrapping_add(c as u8).wrapping_mul(17))
    }

    fn extract(step: &[char]) -> (usize, String, char, usize) {
        let op_at = step.iter().position(|c| matches!(c, '-' | '=')).unwrap();
        let hash = Self::hash(&step[..op_at]);
        let op = step[op_at];

        let num = if op == '=' {
            // num will be 1-9
            step[op_at + 1] as usize - b'0' as usize
        } else {
            0
        };

        (hash as usize, step[..op_at].iter().collect(), op, num)
    }

    fn hash_sum(&self) -> usize {
        self.steps
            .iter()
            .map(|step| Self::hash(step) as usize)
            .sum::<usize>()
    }

    fn focusing_power(&self) -> usize {
        let mut hashmap: [Vec<_>; 256] = std::array::from_fn(|_| Vec::new());
        for step in self.steps.iter() {
            let (hash, label, op, num) = Self::extract(step);
            match op {
                '-' => {
                    // Remove label from box
                    hashmap[hash].retain(|(l, _)| l != &label);
                }
                '=' => {
                    // Add/Set label in box
                    if let Some((_, n)) = hashmap[hash].iter_mut().find(|(l, _)| l == &label) {
                        *n = num;
                    } else {
                        hashmap[hash].push((label, num));
                    }
                }
                _ => unreachable!(),
            }
        }

        hashmap
            .iter()
            .enumerate()
            .map(|(bn, b)| {
                b.iter()
                    .enumerate()
                    .map(|(sn, (_, s))| (bn + 1) * (sn + 1) * *s)
                    .sum::<usize>()
            })
            .sum::<usize>()
    }
}

impl Runner for Day15 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        let lines: Vec<&str> = lines.iter().collect();
        for step in lines[0].split(',') {
            self.steps.push(step.chars().collect());
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.hash_sum().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self.focusing_power().into())
    }
}
