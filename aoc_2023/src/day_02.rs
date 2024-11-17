#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Debug)]
pub enum RunnerError {}

impl From<RunnerError> for Error {
    fn from(e: RunnerError) -> Self {
        Self::Runner(format!("{e:?}"))
    }
}

pub struct Day02 {
    games: Vec<Game>,
}

impl Day02 {
    pub fn new() -> Self {
        Self { games: Vec::new() }
    }
}

impl Runner for Day02 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        for line in Lines::from_bufread(file, LinesOpt::RAW)?.iter() {
            let mut game = Vec::new();
            let (_, line) = line.split_once(": ").expect("Could not split line");
            for line in line.split("; ") {
                let grab: Vec<(Color, usize)> = line
                    .split(", ")
                    .map(|part| {
                        let (cnt, color) = part.split_once(' ').expect("Could not split color");
                        let cnt = cnt.parse().unwrap();
                        let color = match color {
                            "red" => Color::Red,
                            "blue" => Color::Blue,
                            "green" => Color::Green,
                            _ => unreachable!(),
                        };
                        (color, cnt)
                    })
                    .collect();

                game.push(grab);
            }

            self.games.push(Game(game));
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

impl Day02 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        let mut ans = 0;
        for (ii, game) in self.games.iter().enumerate() {
            let possible = game.possible(12, 13, 14);
            println!("Game {}", ii + 1);
            println!("  possible:{possible}");
            println!();
            if possible {
                ans += ii + 1;
            }
        }
        Ok(ans.into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut ans = 0;
        for (ii, game) in self.games.iter().enumerate() {
            let (red, green, blue) = game.fewest();
            let power = red * green * blue;
            println!(
                "Game {}  {red} red, {green} green, {blue} blue.  Power = {power}",
                ii + 1
            );
            ans += power;
        }
        Ok(ans.into())
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
struct Game(Vec<Vec<(Color, usize)>>);

impl Game {
    fn possible(&self, r: usize, g: usize, b: usize) -> bool {
        for grab in self.0.iter() {
            for (color, cnt) in grab.iter() {
                match color {
                    Color::Red => {
                        if *cnt > r {
                            return false;
                        }
                    }
                    Color::Green => {
                        if *cnt > g {
                            return false;
                        }
                    }
                    Color::Blue => {
                        if *cnt > b {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }

    fn fewest(&self) -> (usize, usize, usize) {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for grab in self.0.iter() {
            for (color, cnt) in grab.iter() {
                match color {
                    Color::Red => red = red.max(*cnt),
                    Color::Green => green = green.max(*cnt),
                    Color::Blue => blue = blue.max(*cnt),
                }
            }
        }

        (red, green, blue)
    }
}
