#[allow(unused_imports)]
use helper::{
    print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, Point, RunOutput, Runner,
};
use std::collections::BTreeSet;

fn erosion_level(
    geologic_index: &mut Vec<Vec<usize>>,
    coord: Point<usize>,
    target: Point<usize>,
    depth: usize,
) -> usize {
    while geologic_index.len() <= coord.y {
        geologic_index.push(Vec::new());
    }
    while geologic_index[coord.y].len() <= coord.x {
        geologic_index[coord.y].push(usize::MAX);
    }

    if geologic_index[coord.y][coord.x] == usize::MAX {
        if coord == Point::new(0, 0) || coord == target {
            geologic_index[coord.y][coord.x] = 0;
        } else if coord.y == 0 {
            geologic_index[coord.y][coord.x] = coord.x * 16807;
        } else if coord.x == 0 {
            geologic_index[coord.y][coord.x] = coord.y * 48271;
        } else {
            let a = erosion_level(
                geologic_index,
                Point::new(coord.x - 1, coord.y),
                target,
                depth,
            );
            let b = erosion_level(
                geologic_index,
                Point::new(coord.x, coord.y - 1),
                target,
                depth,
            );
            geologic_index[coord.y][coord.x] = a * b;
        }
    }
    (geologic_index[coord.y][coord.x] + depth) % 20183
}

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
enum Tool {
    None,
    Torch,
    Gear,
}

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
enum Terrain {
    Rocky,
    Wet,
    Narrow,
}

#[derive(Default)]
pub struct Day22 {
    depth: usize,
    target: Point<usize>,
}

impl Day22 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day22 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            if let Some(depth) = line.strip_prefix("depth: ") {
                self.depth = depth.parse()?;
                continue;
            } else if let Some(target) = line.strip_prefix("target: ") {
                if let Some((x, y)) = target.split_once(',') {
                    self.target.x = x.parse()?;
                    self.target.y = y.parse()?;
                    continue;
                }
            }
            return Err(Error::InvalidInput(line.into()));
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Err(Error::Unsolved)
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let width = self.target.x + 6;
        let height = self.target.y + 6;

        let mut geologic_index = Vec::new();

        let mut queue: BTreeSet<(usize, Tool, Point<usize>)> = BTreeSet::new();
        let mut seen: HashMap<Point<usize>, usize> = HashMap::default();

        let mut tools = HashMap::default();
        tools.insert((Terrain::Rocky, Terrain::Rocky, Tool::Gear), Tool::Gear);
        tools.insert((Terrain::Rocky, Terrain::Rocky, Tool::Torch), Tool::Torch);
        tools.insert((Terrain::Rocky, Terrain::Wet, Tool::Gear), Tool::Gear);
        tools.insert((Terrain::Rocky, Terrain::Wet, Tool::Torch), Tool::Gear);
        tools.insert((Terrain::Rocky, Terrain::Narrow, Tool::Gear), Tool::Torch);
        tools.insert((Terrain::Rocky, Terrain::Narrow, Tool::Torch), Tool::Torch);

        tools.insert((Terrain::Wet, Terrain::Wet, Tool::Gear), Tool::Gear);
        tools.insert((Terrain::Wet, Terrain::Wet, Tool::None), Tool::None);
        tools.insert((Terrain::Wet, Terrain::Rocky, Tool::Gear), Tool::Gear);
        tools.insert((Terrain::Wet, Terrain::Rocky, Tool::None), Tool::Gear);
        tools.insert((Terrain::Wet, Terrain::Narrow, Tool::Gear), Tool::None);
        tools.insert((Terrain::Wet, Terrain::Narrow, Tool::None), Tool::None);

        tools.insert((Terrain::Narrow, Terrain::Narrow, Tool::None), Tool::None);
        tools.insert((Terrain::Narrow, Terrain::Narrow, Tool::Torch), Tool::Torch);
        tools.insert((Terrain::Narrow, Terrain::Rocky, Tool::None), Tool::Torch);
        tools.insert((Terrain::Narrow, Terrain::Rocky, Tool::Torch), Tool::Torch);
        tools.insert((Terrain::Narrow, Terrain::Wet, Tool::None), Tool::None);
        tools.insert((Terrain::Narrow, Terrain::Wet, Tool::Torch), Tool::None);

        assert!(tools.len() == 6 * 3);

        queue.insert((0, Tool::Torch, Point::new(0, 0)));
        seen.insert(Point::new(0, 0), 0);

        while queue.len() > 0 {
            let (time, tool, coord) = queue.pop_first().unwrap();
            // println!("{} {:?} {:?}", time, tool, coord);

            if coord == self.target {
                if tool != Tool::Torch {
                    queue.insert((time + 7, Tool::Torch, coord));
                    continue;
                }
                println!("Can get to the target in {} minutes", time);
                return Ok(time.into());
            }

            if let Some(seen_time) = seen.get(&coord) {
                if *seen_time + 10 < time {
                    continue;
                }
            } else {
                seen.insert(coord, time);
            }

            let cur_terrain: Terrain =
                match erosion_level(&mut geologic_index, coord, self.target, self.depth) % 3 {
                    0 => Terrain::Rocky,
                    1 => Terrain::Wet,
                    2 => Terrain::Narrow,
                    _ => unreachable!(),
                };

            for dir in 0..4 {
                let new_coord = match dir {
                    0 => {
                        if coord.y == 0 {
                            continue;
                        }
                        Point::new(coord.x, coord.y - 1)
                    }
                    1 => {
                        if coord.x == 0 {
                            continue;
                        }
                        Point::new(coord.x - 1, coord.y)
                    }
                    2 => Point::new(coord.x, coord.y + 1),
                    3 => Point::new(coord.x + 1, coord.y),
                    _ => unreachable!(),
                };

                let new_terrain: Terrain = match erosion_level(
                    &mut geologic_index,
                    new_coord,
                    self.target,
                    self.depth,
                ) % 3
                {
                    0 => Terrain::Rocky,
                    1 => Terrain::Wet,
                    2 => Terrain::Narrow,
                    _ => unreachable!(),
                };

                let new_tool = tools.get(&(cur_terrain, new_terrain, tool)).unwrap();
                let new_tool = *new_tool;
                let new_time = if new_tool == tool { time + 1 } else { time + 8 };
                // println!(
                //     "{:?} {:?} {:?} {} => {:?} {:?} {:?} {}",
                //     coord, cur_terrain, tool, time, new_coord, new_terrain, new_tool, new_time
                // );
                queue.insert((new_time, new_tool, new_coord));
            }
        }
        Err(Error::Unsolved)
    }
}
