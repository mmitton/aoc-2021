#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};

enum Move {
    U,
    D,
    L,
    R,
}

pub struct Day09 {
    moves: Vec<(Move, usize)>,
}

impl Day09 {
    pub fn new() -> Self {
        Self { moves: Vec::new() }
    }

    fn move_rope<const N: usize>(&self) -> usize {
        let mut knots: [(i16, i16); N] = std::array::from_fn(|_| (0, 0));

        let mut tail_pos = std::collections::HashSet::new();

        for (m, cnt) in self.moves.iter() {
            for _ in 0..*cnt {
                match m {
                    Move::U => knots[0].1 -= 1,
                    Move::D => knots[0].1 += 1,
                    Move::L => knots[0].0 -= 1,
                    Move::R => knots[0].0 += 1,
                }

                for i in 1..knots.len() {
                    let x_diff = knots[i - 1].0 - knots[i].0;
                    let y_diff = knots[i - 1].1 - knots[i].1;

                    if y_diff.abs() > 1 {
                        knots[i].1 += if y_diff > 0 { 1 } else { -1 };
                        if x_diff != 0 {
                            knots[i].0 += if x_diff > 0 { 1 } else { -1 };
                        }
                    } else if x_diff.abs() > 1 {
                        knots[i].0 += if x_diff > 0 { 1 } else { -1 };
                        if y_diff != 0 {
                            knots[i].1 += if y_diff > 0 { 1 } else { -1 };
                        }
                    }
                }

                tail_pos.insert(knots[knots.len() - 1]);
            }
        }

        tail_pos.len()
    }
}

impl Runner for Day09 {
    fn parse(&mut self, path: &str, _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_path(path, LinesOpt::RAW)?;
        self.moves.extend(lines.iter().map(|line| {
            let (dir, cnt) = line.split_once(' ').unwrap();
            let cnt = cnt.parse().unwrap();
            let dir = match dir {
                "U" => Move::U,
                "D" => Move::D,
                "L" => Move::L,
                "R" => Move::R,
                _ => unreachable!(),
            };
            (dir, cnt)
        }));

        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.move_rope::<2>().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self.move_rope::<10>().into())
    }
}
