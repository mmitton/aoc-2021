#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};
use std::collections::BTreeMap;

#[derive(Debug)]
struct Step {
    write: u8,
    pos_delta: isize,
    next_state: char,
}

#[derive(Debug)]
struct State {
    steps: [Step; 2],
}

#[derive(Default)]
pub struct Day25 {
    states: BTreeMap<char, State>,
    current_state: char,
    num_steps: usize,
}

impl Day25 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day25 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;

        self.current_state = lines[0].chars().nth(15).unwrap();
        self.num_steps = lines[1].split_whitespace().nth(5).unwrap().parse()?;

        for lines in lines[3..].chunks(10) {
            let state_name = lines[0].chars().nth(9).unwrap();
            let state = State {
                steps: [
                    Step {
                        write: if &lines[2][22..23] == "0" { 0 } else { 1 },
                        pos_delta: if &lines[3][27..] == "right." { 1 } else { -1 },
                        next_state: lines[4].chars().nth(26).unwrap(),
                    },
                    Step {
                        write: if &lines[6][22..23] == "0" { 0 } else { 1 },
                        pos_delta: if &lines[7][27..] == "right." { 1 } else { -1 },
                        next_state: lines[8].chars().nth(26).unwrap(),
                    },
                ],
            };
            self.states.insert(state_name, state);
        }

        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        let mut pos = 0isize;
        let mut tape: HashSet<isize> = HashSet::default();

        for _ in 0..self.num_steps {
            let cur_num = if tape.contains(&pos) { 1 } else { 0 };
            let state = self.states.get(&self.current_state).unwrap();

            if state.steps[cur_num].write == 0 {
                tape.remove(&pos);
            } else {
                tape.insert(pos);
            }
            pos += state.steps[cur_num].pos_delta;
            self.current_state = state.steps[cur_num].next_state;
        }
        Ok(tape.len().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Err(Error::Skipped)
    }
}
