#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

struct Move {
    dx: isize,
    dy: isize,
}

impl TryFrom<char> for Move {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Self { dx: 0, dy: -1 }),
            'v' => Ok(Self { dx: 0, dy: 1 }),
            '>' => Ok(Self { dx: 1, dy: 0 }),
            '<' => Ok(Self { dx: -1, dy: 0 }),
            _ => Err(Error::InvalidInput(format!("{value}"))),
        }
    }
}

#[derive(Default)]
pub struct Day03 {
    moves: Vec<Move>,
}

impl Day03 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day03 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        assert_eq!(lines.len(), 1);
        for c in lines[0].chars() {
            self.moves.push(c.try_into()?);
        }
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

impl Day03 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        let mut seen = HashSet::default();
        let (mut x, mut y) = (0, 0);
        seen.insert((0, 0));
        for m in self.moves.iter() {
            x += m.dx;
            y += m.dy;
            seen.insert((x, y));
        }
        Ok(seen.len().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut seen = HashSet::default();
        let (mut x0, mut y0) = (0, 0);
        let (mut x1, mut y1) = (0, 0);
        seen.insert((0, 0));
        for m in self.moves.chunks(2) {
            x0 += m[0].dx;
            y0 += m[0].dy;
            x1 += m[1].dx;
            y1 += m[1].dy;
            seen.insert((x0, y0));
            seen.insert((x1, y1));
        }
        Ok(seen.len().into())
    }
}
