use crate::intcode::{IntCode, State, Word};
#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};
use std::collections::VecDeque;

#[derive(Default, Clone)]
pub(crate) struct Amplifiers<T> {
    amps: Vec<IntCode<T>>,
    pub(crate) output: Option<T>,
    pub(crate) master_output: usize,
    pub(crate) outputs: Vec<Vec<usize>>,
}

impl<T> std::ops::Deref for Amplifiers<T> {
    type Target = [IntCode<T>];

    fn deref(&self) -> &Self::Target {
        &self.amps
    }
}

impl<T> std::ops::DerefMut for Amplifiers<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.amps
    }
}
impl<T> Amplifiers<T>
where
    T: Word + Default,
    Error: From<<T as std::str::FromStr>::Err>,
{
    pub(crate) fn load(&mut self, lines: Lines, n: usize) -> Result<(), Error> {
        let mut intcode = IntCode::default();
        intcode.load(lines)?;

        self.amps = vec![intcode.clone(); n];
        self.outputs = (0..n)
            .map(|i| if i == n - 1 { Vec::new() } else { vec![i + 1] })
            .collect();

        self.master_output = n - 1;
        self.output = None;
        Ok(())
    }

    pub(crate) fn run(&mut self) {
        let mut runnable: VecDeque<usize> = self.amps.iter().enumerate().map(|(i, _)| i).collect();
        let mut waiting: Vec<usize> = Vec::new();
        while let Some(idx) = runnable.pop_front() {
            let state = self.amps[idx].run();
            match state {
                State::Stopped => {}
                State::WaitingForInput(..) => {
                    waiting.push(idx);
                }
                State::HasOutput(v) => {
                    if self.master_output == idx {
                        self.output = Some(v);
                    }
                    for idx in self.outputs[idx].iter() {
                        self.amps[*idx].input.push_back(v);
                        if let Some(pos) = waiting.iter().position(|v| v == idx) {
                            // Add waiting chip back to the running queue
                            runnable.push_back(*idx);
                            waiting.swap_remove(pos);
                        }
                    }
                    runnable.push_back(idx);
                }
                State::Running => unreachable!(),
            }
        }
    }
}

pub struct Day07 {
    amps: Amplifiers<i32>,
}

impl Day07 {
    pub fn new() -> Self {
        Self {
            amps: Amplifiers::default(),
        }
    }

    fn generate<F>(n: usize, a: &mut Vec<i32>, f: &mut F)
    where
        F: FnMut(&[i32]),
    {
        if n == 1 {
            f(a);
        } else {
            for i in 0..n - 1 {
                Self::generate(n - 1, a, f);

                if n % 2 == 0 {
                    a.swap(i, n - 1);
                } else {
                    a.swap(0, n - 1);
                }
            }
            Self::generate(n - 1, a, f);
        }
    }

    pub fn get_best(&self, low: i32) -> i32 {
        let mut numbers = (low..low + 5).collect();
        let mut best = 0;
        Self::generate(5, &mut numbers, &mut |vals| {
            let mut amps = self.amps.clone();
            for (i, v) in vals.iter().enumerate() {
                amps[i].input.push_front(*v);
            }
            amps[0].input.push_back(0);
            amps.run();
            best = best.max(amps.output.unwrap());
        });
        best
    }
}

impl Runner for Day07 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        self.amps.load(Lines::from_bufread(file, LinesOpt::RAW)?, 5)
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.get_best(0).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        self.amps.outputs[4].push(0);
        Ok(self.get_best(5).into())
    }
}
