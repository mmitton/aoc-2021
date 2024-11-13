#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Debug, Copy, Clone)]
struct Coord(isize, isize, isize, isize);

impl Coord {
    fn dist_to(&self, rhs: &Self) -> isize {
        (self.0 - rhs.0).abs()
            + (self.1 - rhs.1).abs()
            + (self.2 - rhs.2).abs()
            + (self.3 - rhs.3).abs()
    }
}

#[derive(Default)]
pub struct Day25 {
    set: Vec<Coord>,
}

impl Day25 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day25 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            let parts: Vec<&str> = line.split(",").collect();
            self.set.push(Coord(
                parts[0].parse()?,
                parts[1].parse()?,
                parts[2].parse()?,
                parts[3].parse()?,
            ));
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        let mut constellations: Vec<Vec<Coord>> = Vec::new();
        for c in self.set.iter() {
            constellations.push(vec![*c]);
        }

        loop {
            let mut merged = false;
            for i in 0..constellations.len() {
                let mut j = i + 1;
                loop {
                    if j >= constellations.len() {
                        break;
                    }
                    'check_loop: for ii in 0..constellations[i].len() {
                        for jj in 0..constellations[j].len() {
                            if constellations[i][ii].dist_to(&constellations[j][jj]) <= 3 {
                                // Merge i and j
                                merged = true;
                                let merge = constellations[j].clone();
                                constellations[i].extend_from_slice(&merge);
                                constellations.remove(j);
                                j -= 1;
                                break 'check_loop;
                            }
                        }
                    }
                    j += 1;
                }
            }

            if !merged {
                break;
            }
        }

        Ok(constellations.len().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Err(Error::Skipped)
    }
}
