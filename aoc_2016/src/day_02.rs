#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Default)]
pub struct Day02 {
    instructions: Vec<Vec<char>>,
}

impl Day02 {
    pub fn new() -> Self {
        Self::default()
    }

    fn get_code(&self, keypad: &[&[Option<char>]]) -> String {
        let mut code = String::with_capacity(self.instructions.len());

        let mut xy = (keypad[0].len() / 2, keypad.len() / 2);
        #[allow(clippy::needless_range_loop)]
        for y in 0..keypad.len() {
            for x in 0..keypad[y].len() {
                if keypad[y][x] == Some('5') {
                    xy.0 = x;
                    xy.1 = y;
                }
            }
        }
        println!("starting at {xy:?}");

        for instructions in self.instructions.iter() {
            for m in instructions.iter() {
                let next_xy = match m {
                    'U' => (xy.0, xy.1 - 1),
                    'D' => (xy.0, xy.1 + 1),
                    'L' => (xy.0 - 1, xy.1),
                    'R' => (xy.0 + 1, xy.1),
                    _ => unreachable!(),
                };

                if keypad[next_xy.1][next_xy.0].is_some() {
                    xy = next_xy;
                }
            }

            code.push(keypad[xy.1][xy.0].unwrap());
        }

        code
    }
}

impl Runner for Day02 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            self.instructions.push(line.chars().collect());
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        let keypad: &[&[Option<char>]] = &[
            &[None, None, None, None, None],
            &[None, Some('1'), Some('2'), Some('3'), None],
            &[None, Some('4'), Some('5'), Some('6'), None],
            &[None, Some('7'), Some('8'), Some('9'), None],
            &[None, None, None, None, None],
        ];

        Ok(self.get_code(keypad).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let keypad: &[&[Option<char>]] = &[
            &[None, None, None, None, None, None, None],
            &[None, None, None, Some('1'), None, None, None],
            &[None, None, Some('2'), Some('3'), Some('4'), None, None],
            &[
                None,
                Some('5'),
                Some('6'),
                Some('7'),
                Some('8'),
                Some('9'),
                None,
            ],
            &[None, None, Some('A'), Some('B'), Some('C'), None, None],
            &[None, None, None, Some('D'), None, None, None],
            &[None, None, None, None, None, None, None],
        ];

        Ok(self.get_code(keypad).into())
    }
}
