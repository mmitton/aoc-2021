#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

pub struct Day17 {
    initial: Vec<[i8; 2]>,
}

impl Day17 {
    pub fn new() -> Self {
        Self {
            initial: Vec::new(),
        }
    }

    pub fn boot(&self, w_delta: i8) -> usize {
        let mut cur = HashMap::default();
        let mut next = HashMap::default();

        fn set(map: &mut HashMap<[i8; 4], (u8, bool)>, c: [i8; 4], w_delta: i8) {
            map.entry(c).or_default().1 = true;
            for w in -w_delta..=w_delta {
                for z in -1..=1 {
                    for y in -1..=1 {
                        for x in -1..=1 {
                            if x != 0 || y != 0 || z != 0 || w != 0 {
                                map.entry([c[0] + x, c[1] + y, c[2] + z, c[3] + w])
                                    .or_default()
                                    .0 += 1;
                            }
                        }
                    }
                }
            }
        }

        for initial in self.initial.iter() {
            set(&mut cur, [initial[0], initial[1], 0, 0], w_delta);
        }

        for _ in 0..6 {
            next.clear();
            for (c, v) in cur.iter() {
                if v.1 && (v.0 == 2 || v.0 == 3) {
                    set(&mut next, *c, w_delta);
                }
                if !v.1 && v.0 == 3 {
                    set(&mut next, *c, w_delta);
                }
            }
            std::mem::swap(&mut cur, &mut next);
        }

        cur.values().filter(|(_, on)| *on).count()
    }
}

impl Runner for Day17 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for (y, row) in lines.iter().enumerate() {
            for (x, c) in row.chars().enumerate() {
                if c == '#' {
                    self.initial.push([x as i8, y as i8]);
                }
            }
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.boot(0).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self.boot(1).into())
    }
}
