use helper::BitArray;
#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Default, Debug)]
struct Step {
    write: bool,
    pos_delta: isize,
    next_state: usize,
}

#[derive(Default, Debug)]
struct State {
    steps: [Step; 2],
}

#[derive(Default)]
pub struct Day25 {
    states: [State; 10],
    current_state: usize,
    num_steps: usize,
}

impl Day25 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day25 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;

        self.current_state = (lines[0].chars().nth(15).unwrap() as u8 - b'A') as usize;
        self.num_steps = lines[1].split_whitespace().nth(5).unwrap().parse()?;

        for lines in lines[3..].chunks(10) {
            let state_name = (lines[0].chars().nth(9).unwrap() as u8 - b'A') as usize;
            let state = &mut self.states[state_name];
            state.steps[0].write = &lines[2][22..23] == "0";
            state.steps[0].pos_delta = if &lines[3][27..] == "right." { 1 } else { -1 };
            state.steps[0].next_state = (lines[4].chars().nth(26).unwrap() as u8 - b'A') as usize;
            state.steps[1].write = &lines[6][22..23] == "0";
            state.steps[1].pos_delta = if &lines[7][27..] == "right." { 1 } else { -1 };
            state.steps[1].next_state = (lines[8].chars().nth(26).unwrap() as u8 - b'A') as usize;
        }

        Ok(())
    }

    fn run_part(&mut self, part: u8) -> Result<RunOutput, Error> {
        match part {
            1 => self.part1(),
            _ => Err(Error::Skipped),
        }
    }
}

impl Day25 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        const OFFSET: isize = 4096;
        let mut tape: BitArray = BitArray::new(OFFSET as usize * 3);
        let mut pos = 0isize;
        let mut current_state: usize = self.current_state;

        for _ in 0..self.num_steps {
            let tape_idx = (OFFSET + pos) as usize;
            let cur_num = if tape.get(tape_idx) { 1 } else { 0 };

            let state = &self.states[current_state];
            tape.set(tape_idx, state.steps[cur_num].write);
            pos += state.steps[cur_num].pos_delta;
            current_state = state.steps[cur_num].next_state;
        }

        Ok(tape.popcount().into())
    }
}
