#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, PriorityVec, RunOutput, Runner};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::{Debug, Display},
};

pub struct Day18 {
    map: Vec<Vec<Tile>>,
    robots: Vec<Coord>,
    all_keys: Keys,
    keys: Vec<(Coord, u8)>,
    paths: [Vec<Path>; 256],
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Keys(u32);

impl Debug for Keys {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:026b}", self.0)
    }
}

impl From<u32> for Keys {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl std::ops::Not for Keys {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl std::ops::BitAnd for Keys {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitOr for Keys {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for Keys {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Coord(usize, usize);

impl Coord {
    fn neighbors(&self) -> [Self; 4] {
        [
            Self(self.0 - 1, self.1),
            Self(self.0 + 1, self.1),
            Self(self.0, self.1 - 1),
            Self(self.0, self.1 + 1),
        ]
    }
}

impl From<(usize, usize)> for Coord {
    fn from(value: (usize, usize)) -> Self {
        Self(value.0, value.1)
    }
}

impl Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:02}, {:02})", self.0, self.1)
    }
}

#[derive(Debug)]
enum Tile {
    Hall,
    Wall,
    Key(u8),
    Door(u8),
}

#[derive(Debug)]
struct Path {
    to: u8,
    keys_acquired: Keys,
    keys_required: Keys,
    steps: usize,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Hall => write!(f, "."),
            Self::Wall => write!(f, "#"),
            Self::Key(k) => write!(f, "{}", (k + b'a') as char),
            Self::Door(d) => write!(f, "{}", (d + b'A') as char),
        }
    }
}

impl Day18 {
    pub fn new() -> Self {
        Self {
            map: Vec::new(),
            robots: Vec::new(),
            all_keys: 0u32.into(),
            keys: Vec::new(),
            paths: std::array::from_fn(|_| Vec::new()),
        }
    }

    fn generate_paths(&mut self) {
        for (idx, robot) in self.robots.iter().enumerate() {
            let idx = idx + 26;
            self.paths[idx].extend(self.find_paths_from(*robot));
        }
        for key in self.keys.iter() {
            self.paths[key.1 as usize].extend(self.find_paths_from(key.0));
        }
    }

