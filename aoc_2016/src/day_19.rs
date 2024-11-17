#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Debug)]
struct Elf {
    num: usize,
    presents: usize,
    next: usize,
    prev: usize,
}

#[derive(Default)]
pub struct Day19 {
    elves: Vec<Elf>,
}

impl Day19 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day19 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        assert_eq!(lines.len(), 1);
        let num: usize = lines[0].parse()?;

        self.elves.extend((0..num).map(|num| Elf {
            num: num + 1,
            presents: 1,
            next: num + 1,
            prev: num.saturating_sub(1),
        }));

        self.elves[0].prev = num - 1;
        self.elves[num - 1].next = 0;

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

impl Day19 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        let mut cur = 0;
        let mut elves_left = self.elves.len();
        loop {
            let next = self.elves[cur].next;
            self.elves[cur].presents += self.elves[next].presents;
            self.elves[next].presents = 0;

            // Remove next
            let prev = self.elves[next].prev;
            let next = self.elves[next].next;
            self.elves[prev].next = next;
            self.elves[next].prev = prev;

            elves_left -= 1;
            cur = self.elves[cur].next;

            if elves_left == 1 {
                break;
            }
        }
        Ok(self.elves[cur].num.into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut cur = 0;
        let mut across = self.elves.len() / 2;
        let mut elves_left = self.elves.len();
        loop {
            self.elves[cur].presents += self.elves[across].presents;
            self.elves[across].presents = 0;

            // Remove across
            let prev = self.elves[across].prev;
            let next = self.elves[across].next;
            self.elves[prev].next = next;
            self.elves[next].prev = prev;

            elves_left -= 1;
            if elves_left % 2 == 0 {
                across = self.elves[next].next;
            } else {
                across = next;
            }

            cur = self.elves[cur].next;

            if elves_left == 1 {
                break;
            }
        }
        Ok(self.elves[cur].num.into())
    }
}
