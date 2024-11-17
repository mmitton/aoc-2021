#[allow(unused_imports)]
use helper::{
    print, println, BitGrid, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Spot {
    Floor,
    Vacant,
    Occuipied,
}

pub struct Day11 {
    tiles: Vec<Vec<[Spot; 2]>>,
}

impl Day11 {
    pub fn new() -> Self {
        Self { tiles: Vec::new() }
    }

    fn simulate<const MAX_DIST: usize, const SURROUND: usize>(&mut self) -> usize {
        let height = self.tiles.len();
        let width = self.tiles[0].len();
        for round in 0.. {
            let cur = if round % 2 == 0 { 0 } else { 1 };
            let next = 1 - cur;
            let mut changed = false;

            for y in 0..height {
                for x in 0..width {
                    if self.tiles[y][x][cur] == Spot::Floor {
                        continue;
                    }
                    let mut neighbors = 0;
                    for (dx, dy) in [
                        (-1, -1),
                        (0, -1),
                        (1, -1),
                        (-1, 0),
                        (1, 0),
                        (-1, 1),
                        (0, 1),
                        (1, 1),
                    ]
                    .iter()
                    {
                        for d in 1..=MAX_DIST {
                            let nx = (x as isize + (d as isize * dx)) as usize;
                            let ny = (y as isize + (d as isize * dy)) as usize;
                            if nx >= width || ny >= height {
                                break;
                            }
                            match self.tiles[ny][nx][cur] {
                                Spot::Vacant => break,
                                Spot::Occuipied => {
                                    neighbors += 1;
                                    break;
                                }
                                Spot::Floor => {}
                            }
                        }
                    }
                    self.tiles[y][x][next] = Spot::Vacant;
                    if self.tiles[y][x][cur] == Spot::Vacant {
                        if neighbors == 0 {
                            self.tiles[y][x][next] = Spot::Occuipied;
                            changed = true;
                        }
                    } else if neighbors < SURROUND {
                        self.tiles[y][x][next] = Spot::Occuipied;
                    } else {
                        changed = true;
                    }
                }
            }

            if !changed {
                return self.tiles.iter().fold(0, |count, row| {
                    count
                        + row
                            .iter()
                            .filter(|s| matches!(s[cur], Spot::Occuipied))
                            .count()
                });
            }
        }
        0
    }
}

impl Runner for Day11 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            self.tiles.push(
                line.chars()
                    .map(|c| match c {
                        '.' => [Spot::Floor, Spot::Floor],
                        'L' => [Spot::Vacant, Spot::Vacant],
                        '#' => [Spot::Occuipied, Spot::Vacant],
                        _ => unreachable!(),
                    })
                    .collect(),
            );
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

impl Day11 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.simulate::<1, 4>().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self.simulate::<1024, 5>().into())
    }
}
