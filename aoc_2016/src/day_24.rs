use std::collections::{BTreeSet, VecDeque};

#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Default)]
pub struct Day24 {
    grid: Vec<Vec<char>>,
    locations: HashMap<u8, (usize, usize)>,
    paths: HashMap<u8, Vec<(u8, usize)>>,
}

impl Day24 {
    pub fn new() -> Self {
        Self::default()
    }

    fn find_paths(&mut self) {
        for (from_num, (x, y)) in self.locations.iter() {
            let mut paths = Vec::new();
            let mut seen = HashSet::default();
            let mut work_queue = VecDeque::new();
            seen.insert((*x, *y));
            work_queue.push_front((0, (*x, *y)));
            while let Some((dist, (x, y))) = work_queue.pop_front() {
                let next_dist = dist + 1;
                macro_rules! check {
                    ($x:expr, $y:expr) => {{
                        if seen.insert(($x, $y)) {
                            match self.grid[$y][$x] {
                                '.' => work_queue.push_back((next_dist, ($x, $y))),
                                c if c.is_ascii_digit() => {
                                    paths.push((c as u8 - b'0', next_dist));
                                    work_queue.push_back((next_dist, ($x, $y)));
                                }
                                _ => {}
                            }
                        }
                    }};
                }
                check!(x - 1, y);
                check!(x + 1, y);
                check!(x, y - 1);
                check!(x, y + 1);
            }

            self.paths.insert(*from_num, paths);
        }

        for from_num in self.locations.keys() {
            if let Some(to) = self.paths.get(from_num) {
                assert_eq!(to.len(), self.locations.len() - 1);
            } else {
                panic!("No paths from {from_num}");
            }
        }
    }

    fn find_shortest_path(&mut self, return_home: bool) -> usize {
        self.find_paths();

        let mut full_mask: u8 = 0;
        for from in self.locations.keys() {
            full_mask |= 1 << from;
        }

        let mut work_queue: BTreeSet<(usize, u8, u8)> = BTreeSet::default();
        work_queue.insert((0, 0b1, 0));

        while let Some((dist, seen, at)) = work_queue.pop_first() {
            let paths = self.paths.get(&at).unwrap();
            if seen == full_mask {
                if return_home {
                    if at == 0 {
                        return dist;
                    } else {
                        // Go home
                        for (to, to_dist) in paths.iter().copied() {
                            if to == 0 {
                                work_queue.insert((dist + to_dist, seen, 0));
                            }
                        }
                    }
                } else {
                    return dist;
                }
            }

            for (to, to_dist) in paths.iter().copied() {
                let seen_mask = 1 << to;
                if seen & seen_mask == 0 {
                    work_queue.insert((dist + to_dist, seen | seen_mask, to));
                }
            }
        }

        0
    }
}

impl Runner for Day24 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for (y, line) in lines.iter().enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                if c.is_ascii_digit() {
                    let n = c as u8 - b'0';
                    self.locations.insert(n, (x, y));
                }
                row.push(c);
            }

            self.grid.push(row);
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

impl Day24 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.find_shortest_path(false).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self.find_shortest_path(true).into())
    }
}
