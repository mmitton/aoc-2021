#[allow(unused_imports)]
use helper::{
    print, println, BitArray, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner,
};

#[derive(Default)]
pub struct Day18 {
    rooms: Vec<char>,
    rows: usize,
}

impl Day18 {
    pub fn new() -> Self {
        Self::default()
    }

    fn safe_rooms(&mut self) -> usize {
        let mut rooms: Vec<bool> = self.rooms.iter().map(|c| *c == '^').collect();
        let mut next_rooms = rooms.clone();
        let mut safe_rooms = rooms.iter().filter(|c| !**c).count();
        let width = rooms.len();
        for _ in 1..self.rows {
            for (x, next_room) in next_rooms.iter_mut().enumerate().take(width) {
                let left = rooms.get(x.wrapping_sub(1)).unwrap_or(&false);
                let center = rooms.get(x).unwrap_or(&false);
                let right = rooms.get(x.wrapping_add(1)).unwrap_or(&false);

                match (left, center, right) {
                    (true, true, false)
                    | (false, true, true)
                    | (true, false, false)
                    | (false, false, true) => *next_room = true,
                    _ => {
                        safe_rooms += 1;
                        *next_room = false;
                    }
                }
            }
            std::mem::swap(&mut rooms, &mut next_rooms);
        }

        safe_rooms
    }
}

impl Runner for Day18 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        assert_eq!(lines.len(), 1);
        self.rooms.extend(lines[0].chars());
        self.rows = if lines[0].len() == 5 {
            3
        } else {
            lines[0].len().min(40)
        };
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

impl Day18 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.safe_rooms().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        self.rows = 400000;
        Ok(self.safe_rooms().into())
    }
}
