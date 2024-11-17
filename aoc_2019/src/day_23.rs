use crate::intcode::{IntCode, State};
#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};

struct Nic {
    intcode: IntCode<isize>,
    packet: Vec<isize>,
    idle: usize,
}

pub struct Day23 {
    nics: Vec<Nic>,
    nat: [isize; 2],
}

impl Day23 {
    pub fn new() -> Self {
        Self {
            nics: Vec::new(),
            nat: [0; 2],
        }
    }

    fn run<F>(&mut self, mut is_done: F)
    where
        F: FnMut(bool, isize) -> bool,
    {
        loop {
            let mut idle_cnt = 0;
            for i in 0..50 {
                let nic = &mut self.nics[i];
                if nic.intcode.is_stopped() {
                    unreachable!();
                }
                match nic.intcode.run() {
                    State::HasOutput(v) => {
                        nic.packet.push(v);
                        if nic.packet.len() == 3 {
                            let to = nic.packet[0];
                            let x = nic.packet[1];
                            let y = nic.packet[2];
                            nic.packet.clear();
                            if to == 255 {
                                self.nat[0] = x;
                                self.nat[1] = y;
                                if is_done(false, y) {
                                    return;
                                }
                            } else {
                                let to = &mut self.nics[to as usize];
                                to.intcode.input.push_back(x);
                                to.intcode.input.push_back(y);
                                to.idle = 0;
                            }
                        }
                    }
                    State::WaitingForInput(..) => {
                        nic.intcode.input.push_front(-1);
                        if nic.idle != 0 {
                            idle_cnt += 1;
                        }
                        nic.idle += 1;
                    }
                    State::Stopped => continue,
                    x => unreachable!("Unexpected state: {x:?}"),
                }
            }
            if idle_cnt == 50 {
                if is_done(true, self.nat[1]) {
                    return;
                }
                let nic = &mut self.nics[0];
                nic.idle = 0;
                nic.intcode.input.push_back(self.nat[0]);
                nic.intcode.input.push_back(self.nat[1]);
            }
        }
    }
}

impl Runner for Day23 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let mut master: IntCode<_> = IntCode::default();
        master.load(Lines::from_bufread(file, LinesOpt::RAW)?)?;

        self.nics.extend((0..50).map(|i| {
            let mut intcode = master.clone();
            intcode.input.push_back(i);
            Nic {
                intcode,
                packet: Vec::with_capacity(3),
                idle: 0,
            }
        }));
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

impl Day23 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        self.run(|_, _| true);
        Ok(self.nat[1].into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut last_y = isize::MAX;
        self.run(|part2, y| {
            if !part2 {
                false
            } else {
                let is_done = last_y == y;
                last_y = y;
                is_done
            }
        });
        Ok(self.nat[1].into())
    }
}
