#[allow(unused_imports)]
use helper::{print, println, Dijkstra, Error, HashMap, HashSet, Lines, LinesOpt, Point2D};

#[derive(Debug)]
struct Game {
    a: Point2D<isize>,
    b: Point2D<isize>,
    prize: Point2D<isize>,
}

impl Game {
    fn solve(&self) -> Option<isize> {
        let b_num = self.prize.x * self.a.y - self.a.x * self.prize.y;
        let b_div = self.a.y * self.b.x - self.a.x * self.b.y;

        if b_num % b_div == 0 {
            let b = b_num / b_div;
            let a = (self.prize.x - b * self.b.x) / self.a.x;
            if self.a.scale(a) + self.b.scale(b) == self.prize {
                return Some(a * 3 + b);
            }
        }
        None
    }
}

#[derive(Default)]
pub struct Day13 {
    games: Vec<Game>,
}

impl Day13 {
    pub fn new() -> Self {
        Self::default()
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self
            .games
            .iter()
            .flat_map(Game::solve)
            .sum::<isize>()
            .into())
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        self.games.iter_mut().for_each(|g| {
            g.prize.x += 10000000000000;
            g.prize.y += 10000000000000;
        });
        Ok(self
            .games
            .iter()
            .flat_map(Game::solve)
            .sum::<isize>()
            .into())
    }
}

impl helper::Runner for Day13 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::REMOVE_EMPTY)?;
        for chunk in lines.chunks(3) {
            macro_rules! parse_line {
                ($line:expr) => {{
                    let line = $line.replace(&[',', '+', '='], " ");
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    let offset = if parts.len() == 6 { 1 } else { 0 };
                    let x: isize = parts[2 + offset].parse()?;
                    let y: isize = parts[4 + offset].parse()?;
                    Point2D::new(x, y)
                }};
            }

            let a = parse_line!(chunk[0]);
            let b = parse_line!(chunk[1]);
            let prize = parse_line!(chunk[2]);
            self.games.push(Game { a, b, prize });
        }
        Ok(())
    }

    fn run_part(&mut self, part: u8) -> Result<helper::RunOutput, Error> {
        match part {
            1 => self.part1(),
            2 => self.part2(),
            _ => Err(Error::Skipped),
        }
    }
}
