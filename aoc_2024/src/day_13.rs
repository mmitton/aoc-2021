#[allow(unused_imports)]
use helper::{print, println, Dijkstra, Error, HashMap, HashSet, Lines, LinesOpt, Point2D};

#[derive(Debug)]
struct Game {
    a: Point2D<usize>,
    b: Point2D<usize>,
    prize: Point2D<usize>,
}

impl Game {
    fn simple_cheapest(&self, min_a: usize, max_a: usize) -> Option<usize> {
        let mut cheapest = None;

        for a in min_a..=max_a {
            let ax = self.a.x * a;
            let ay = self.a.y * a;

            if ax > self.prize.x || ay > self.prize.y {
                break;
            }

            let b = ((self.prize.x - ax) / self.b.x).min((self.prize.y - ay) / self.b.y);
            let x = ax + self.b.x * b;
            let y = ay + self.b.y * b;
            if x == self.prize.x && y == self.prize.y {
                cheapest = Some(cheapest.unwrap_or(usize::MAX).min(a * 3 + b));
            }
        }
        cheapest
    }

    fn big_cheapest(&self) -> Option<usize> {
        let x1 = self.a.x as isize;
        let x2 = self.b.x as isize;
        let x3 = self.prize.x as isize;

        let y1 = self.a.y as isize;
        let y2 = self.b.y as isize;
        let y3 = self.prize.y as isize;

        let b_num = x3 * y1 - x1 * y3;
        let b_div = y1 * x2 - x1 * y2;

        if b_num % b_div == 0 {
            let b = (b_num / b_div) as usize;
            let a = (self.prize.x - b * self.b.x) / self.a.x;
            self.simple_cheapest(a, a)
        } else {
            None
        }
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
            .flat_map(|g| g.simple_cheapest(0, 100))
            .sum::<usize>()
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
            .flat_map(|g| g.big_cheapest())
            .sum::<usize>()
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
                    let x: usize = parts[2 + offset].parse()?;
                    let y: usize = parts[4 + offset].parse()?;
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
