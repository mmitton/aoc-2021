use crate::intcode::{IntCode, State, Word};
use helper::{Error, Lines};
use std::{
    collections::{BTreeSet, VecDeque},
    ops::{Deref, DerefMut},
    str::FromStr,
};

#[derive(Default, Clone)]
pub(crate) struct IntCodeMulti<T> {
    chips: Vec<IntCode<T>>,
    pub(crate) output: Option<T>,
    pub(crate) master_output: usize,
    pub(crate) outputs: Vec<Vec<usize>>,
}

impl<T> Deref for IntCodeMulti<T> {
    type Target = [IntCode<T>];

    fn deref(&self) -> &Self::Target {
        &self.chips
    }
}

impl<T> DerefMut for IntCodeMulti<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.chips
    }
}

impl<T> IntCodeMulti<T>
where
    T: Word + Default,
    Error: From<<T as FromStr>::Err>,
{
    pub(crate) fn load(&mut self, lines: Lines, n: usize) -> Result<(), Error> {
        let mut intcode = IntCode::default();
        intcode.load(lines)?;

        self.chips = vec![intcode.clone(); n];
        self.outputs = (0..n)
            .map(|i| if i == n - 1 { Vec::new() } else { vec![i + 1] })
            .collect();

        self.master_output = n - 1;
        self.output = None;
        Ok(())
    }

    pub(crate) fn run(&mut self) {
        let mut runnable: VecDeque<usize> = self.chips.iter().enumerate().map(|(i, _)| i).collect();
        let mut waiting: BTreeSet<usize> = BTreeSet::new();
        while let Some(idx) = runnable.pop_front() {
            let state = self.chips[idx].run();
            match state {
                State::Stopped => {}
                State::WaitingForInput(_) => {
                    waiting.insert(idx);
                }
                State::HasOutput(v) => {
                    if self.master_output == idx {
                        self.output = Some(v);
                    }
                    for idx in self.outputs[idx].iter() {
                        self.chips[*idx].input.push_back(v);
                        if waiting.remove(idx) {
                            // Add waiting chip back to the running queue
                            runnable.push_back(*idx);
                        }
                    }
                    runnable.push_back(idx);
                }
                State::Running => unreachable!(),
            }
        }
    }
}
