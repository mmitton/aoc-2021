#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};
use std::collections::{BTreeMap, VecDeque};

#[derive(Debug, Clone, Default)]
struct Room {
    flow_rate: usize,
    exits: Vec<usize>,
    paths: Vec<(usize, usize)>,
}

pub struct Day16 {
    rooms: Vec<Room>,
    names: Vec<String>,
}

impl Day16 {
    pub fn new() -> Self {
        Self {
            rooms: Vec::new(),
            names: Vec::new(),
        }
    }

    fn _print(&self) {
        for i in 0..self.rooms.len() {
            println!(
                "{} with flow rate of {} =>",
                self.names[i], self.rooms[i].flow_rate
            );
            for path in self.rooms[i].paths.iter() {
                println!("   {} in {} steps", self.names[path.0], path.1);
            }
        }
    }

    fn calc_paths(&mut self) {
        for i in 0..self.rooms.len() {
            if i != 0 && self.rooms[i].flow_rate == 0 {
                continue;
            }
            let mut seen = vec![false; self.rooms.len()];
            seen[i] = true;

            let mut work = VecDeque::new();
            work.push_front((i, 0));

            while let Some((j, dur)) = work.pop_front() {
                if j != i && self.rooms[j].flow_rate != 0 {
                    self.rooms[i].paths.push((j, dur + 1));
                }
                for exit in self.rooms[j].exits.iter() {
                    if !seen[*exit] {
                        seen[*exit] = true;
                        work.push_back((*exit, dur + 1));
                    }
                }
            }
        }
    }

    fn calc_path_lists(&self, minutes: usize) -> Vec<(usize, usize)> {
        let mut path_lists = BTreeMap::new();

        let mut work = Vec::new();
        work.push((minutes, 0, 0, 1));

        while let Some((minutes_left, flow, at, seen)) = work.pop() {
            let path = path_lists.entry(seen).or_insert((flow, seen));
            path.0 = path.0.max(flow);

            for (room, dur) in self.rooms[at].paths.iter() {
                if minutes_left < *dur || seen & (1 << room) != 0 {
                    continue;
                }

                let minutes_left = minutes_left - dur;
                let seen = seen | (1 << room);
                let flow = flow + (minutes_left * self.rooms[*room].flow_rate);
                work.push((minutes_left, flow, *room, seen));
            }
        }

        path_lists.values().copied().collect()
    }
}

impl Runner for Day16 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;

        macro_rules! get_room_idx {
            ($name:expr) => {
                if let Some(idx) = self.names.iter().position(|s| *s == $name) {
                    idx
                } else {
                    self.names.push($name.to_string());
                    self.rooms.push(Room::default());
                    self.names.len() - 1
                }
            };
        }

        get_room_idx!("AA");

        for line in lines.iter() {
            let name = &line[6..8];
            let idx = get_room_idx!(name);
            let (flow, tunnels) = line.split_once(';').unwrap();
            let (_, flow) = flow.split_once('=').unwrap();
            self.rooms[idx].flow_rate = flow.parse()?;

            for exit in tunnels.split(", ") {
                let exit_name = if exit.len() > 2 {
                    &exit[exit.len() - 2..]
                } else {
                    exit
                };
                let exit_idx = get_room_idx!(exit_name);
                self.rooms[idx].exits.push(exit_idx);
            }
        }
        self.calc_paths();
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        let paths = self.calc_path_lists(30);
        Ok(paths.iter().fold(0, |acc, p| acc.max(p.0)).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let paths = self.calc_path_lists(26);
        let mut best = 0;
        for i in 0..paths.len() {
            for j in i + 1..paths.len() {
                if paths[i].1 & paths[j].1 == 1 {
                    best = best.max(paths[i].0 + paths[j].0);
                }
            }
        }
        Ok(best.into())
    }
}
