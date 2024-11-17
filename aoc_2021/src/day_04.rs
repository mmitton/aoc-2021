#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Default)]
struct Board {
    win: bool,
    numbers: [[u8; 5]; 5],
}

impl Board {
    fn mark(&mut self, num: u8) {
        if !self.win {
            self.numbers.iter_mut().for_each(|row| {
                let mut all_marked = true;
                row.iter_mut().for_each(|n| {
                    if *n == num {
                        *n = !0;
                    } else if *n != !0 {
                        all_marked = false;
                    }
                });

                if all_marked {
                    self.win = true;
                }
            });

            for x in 0..5 {
                if self.numbers[0][x] != !0
                    || self.numbers[1][x] != !0
                    || self.numbers[2][x] != !0
                    || self.numbers[3][x] != !0
                    || self.numbers[4][x] != !0
                {
                    continue;
                }
                self.win = true;
            }
        }
    }

    fn uncalled_sum(&self) -> usize {
        self.numbers.iter().copied().fold(0, |sum, row| {
            sum + row
                .iter()
                .filter_map(|n| if *n != !0 { Some(*n as usize) } else { None })
                .sum::<usize>()
        })
    }
}

pub struct Day04 {
    numbers: Vec<u8>,
    boards: Vec<Board>,
}

impl Day04 {
    pub fn new() -> Self {
        Self {
            numbers: Vec::new(),
            boards: Vec::new(),
        }
    }
}

impl Runner for Day04 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        self.numbers
            .extend(lines[0].split(',').map(|n| n.parse::<u8>().unwrap()));

        lines[2..].chunks(6).for_each(|b| {
            let mut board = Board::default();
            for (y, row) in b[..5].iter().enumerate() {
                for (x, n) in row.split_whitespace().enumerate() {
                    board.numbers[y][x] = n.parse().unwrap();
                }
            }
            self.boards.push(board);
        });
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

impl Day04 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        for num in self.numbers.iter().copied() {
            for board in self.boards.iter_mut() {
                board.mark(num);
                if board.win {
                    return Ok((board.uncalled_sum() * num as usize).into());
                }
            }
        }
        Err(Error::Unsolved)
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        for num in self.numbers.iter().copied() {
            for board in self.boards.iter_mut() {
                board.mark(num);
            }

            if self.boards.len() == 1 && self.boards[0].win {
                return Ok((self.boards[0].uncalled_sum() * num as usize).into());
            }
            self.boards.retain(|b| !b.win);
        }
        Err(Error::Unsolved)
    }
}
