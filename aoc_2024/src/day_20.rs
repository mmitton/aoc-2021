#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Point2D};
use std::collections::BTreeMap;

#[derive(Default)]
pub struct Day20 {
    track: HashMap<Point2D<usize>, usize>,
    start: Point2D<usize>,
    end: Point2D<usize>,
}

impl Day20 {
    pub fn new() -> Self {
        Self::default()
    }

    fn flood_fill(&mut self) -> usize {
        let mut work: BTreeMap<usize, HashSet<Point2D<usize>>> = BTreeMap::new();
        work.entry(0).or_default().insert(self.start);

        while let Some((cost, points)) = work.pop_first() {
            for p in points {
                for n in p.cardinal_neighbors() {
                    if let Some(c) = self.track.get_mut(&n) {
                        if *c > cost + 1 {
                            *c = cost + 1;
                            work.entry(cost + 1).or_default().insert(n);
                        }
                    }
                }
            }
        }

        *self.track.get(&self.end).unwrap()
    }

    fn find_cheats(&self, max_skip: usize, min_cheat: usize) -> usize {
        let mut cheats = 0;
        let mut track: Vec<(usize, Point2D<usize>)> =
            self.track.iter().map(|(p, t)| (*t, *p)).collect();
        track.sort();
        for (i, (start_time, start)) in track.iter().enumerate() {
            for (end_time, end) in track.iter().skip(i + 1) {
                let skip_path = start.manhattan_dist(end);
                let cheat_time = end_time.saturating_sub(start_time + skip_path);
                if skip_path <= max_skip && cheat_time >= min_cheat {
                    cheats += 1;
                }
            }
        }
        cheats
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        let min_cheat = if self.track.len() == 85 { 12 } else { 100 };
        self.flood_fill();
        let cheats = self.find_cheats(2, min_cheat);

        Ok(cheats.into())
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        let min_cheat = if self.track.len() == 85 { 74 } else { 100 };
        self.flood_fill();
        let cheats = self.find_cheats(20, min_cheat);

        Ok(cheats.into())
    }
}

impl helper::Runner for Day20 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let p = Point2D::new(x, y);
                match c {
                    '.' => {
                        self.track.insert(p, usize::MAX);
                    }
                    'S' => {
                        self.start = p;
                        self.track.insert(p, 0);
                    }
                    'E' => {
                        self.end = p;
                        self.track.insert(p, usize::MAX);
                    }
                    '#' => {}
                    _ => return Err(Error::InvalidInput(line.into())),
                }
            }
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
