#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};
use std::collections::BTreeMap;

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
enum Path {
    Forward(usize),
    Left,
    Right,
}

#[derive(Copy, Clone, Debug, Default, PartialOrd, Ord, PartialEq, Eq)]
enum Dir {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

impl From<Dir> for usize {
    fn from(dir: Dir) -> Self {
        match dir {
            Dir::Right => 0,
            Dir::Down => 1,
            Dir::Left => 2,
            Dir::Up => 3,
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialOrd, Ord, PartialEq, Eq)]
struct Warp {
    from: (usize, usize),
    to: (usize, usize),
    dir: Dir,
}

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
enum Tile {
    Empty,
    Walkable,
    Block,
    Warp([Warp; 2]),
}

#[derive(Copy, Clone, Debug)]
struct Tail {
    p: (usize, usize),
    d: (isize, isize),
    e: (usize, usize),
}

pub struct Day22 {
    map: Vec<Vec<Tile>>,
    path: Vec<Path>,
    start: (usize, usize),
}

impl Day22 {
    pub fn new() -> Self {
        Self {
            map: Vec::new(),
            path: Vec::new(),
            start: (0, 1),
        }
    }

    fn add_warp(&mut self, at: (usize, usize), from: (usize, usize), to: (usize, usize), dir: Dir) {
        let at_tile = &mut self.map[at.1][at.0];
        match at_tile {
            Tile::Empty => {
                *at_tile = Tile::Warp([Warp { from, to, dir }, Warp::default()]);
            }
            Tile::Warp(warps) => {
                if (warps[0].from == from && warps[0].to == to && warps[0].dir == dir)
                    || (warps[1].from == from && warps[1].to == to && warps[1].dir == dir)
                {
                    return;
                }
                assert_eq!(
                    warps[1].from,
                    (0, 0),
                    "add_warp({at:?}, {from:?}, {to:?}, {dir:?}) {warps:?}"
                );
                warps[1].from = from;
                warps[1].to = to;
                warps[1].dir = dir;
            }
            _ => unreachable!("add_warp({at:?}, {from:?}, {to:?}, {dir:?})  found {at_tile:?}"),
        }
    }

    fn get_next(&self, pos: (usize, usize), dir: Dir) -> Option<((usize, usize), Dir)> {
        let next_pos = match dir {
            Dir::Up => (pos.0, pos.1 - 1),
            Dir::Down => (pos.0, pos.1 + 1),
            Dir::Left => (pos.0 - 1, pos.1),
            Dir::Right => (pos.0 + 1, pos.1),
        };
        match self.map[next_pos.1][next_pos.0] {
            Tile::Walkable => Some((next_pos, dir)),
            Tile::Block => None,
            Tile::Warp(warps) => {
                for warp in warps.iter() {
                    if warp.from == pos {
                        match self.map[warp.to.1][warp.to.0] {
                            Tile::Walkable => return Some((warp.to, warp.dir)),
                            Tile::Block => return None,
                            _ => unreachable!(),
                        }
                    }
                }
                unreachable!();
            }
            Tile::Empty => unreachable!(),
        }
    }

