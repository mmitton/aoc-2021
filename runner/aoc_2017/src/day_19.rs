#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

enum Dir {
    North,
    South,
    East,
    West,
}

#[derive(Default)]
pub struct Day19 {
    map: Vec<Vec<char>>,
}

impl Day19 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn walk(&self) -> (String, usize) {
        let max_x = self.map[0].len() - 1;
        let max_y = self.map.len() - 1;

        let mut letters = String::new();

        let mut x: usize = self.map[0].iter().position(|c| *c == '|').unwrap();
        let mut y: usize = 0;
        let mut dir = Dir::South;

        let mut steps = 0;
        'walk_loop: loop {
            steps += 1;
            let c = self.map[y as usize][x as usize];
            if c.is_alphabetic() {
                letters.push(c);
            }

            let (next_x, next_y) = match dir {
                Dir::North => (x, y.wrapping_sub(1)),
                Dir::South => (x, y + 1),
                Dir::East => (x.wrapping_sub(1), y),
                Dir::West => (x + 1, y),
            };

            if next_x > max_x || next_y > max_y || self.map[next_y][next_x] == ' ' {
                // Change direction
                match dir {
                    Dir::North | Dir::South => {
                        if x != 0 && self.map[y][x - 1] != ' ' {
                            x -= 1;
                            dir = Dir::East;
                        } else if x != max_x && self.map[y][x + 1] != ' ' {
                            x += 1;
                            dir = Dir::West;
                        } else {
                            break 'walk_loop;
                        }
                    }
                    Dir::West | Dir::East => {
                        if y != 0 && self.map[y - 1][x] != ' ' {
                            y -= 1;
                            dir = Dir::North;
                        } else if y != max_y && self.map[y + 1][x] != ' ' {
                            y += 1;
                            dir = Dir::South;
                        } else {
                            break 'walk_loop;
                        }
                    }
                }
            } else {
                x = next_x;
                y = next_y;
            }
        }

        (letters, steps)
    }
}

impl Runner for Day19 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            self.map.push(line.chars().collect());
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.walk().0.into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self.walk().1.into())
    }
}
