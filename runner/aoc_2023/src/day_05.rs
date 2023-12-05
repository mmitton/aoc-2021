use std::collections::{BTreeMap, VecDeque};

#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Debug)]
pub enum RunnerError {}

impl From<RunnerError> for Error {
    fn from(e: RunnerError) -> Self {
        Self::Runner(format!("{e:?}"))
    }
}

#[derive(Default)]
pub struct Day05 {
    seeds: Vec<usize>,
    map: BTreeMap<Type, BTreeMap<Type, Vec<Map>>>,
}

impl Day05 {
    pub fn new() -> Self {
        Self::default()
    }

    fn find_location(&self, start: usize, len: usize) -> usize {
        use std::ops::Range;

        let mut work: VecDeque<(Type, Range<usize>)> = VecDeque::new();
        let mut location = usize::MAX;
        work.push_front((Type::Seed, start..start + len));
        while let Some((from_typ, seed_range)) = work.pop_front() {
            if let Some(from) = self.map.get(&from_typ) {
                for (to, map) in from.iter() {
                    let mut pool = vec![seed_range.clone()];
                    macro_rules! new_work {
                        ($typ:expr, $dest:expr) => {{
                            let new_work = ($typ, $dest);
                            if (new_work.0 != Type::Location) {
                                work.push_back(new_work);
                            } else if new_work.1.start < location {
                                location = new_work.1.start;
                            }
                        }};
                    }
                    for map in map.iter() {
                        let map_src = map.src..map.src + map.len;
                        let map_dest = map.dest..map.dest + map.len;
                        let mut i = 0;
                        while i < pool.len() {
                            let cur_pool = &mut pool[i];
                            let start = map_src.start.max(cur_pool.start);
                            let end = map_src.end.min(cur_pool.end);
                            if end > start {
                                let offset = start - map_src.start;
                                let dest =
                                    map_dest.start + offset..map_dest.start + offset + end - start;
                                new_work!(*to, dest.clone());
                                if *cur_pool == (start..end) {
                                    pool.remove(i);
                                    continue;
                                } else if cur_pool.start < start && cur_pool.end > end {
                                    let new_pool = cur_pool.start..start;
                                    let new_pool2 = end..cur_pool.end;
                                    *cur_pool = new_pool;
                                    pool.push(new_pool2);
                                } else if cur_pool.start < start {
                                    let new_pool = cur_pool.start..start;
                                    *cur_pool = new_pool;
                                } else if cur_pool.end > end {
                                    let new_pool = end..cur_pool.end;
                                    *cur_pool = new_pool;
                                } else {
                                    unreachable!();
                                }
                            }
                            i += 1;
                        }
                    }
                    for new in pool {
                        new_work!(*to, new);
                    }
                }
            }
        }

        location
    }
}

impl Runner for Day05 {
    fn parse(&mut self, path: &str) -> Result<(), Error> {
        let lines = Lines::from_path(path, LinesOpt::TRIM | LinesOpt::REMOVE_EMPTY)?;
        let lines: Vec<&str> = lines.iter().collect();
        self.seeds.extend(
            lines[0]
                .split_whitespace()
                .skip(1)
                .map(|s| s.parse::<usize>().unwrap()),
        );

        let mut from = Type::None;
        let mut to = Type::None;
        for line in lines.iter().skip(1) {
            if let Some(line) = line.strip_suffix(" map:") {
                let parts = line.split('-').collect::<Vec<&str>>();
                assert_eq!(parts.len(), 3);
                from = parts[0].into();
                to = parts[2].into();
            } else {
                let parts = line.split_whitespace().collect::<Vec<&str>>();
                assert_eq!(parts.len(), 3);
                let dest: usize = parts[0].parse().unwrap();
                let src: usize = parts[1].parse().unwrap();
                let len: usize = parts[2].parse().unwrap();
                assert_ne!(from, Type::None);
                assert_ne!(to, Type::None);
                let map = self.map.entry(from).or_default().entry(to).or_default();
                map.push(Map { dest, src, len });
            }
        }

        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        let mut best_location = usize::MAX;
        for seed in self.seeds.iter() {
            let location = self.find_location(*seed, 1);
            println!("{seed} => Location {location}");
            if location < best_location {
                best_location = location;
            }
        }
        Ok(best_location.into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut best_location = usize::MAX;
        for seed in self.seeds.chunks(2) {
            let location = self.find_location(seed[0], seed[1]);
            println!("{seed:?} => Location {location}");
            if location < best_location {
                best_location = location;
            }
        }
        Ok(best_location.into())
    }
}

#[derive(Debug)]
struct Map {
    dest: usize,
    src: usize,
    len: usize,
}

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
enum Type {
    None,
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl From<&str> for Type {
    fn from(value: &str) -> Self {
        match value {
            "seed" => Self::Seed,
            "soil" => Self::Soil,
            "fertilizer" => Self::Fertilizer,
            "water" => Self::Water,
            "light" => Self::Light,
            "temperature" => Self::Temperature,
            "humidity" => Self::Humidity,
            "location" => Self::Location,
            _ => unreachable!("Unknown Type: '{value}'"),
        }
    }
}
