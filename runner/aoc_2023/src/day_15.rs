#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Debug)]
pub enum RunnerError {}

impl From<RunnerError> for Error {
    fn from(e: RunnerError) -> Self {
        Self::Runner(format!("{e:?}"))
    }
}

pub struct Day15 {
    steps: Vec<Vec<char>>,
}

impl Day15 {
    pub fn new() -> Self {
        Self { steps: Vec::new() }
    }

    fn hash(chars: &[char]) -> u8 {
        let mut hash = 0u8;
        for &c in chars.iter() {
            hash = hash.wrapping_add(c as u8);
            hash = hash.wrapping_mul(17);
        }
        hash
    }

    fn extract(step: &[char]) -> (u8, String, char, usize) {
        let op_at = step.iter().position(|c| matches!(c, '-' | '=')).unwrap();
        let hash = Self::hash(&step[..op_at]);
        let op = step[op_at];

        let num = if op == '=' {
            step.iter()
                .skip(op_at + 1)
                .fold(0, |num, &c| num * 10 + (c as usize - '0' as usize))
        } else {
            0
        };

        (hash, step[..op_at].iter().collect(), op, num)
    }

    fn hash_sum(&self) -> usize {
        self.steps
            .iter()
            .map(|step| Self::hash(step) as usize)
            .sum::<usize>()
    }

    fn focusing_power(&self) -> usize {
        let mut hashmap: [Vec<(String, usize)>; 256] = std::array::from_fn(|_| Vec::new());
        'step: for step in self.steps.iter() {
            let (hash, label, op, num) = Self::extract(step);
            println!("{step:?} hash:{hash} label:{label} op:{op} num:{num}");
            match op {
                '-' => {
                    // Remove label from box
                    hashmap[hash as usize].retain(|(l, _)| l != &label);
                }
                '=' => {
                    // Add/Set label in box
                    for (l, n) in hashmap[hash as usize].iter_mut() {
                        if l == &label {
                            *n = num;
                            continue 'step;
                        }
                    }
                    hashmap[hash as usize].push((label, num));
                }
                _ => unreachable!(),
            }
        }

        for (bn, b) in hashmap.iter().enumerate() {
            for (sn, (label, num)) in b.iter().enumerate() {
                println!("Box {bn}: Slot {sn}:  [{label} {num}]");
            }
        }

        hashmap
            .iter()
            .enumerate()
            .map(|(bn, b)| {
                let bn = bn + 1;
                b.iter()
                    .enumerate()
                    .map(|(sn, (_, s))| bn * (sn + 1) * *s)
                    .sum::<usize>()
            })
            .sum::<usize>()
    }
}

impl Runner for Day15 {
    fn parse(&mut self, path: &str, _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_path(path, LinesOpt::RAW)?;
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