    fn find_paths_from(&self, from: Coord) -> Vec<Path> {
        let mut best: BTreeMap<Coord, Vec<(Keys, usize)>> = BTreeMap::new();
        let mut paths = Vec::new();

        let mut steps = 0;
        let mut next: Vec<(Coord, Keys, Keys)> = Vec::new();
        let mut cur: Vec<(Coord, Keys, Keys)> = Vec::new();

        best.insert(from, vec![(0u32.into(), 0)]);
        for neighbor in from.neighbors() {
            if matches!(self.map[neighbor.1][neighbor.0], Tile::Wall) {
                continue;
            }
            next.push((neighbor, 0u32.into(), 0u32.into()));
        }

        while !next.is_empty() {
            // Set current work from next work (and clear out next since cur is currently empty),
            // and update step to next step count
            std::mem::swap(&mut cur, &mut next);
            steps += 1;

            'coord: for (coord, mut keys_required, mut keys_acquired) in cur.drain(..) {
                // Check to see if there is a better way to get here with the current key set
                let best_vec = best.entry(coord).or_default();

                for (path_keys, _path_steps) in best_vec.iter() {
                    if *path_keys & !keys_required == Keys(0) {
                        continue 'coord;
                    }
                }

                match self.map[coord.1][coord.0] {
                    Tile::Hall => { /* Walk down the hall */ }
                    Tile::Wall => {
                        /* Do nothing */
                        continue;
                    }
                    Tile::Door(d) => {
                        /* Add key needed to the list */
                        keys_required |= (1 << d).into();
                    }
                    Tile::Key(k) => {
                        /* Excellent!  Found a key! */
                        let keys: Keys = (1 << k).into();
                        paths.push(Path {
                            to: k,
                            keys_acquired: keys_acquired | keys,
                            keys_required,
                            steps,
                        });
                        keys_acquired |= keys;
                    }
                }

                best_vec.push((keys_required, steps));
                for neighbor in coord.neighbors() {
                    if matches!(self.map[neighbor.1][neighbor.0], Tile::Wall) {
                        continue;
                    }
                    next.push((neighbor, keys_required, keys_acquired));
                }
            }
        }
        paths.sort_by_key(|p| p.steps);
        paths
    }

    fn solve(&mut self) -> usize {
        println!("{:?}", self.robots);
        for (y, row) in self.map.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if self.robots.contains(&Coord(x, y)) {
                    print!("@");
                } else {
                    print!("{tile}");
                }
            }
            println!();
        }

        self.generate_paths();
        println!(
            "Found {} paths",
            self.paths.iter().map(|paths| paths.len()).sum::<usize>()
        );

        #[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
        struct State {
            steps: usize,
            robots: [u8; 4],
            keys_acquired: Keys,
        }

        let mut work: BTreeSet<State> = BTreeSet::new();
        // let mut work: PriorityVec<usize, State, 32> = PriorityVec::new();
        let mut initial_robots = [255u8; 4];
        initial_robots
            .iter_mut()
            .enumerate()
            .for_each(|(i, v)| *v = 26 + i as u8);

        work.insert(State {
            steps: 0,
            keys_acquired: Keys(0),
            robots: initial_robots,
        });

        // let mut best: HashMap<u64, usize> = HashMap::new();
        let mut best_steps = !0;

        while let Some(state) = work.pop_first() {
            if state.steps > best_steps {
                break;
            }
            for (i, robot) in state.robots.iter().enumerate() {
                if *robot == 255 {
                    break;
                }
                for path in self.paths[*robot as usize].iter() {
                    // Skip this path if we already have this key
                    if (1 << path.to) & state.keys_acquired.0 != 0 {
                        continue;
                    }

                    // Skip path if we have not unlock all doors along this path
                    if path.keys_required & state.keys_acquired != path.keys_required {
                        continue;
                    }

                    // Skip path if there is a key along the path that we have not already picked up
                    if path.keys_acquired & !state.keys_acquired != (1 << path.to).into() {
                        continue;
                    }

                    let path_steps = state.steps + path.steps;
                    if path_steps >= best_steps {
                        continue;
                    }
                    let mut new_state = state.clone();
                    new_state.robots[i] = path.to;
                    new_state.keys_acquired |= path.keys_acquired;
                    new_state.steps = path_steps;
                    if new_state.keys_acquired == self.all_keys {
                        best_steps = best_steps.min(path_steps);
                    }
                    if new_state.steps < best_steps {
                        work.insert(new_state);
                    }
                }
            }
        }
        best_steps
    }
}

impl Runner for Day18 {
    fn parse(&mut self, path: &str, _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_path(path, LinesOpt::RAW)?;
        for (y, line) in lines.iter().enumerate() {
            let row: Vec<Tile> = line
                .chars()
                .enumerate()
                .map(|(x, c)| match c {
                    'A'..='Z' => Tile::Door(c as u8 - b'A'),
                    'a'..='z' => {
                        let k = c as u8 - b'a';
                        self.keys.push(((x, y).into(), k));
                        self.all_keys |= (1 << k).into();
                        Tile::Key(k)
                    }
                    '.' => Tile::Hall,
                    '#' => Tile::Wall,
                    '@' => {
                        self.robots.push((x, y).into());
                        Tile::Hall
                    }
                    _ => unreachable!(),
                })
                .collect();
            self.map.push(row);
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.solve().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let robot = self.robots.pop().unwrap();
        self.map[robot.1 - 1][robot.0] = Tile::Wall;
        self.map[robot.1 + 1][robot.0] = Tile::Wall;
        self.map[robot.1][robot.0] = Tile::Wall;
        self.map[robot.1][robot.0 - 1] = Tile::Wall;
        self.map[robot.1][robot.0 + 1] = Tile::Wall;
        self.robots.push(Coord(robot.0 - 1, robot.1 - 1));
        self.robots.push(Coord(robot.0 - 1, robot.1 + 1));
        self.robots.push(Coord(robot.0 + 1, robot.1 - 1));
        self.robots.push(Coord(robot.0 + 1, robot.1 + 1));
        Ok(self.solve().into())
    }
}