    fn walk(&self) -> usize {
        let mut pos = self.start;
        let mut dir = Dir::Right;

        for p in self.path.iter() {
            match p {
                Path::Left => {
                    dir = match dir {
                        Dir::Up => Dir::Left,
                        Dir::Left => Dir::Down,
                        Dir::Down => Dir::Right,
                        Dir::Right => Dir::Up,
                    }
                }
                Path::Right => {
                    dir = match dir {
                        Dir::Up => Dir::Right,
                        Dir::Left => Dir::Up,
                        Dir::Down => Dir::Left,
                        Dir::Right => Dir::Down,
                    }
                }
                Path::Forward(tiles) => {
                    for _ in 0..*tiles {
                        if let Some((next_pos, next_dir)) = self.get_next(pos, dir) {
                            pos = next_pos;
                            dir = next_dir;
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        let row = pos.1 * 1000;
        let col = pos.0 * 4;
        let dir_usize: usize = dir.into();
        row + col + dir_usize
    }

    fn zip(
        &mut self,
        tails: &mut BTreeMap<(usize, usize), Tail>,
        c1: ((usize, usize), (isize, isize), (usize, usize)),
        c2: ((usize, usize), (isize, isize), (usize, usize)),
    ) {
        let mut p1 = c1.0;
        let mut d1 = c1.1;
        let mut e1 = c1.2;

        let mut p2 = c2.0;
        let mut d2 = c2.1;
        let mut e2 = c2.2;

        let mut t1 = 0;
        let mut t2 = 0;

        while t1 == 0 || t2 == 0 {
            fn calc_dir(p: (usize, usize), e: (usize, usize)) -> Dir {
                if (p.0 - 1, p.1) == e {
                    Dir::Right
                } else if (p.0 + 1, p.1) == e {
                    Dir::Left
                } else if (p.0, p.1 - 1) == e {
                    Dir::Down
                } else if (p.0, p.1 + 1) == e {
                    Dir::Up
                } else {
                    unreachable!("{p:?} {e:?}");
                }
            }

            self.add_warp(e1, p1, p2, calc_dir(p2, e2));
            self.add_warp(e2, p2, p1, calc_dir(p1, e1));

            macro_rules! next {
                ($p:expr, $d:expr) => {
                    (
                        ($p.0 as isize + $d.0 as isize) as usize,
                        ($p.1 as isize + $d.1 as isize) as usize,
                    )
                };
            }
            macro_rules! walk {
                ($p:expr, $e:expr, $d:expr, $t:expr) => {{
                    let new_p = next!($p, $d);
                    if !matches!(self.map[new_p.1][new_p.0], Tile::Walkable | Tile::Block) {
                        // Turn!
                        if $d.0 != 0 {
                            if $e.1 == $p.1 - 1 {
                                $d = (0, 1);
                            } else {
                                $d = (0, -1);
                            }
                        } else {
                            if $e.0 == $p.0 - 1 {
                                $d = (1, 0);
                            } else {
                                $d = (-1, 0);
                            }
                        }

                        $t += 1;
                        $e = new_p;
                    } else {
                        $e = next!($e, $d);
                        $p = new_p;
                    }
                }};
            }

            // Move p1
            walk!(p1, e1, d1, t1);

            // Move p2
            walk!(p2, e2, d2, t2);
        }

        if let std::collections::btree_map::Entry::Vacant(e) = tails.entry(p1) {
            e.insert(Tail {
                p: p1,
                d: d1,
                e: e1,
            });
        } else {
            tails.remove(&p1);
        }
        if let std::collections::btree_map::Entry::Vacant(e) = tails.entry(p2) {
            e.insert(Tail {
                p: p2,
                d: d2,
                e: e2,
            });
        } else {
            tails.remove(&p2);
        }
    }

    fn zip_tails(&mut self, tails: &mut BTreeMap<(usize, usize), Tail>) {
        fn find_neighbor(map: &[Vec<Tile>], p: (usize, usize)) -> Option<Warp> {
            macro_rules! check {
                ($x:expr, $y:expr) => {
                    if let Tile::Warp(warps) = &map[$y][$x] {
                        assert_eq!(warps[1].from, (0, 0));
                        return Some(warps[0]);
                    }
                };
            }

            check!(p.0 - 1, p.1);
            check!(p.0 + 1, p.1);
            check!(p.0, p.1 - 1);
            check!(p.0, p.1 + 1);
            None
        }

        let mut values: Vec<Tail> = Vec::new();
        for tail in tails.values() {
            values.push(*tail);
        }
        for i in 0..values.len() {
            let i_neighbor = find_neighbor(&self.map, values[i].p);
            if let Some(i_neighbor) = i_neighbor {
                for j in i + 1..values.len() {
                    // Check to see if values[i] is next to values[j]
                    let j_neighbor = find_neighbor(&self.map, values[j].p);
                    if let Some(j_neighbor) = j_neighbor {
                        if i_neighbor.to == j_neighbor.to {
                            let p1 = values[i].p;
                            let d1 = values[i].d;
                            let e1 = values[i].e;
                            let p2 = values[j].p;
                            let d2 = values[j].d;
                            let e2 = values[j].e;
                            self.zip(tails, (p1, d1, e1), (p2, d2, e2));
                        }
                    }
                }
            }
        }
    }
}

impl Runner for Day22 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        let mut lines = lines.iter();
        let mut max_width = usize::MIN;
        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }
            let mut row: Vec<Tile> = line
                .chars()
                .map(|c| match c {
                    '.' => Tile::Walkable,
                    '#' => Tile::Block,
                    ' ' => Tile::Empty,
                    _ => unreachable!(),
                })
                .collect();
            row.insert(0, Tile::Empty);
            row.push(Tile::Empty);
            max_width = max_width.max(row.len());
            if self.map.is_empty() {
                self.map.push((0..row.len()).map(|_| Tile::Empty).collect());
                self.start.0 = row.iter().position(|t| *t == Tile::Walkable).unwrap();
            }
            self.map.push(row);
        }
        self.map
            .push((0..self.map[0].len()).map(|_| Tile::Empty).collect());
        self.map
            .iter_mut()
            .for_each(|row| row.resize(max_width, Tile::Empty));

        let dirs = lines.next().unwrap();
        assert_eq!(lines.next(), None);

        let mut num_start = 0;
        for (i, c) in dirs.chars().enumerate() {
            if let Some(dir) = match c {
                'L' => Some(Path::Left),
                'R' => Some(Path::Right),
                '0'..='9' => None,
                _ => unreachable!(),
            } {
                self.path.push(Path::Forward(dirs[num_start..i].parse()?));
                self.path.push(dir);
                num_start = i + 1;
            }
        }
        self.path.push(Path::Forward(dirs[num_start..].parse()?));

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

impl Day22 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        // Generate warps for part 1
        for x in 1..self.map[0].len() - 1 {
            let mut min = usize::MAX;
            let mut max = usize::MIN;
            for y in 1..self.map.len() - 1 {
                if matches!(self.map[y][x], Tile::Walkable | Tile::Block) {
                    min = min.min(y);
                    max = max.max(y);
                }
            }
            self.add_warp((x, min - 1), (x, min), (x, max), Dir::Up);
            self.add_warp((x, max + 1), (x, max), (x, min), Dir::Down);
        }
        for y in 1..self.map.len() - 1 {
            let mut min = usize::MAX;
            let mut max = usize::MIN;
            for x in 1..self.map[0].len() - 1 {
                if matches!(self.map[y][x], Tile::Walkable | Tile::Block) {
                    min = min.min(x);
                    max = max.max(x);
                }
            }
            self.add_warp((min - 1, y), (min, y), (max, y), Dir::Left);
            self.add_warp((max + 1, y), (max, y), (min, y), Dir::Right);
        }

        Ok(self.walk().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut tails: BTreeMap<(usize, usize), Tail> = BTreeMap::new();

        // Find the inside corners
        for y in 1..self.map.len() - 1 {
            for x in 1..self.map[0].len() - 1 {
                if matches!(self.map[y][x], Tile::Empty) {
                    if matches!(self.map[y + 1][x], Tile::Walkable | Tile::Block)
                        && matches!(self.map[y][x + 1], Tile::Walkable | Tile::Block)
                    {
                        let p1 = (x, y + 1);
                        let p2 = (x + 1, y);
                        let d1 = (-1, 0);
                        let d2 = (0, -1);
                        let e = (x, y);

                        self.zip(&mut tails, (p1, d1, e), (p2, d2, e));
                    }
                    if matches!(self.map[y + 1][x], Tile::Walkable | Tile::Block)
                        && matches!(self.map[y][x - 1], Tile::Walkable | Tile::Block)
                    {
                        let p1 = (x, y + 1);
                        let p2 = (x - 1, y);
                        let d1 = (1, 0);
                        let d2 = (0, -1);
                        let e = (x, y);

                        self.zip(&mut tails, (p1, d1, e), (p2, d2, e));
                    }
                    if matches!(self.map[y - 1][x], Tile::Walkable | Tile::Block)
                        && matches!(self.map[y][x + 1], Tile::Walkable | Tile::Block)
                    {
                        let p1 = (x, y - 1);
                        let p2 = (x + 1, y);
                        let d1 = (-1, 0);
                        let d2 = (0, 1);
                        let e = (x, y);

                        self.zip(&mut tails, (p1, d1, e), (p2, d2, e));
                    }
                    if matches!(self.map[y - 1][x], Tile::Walkable | Tile::Block)
                        && matches!(self.map[y][x - 1], Tile::Walkable | Tile::Block)
                    {
                        let p1 = (x, y - 1);
                        let p2 = (x - 1, y);
                        let d1 = (1, 0);
                        let d2 = (0, 1);
                        let e = (x, y);

                        self.zip(&mut tails, (p1, d1, e), (p2, d2, e));
                    }
                }
            }
        }
        self.zip_tails(&mut tails);

        Ok(self.walk().into())
    }
}
