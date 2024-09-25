#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};
use std::collections::{BTreeMap, HashSet, VecDeque};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum Tile {
    Path,
    Forest,
    Slope(i8, i8),
}

impl TryFrom<char> for Tile {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Tile::Path),
            '#' => Ok(Tile::Forest),
            '^' => Ok(Tile::Slope(0, -1)),
            'v' => Ok(Tile::Slope(0, 1)),
            '<' => Ok(Tile::Slope(-1, 0)),
            '>' => Ok(Tile::Slope(1, 0)),
            _ => Err(Error::InvalidInput(format!("Tile: '{value}'"))),
        }
    }
}

pub struct Day23 {
    map: Vec<Vec<Tile>>,
    paths: Vec<Vec<(usize, usize)>>,
}

impl Day23 {
    pub fn new() -> Self {
        Self {
            map: Vec::new(),
            paths: Vec::new(),
        }
    }

    fn make_paths(&mut self, icy: bool) {
        let mut intersections: Vec<(usize, usize)> = Vec::new();
        intersections.push((1, 0));
        intersections.push((self.map[0].len() - 2, self.map.len() - 1));

        for y in 1..self.map.len() - 1 {
            for x in 1..self.map[y].len() - 1 {
                if self.map[y][x] == Tile::Forest {
                    continue;
                }
                let mut outbound = 0;
                for dir in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
                    let x = (x as isize + dir.0) as usize;
                    let y = (y as isize + dir.1) as usize;
                    if self.map[y][x] != Tile::Forest {
                        outbound += 1;
                    }
                }
                if outbound > 2 {
                    intersections.push((x, y));
                }
            }
        }

        let mut work = VecDeque::new();
        let mut seen = HashSet::new();
        #[allow(clippy::type_complexity)]
        let mut paths: BTreeMap<(usize, usize), Vec<((usize, usize), usize)>> = BTreeMap::new();
        for i in 0..intersections.len() {
            // Walk from i to next intersections
            let (x, y) = intersections[i];
            work.clear();
            seen.clear();
            seen.insert((x, y));
            macro_rules! add_step {
                ($steps:expr, $x:expr, $y:expr, $dx:expr, $dy:expr) => {{
                    match self.map[$y][$x] {
                        Tile::Path => {
                            work.push_back(($steps, $x, $y));
                            seen.insert(($x, $y));
                        }
                        Tile::Forest => {}
                        Tile::Slope(dx, dy) => {
                            if !icy || (dx == $dx as i8 && dy == $dy as i8) {
                                work.push_back(($steps, $x, $y));
                                seen.insert(($x, $y));
                            }
                        }
                    }
                }};
            }
            for dir in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
                let x = (x as isize + dir.0) as usize;
                let y = (y as isize + dir.1) as usize;
                if y >= self.map.len() {
                    continue;
                }
                add_step!(1, x, y, dir.0, dir.1);
            }

            while let Some((steps, x, y)) = work.pop_front() {
                if intersections.contains(&(x, y)) {
                    let paths = paths.entry(intersections[i]).or_default();
                    paths.push(((x, y), steps));
                } else {
                    for dir in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
                        let x = (x as isize + dir.0) as usize;
                        let y = (y as isize + dir.1) as usize;
                        if y >= self.map.len() {
                            continue;
                        }
                        if seen.contains(&(x, y)) {
                            continue;
                        }
                        add_step!(steps + 1, x, y, dir.0, dir.1);
                    }
                }
            }
        }

        self.paths = vec![Vec::new(); intersections.len()];
        for (from, to) in paths.iter() {
            let from = intersections.iter().position(|p| p == from).unwrap();
            for (to, steps) in to.iter() {
                let to = intersections.iter().position(|p| p == to).unwrap();
                self.paths[from].push((to, *steps));
            }
            self.paths[from].sort();
        }
    }

    fn longest_path(&self) -> usize {
        fn find_paths(
            paths: &[Vec<(usize, usize)>],
            at: usize,
            steps: usize,
            visited: u64,
        ) -> usize {
            let mut longest = 0;
            for &(next, next_steps) in paths[at].iter() {
                if visited & (1 << next) != 0 {
                    continue;
                }
                if next == 1 {
                    return longest.max(steps + next_steps);
                } else {
                    let visited = visited | (1 << next);
                    longest = longest.max(find_paths(paths, next, steps + next_steps, visited));
                }
            }

            longest
        }

        find_paths(&self.paths, 0, 0, 1)
    }
}

impl Runner for Day23 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        for line in Lines::from_bufread(file, LinesOpt::RAW)?.iter() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(c.try_into()?);
            }
            self.map.push(row);
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        self.make_paths(true);
        Ok(self.longest_path().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        self.make_paths(false);
        Ok(self.longest_path().into())
    }
}
