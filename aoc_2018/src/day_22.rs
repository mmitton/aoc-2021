#[allow(unused_imports)]
use helper::{
    print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, Point, RunOutput, Runner,
};
use std::collections::BTreeMap;

fn add_seen(seen: &mut Vec<Vec<usize>>, coord: Point<u16>, time: usize) -> bool {
    if seen.len() <= coord.y as usize {
        seen.resize(coord.y as usize + 1, Vec::new());
    }
    if seen[coord.y as usize].len() <= coord.x as usize {
        seen[coord.y as usize].resize(coord.x as usize + 1, usize::MAX);
    }

    let seen_time = seen[coord.y as usize][coord.x as usize];
    if seen_time == usize::MAX {
        seen[coord.y as usize][coord.x as usize] = time;
        true
    } else {
        seen_time > time
    }
}

fn erosion_level(
    geologic_index: &mut Vec<Vec<usize>>,
    coord: Point<u16>,
    target: Point<u16>,
    depth: usize,
) -> usize {
    if geologic_index.len() <= coord.y as usize {
        geologic_index.resize(coord.y as usize + 1, Vec::new());
    }
    if geologic_index[coord.y as usize].len() <= coord.x as usize {
        geologic_index[coord.y as usize].resize(coord.x as usize + 1, usize::MAX);
    }

    if geologic_index[coord.y as usize][coord.x as usize] == usize::MAX {
        if coord == Point::new(0, 0) || coord == target {
            geologic_index[coord.y as usize][coord.x as usize] = 0;
        } else if coord.y == 0 {
            geologic_index[coord.y as usize][coord.x as usize] = coord.x as usize * 16807;
        } else if coord.x == 0 {
            geologic_index[coord.y as usize][coord.x as usize] = coord.y as usize * 48271;
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
            geologic_index[coord.y as usize][coord.x as usize] = a * b;
        }
    }
    (geologic_index[coord.y as usize][coord.x as usize] + depth) % 20183
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
    target: Point<u16>,
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
        let mut geologic_index = Vec::new();
        let mut answer = 0;
        for y in 0..=self.target.y {
            for x in 0..=self.target.x {
                let erosion_level = erosion_level(
                    &mut geologic_index,
                    Point::new(x, y),
                    self.target,
                    self.depth,
                );
                answer += erosion_level % 3;
            }
        }

        Ok(answer.into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut geologic_index = Vec::new();
        let mut seen_none: Vec<Vec<usize>> = Vec::new();
        let mut seen_torch: Vec<Vec<usize>> = Vec::new();
        let mut seen_gear: Vec<Vec<usize>> = Vec::new();

        let mut queue: BTreeMap<usize, HashSet<(Tool, Point<u16>)>> = BTreeMap::new();

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

        queue
            .entry(0)
            .or_default()
            .insert((Tool::Torch, Point::new(0, 0)));

        while let Some((time, time_queue)) = queue.pop_first() {
            for (tool, coord) in time_queue {
                if coord == self.target {
                    if tool != Tool::Torch {
                        queue
                            .entry(time + 7)
                            .or_default()
                            .insert((Tool::Torch, coord));
                        continue;
                    }
                    return Ok(time.into());
                }

                let seen = match tool {
                    Tool::None => &mut seen_none,
                    Tool::Gear => &mut seen_gear,
                    Tool::Torch => &mut seen_torch,
                };
                if !add_seen(seen, coord, time) {
                    continue;
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
                    queue
                        .entry(new_time)
                        .or_default()
                        .insert((new_tool, new_coord));
                }
            }
        }
        Err(Error::Unsolved)
    }
}
