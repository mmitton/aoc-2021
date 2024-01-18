#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};

use crate::intcode::{IntCode, State};
use std::{
    collections::{BTreeMap, HashSet, VecDeque},
    fmt::Debug,
};

#[derive(Default)]
struct Room {
    doors: [Option<String>; 4],
}

impl Debug for Room {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut doors = f.debug_map();
        for (i, room) in self.doors.iter().enumerate() {
            if let Some(room) = room {
                let dir: Dir = i.into();
                doors.entry(&dir, room);
            }
        }
        doors.finish()
    }
}

#[derive(Copy, Clone, Debug)]
enum Dir {
    North,
    South,
    East,
    West,
}

impl Dir {
    fn rev(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }

    fn to_str(self) -> &'static str {
        match self {
            Self::North => "north",
            Self::South => "south",
            Self::East => "east",
            Self::West => "west",
        }
    }
}

impl From<&str> for Dir {
    fn from(value: &str) -> Self {
        match value {
            "north" => Self::North,
            "south" => Self::South,
            "east" => Self::East,
            "west" => Self::West,
            _ => unreachable!("Unknown direction: '{value}'"),
        }
    }
}

impl From<usize> for Dir {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::North,
            1 => Self::South,
            2 => Self::East,
            3 => Self::West,
            _ => unreachable!("Unknown direction: '{value}'"),
        }
    }
}

enum RobotState {
    Exploring,
    CheckingWeight(Vec<usize>, usize, usize, Dir),
}

pub struct Day25 {
    intcode: IntCode<isize>,
    last_area: Option<(String, Dir)>,
    rooms: BTreeMap<String, Room>,
    items: Vec<String>,
    state: RobotState,
}

enum Command<'a> {
    Single(Option<&'a str>),
    Slice(&'a [&'a str], usize),
}

impl<'a> Iterator for Command<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Single(s) => {
                if let Some(s) = s.take() {
                    Some(s)
                } else {
                    None
                }
            }
            Self::Slice(s, i) => {
                if *i >= s.len() {
                    None
                } else {
                    let s = s[*i];
                    *i += 1;
                    Some(s)
                }
            }
        }
    }
}

impl<'a> From<&'a str> for Command<'a> {
    fn from(s: &'a str) -> Self {
        Self::Single(Some(s))
    }
}

impl<'a> From<&'a [&'a str]> for Command<'a> {
    fn from(s: &'a [&'a str]) -> Self {
        Self::Slice(s, 0)
    }
}

impl Day25 {
    pub fn new() -> Self {
        let mut rooms = BTreeMap::new();
        rooms.insert("Pressure-Sensitive Floor".to_string(), Room::default());
        Self {
            intcode: IntCode::default(),
            last_area: None,
            rooms,
            items: Vec::new(),
            state: RobotState::Exploring,
        }
    }

    fn send_command<'a>(&mut self, command: impl Into<Command<'a>>) {
        for command in command.into() {
            print!("{command}");
            self.intcode
                .input
                .extend(command.as_bytes().iter().map(|v| *v as isize));
        }
        println!();
        self.intcode.input.push_back(b'\n' as isize);
    }

    pub fn explore(&mut self, output: &str) {
        enum ExploreState {
            None,
            Items,
            Doors,
        }

        let mut room_name = String::new();
        let mut state = ExploreState::None;
        let mut items = Vec::new();
        let mut doors: Vec<Dir> = Vec::new();

        for line in output.split('\n') {
            if let Some(line) = line.strip_prefix("== ") {
                room_name = line.trim_end_matches(" ==").to_string();
                items.clear();
                doors.clear();
            } else if line.starts_with("Doors ") {
                state = ExploreState::Doors;
            } else if line.starts_with("Items ") {
                state = ExploreState::Items;
            }
            match state {
                ExploreState::None => {}
                ExploreState::Items => {
                    if let Some(item) = line.strip_prefix("- ") {
                        items.push(item.to_string());
                    }
                }
                ExploreState::Doors => {
                    if let Some(door) = line.strip_prefix("- ") {
                        doors.push(door.into());
                    }
                }
            }
        }

        if let Some((last_room, last_dir)) = self.last_area.take() {
            self.rooms.get_mut(&last_room).unwrap().doors[last_dir as usize] =
                Some(room_name.clone());
            self.rooms.entry(room_name.clone()).or_default().doors[last_dir.rev() as usize] =
                Some(last_room);
        }
        let room = self.rooms.entry(room_name.clone()).or_default();
        let unknown = if room_name == "Security Checkpoint" {
            "Pressure-Sensitive Floor"
        } else {
            "Unknown"
        };
        for &door in doors.iter() {
            if room.doors[door as usize].is_none() {
                room.doors[door as usize] = Some(unknown.into());
            }
        }

        let ignore = &[
            "giant electromagnet",
            "molten lava",
            "photons",
            "infinite loop",
            "escape pod",
        ];
        for item in items.iter().filter(|s| !ignore.contains(&s.as_str())) {
            self.send_command(["take ", item.as_str()].as_slice());
            self.items.push(item.clone());
        }

        if let Some((directions, last_room)) = self.find_path(&room_name, |s| s == "Unknown") {
            self.last_area = Some((last_room, *directions.last().unwrap()));
            for dir in directions {
                self.send_command(dir.to_str());
            }
        } else if let Some((directions, _)) =
            self.find_path(&room_name, |s| s == "Pressure-Sensitive Floor")
        {
            let last_dir = directions.last().unwrap();
            for dir in directions.iter() {
                self.send_command(dir.to_str());
            }
            let possible: Vec<usize> = (1..1 << self.items.len()).collect();
            let idx = possible.len() - 1;
            let mask = possible[idx];
            self.state = RobotState::CheckingWeight(possible, idx, mask, *last_dir);
        } else {
            panic!("Could not get to security checkpoint");
        }
    }

    pub fn check_weight(&mut self, output: &str) {
        if let RobotState::CheckingWeight(possible, current_idx, mask, dir) = &mut self.state {
            let current = possible[*current_idx];
            possible.swap_remove(*current_idx);
            fn retain(possible: &mut Vec<usize>, f: impl Fn(usize) -> bool) {
                let mut idx = 0;
                while idx < possible.len() {
                    if !f(possible[idx]) {
                        possible.swap_remove(idx);
                    } else {
                        idx += 1;
                    }
                }
            }
            if output.contains("heavier") {
                // Strip out all possible entries that are lighter than the current value
                retain(possible, |v| v & !current != 0);
            } else {
                // Strip out all possible entries that are heavier than the current value
                retain(possible, |v| v & current != current);
            }
            let mut min_dist = u32::MAX;
            *current_idx = usize::MAX;
            for (idx, p) in possible.iter().enumerate() {
                let diff = ((*p ^ current) & *mask).count_ones();
                if diff < min_dist {
                    min_dist = diff;
                    *current_idx = idx;
                }
            }
            let next = possible[*current_idx];
            let drop_items = current & !next;
            let take_items = next & !current;

            macro_rules! item {
                ($mask:expr, $action:expr) => {{
                    let mask = $mask;
                    let action = $action;
                    for i in 0..self.items.len() {
                        if mask & (1 << i) != 0 {
                            let item = self.items[i].clone();
                            self.send_command([action, " ", item.as_str()].as_slice());
                        }
                    }
                }};
            }
            let dir = *dir;
            item!(drop_items, "drop");
            item!(take_items, "take");
            self.send_command(dir.to_str())
        } else {
            panic!("Wrong state!");
        }
    }

    pub fn next_commands(&mut self, output: &str) {
        match self.state {
            RobotState::Exploring => self.explore(output),
            RobotState::CheckingWeight(..) => self.check_weight(output),
        }
    }

    fn find_path<F>(&self, from: &str, f: F) -> Option<(Vec<Dir>, String)>
    where
        F: Fn(&str) -> bool,
    {
        let mut queue: VecDeque<(String, Vec<Dir>)> = VecDeque::new();
        let mut seen = HashSet::new();
        queue.push_front((from.to_string(), Vec::new()));
        seen.insert(from.to_string());
        while let Some((room_name, directions)) = queue.pop_front() {
            let room = self.rooms.get(&room_name).unwrap();
            for (i, door) in room.doors.iter().enumerate() {
                if let Some(door) = door {
                    if door != "Unknown" && !seen.insert(door.clone()) {
                        continue;
                    }
                    let mut directions = directions.clone();
                    let dir: Dir = i.into();
                    directions.push(dir);
                    println!("{room_name} {door} {directions:?}");
                    if f(door) {
                        return Some((directions, room_name));
                    }
                    queue.push_back((door.clone(), directions));
                }
            }
        }
        None
    }
}

impl Runner for Day25 {
    fn parse(&mut self, path: &str, _part1: bool) -> Result<(), Error> {
        self.intcode.load(Lines::from_path(path, LinesOpt::RAW)?)
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        let mut output = String::new();
        loop {
            match self.intcode.run() {
                State::WaitingForInput(..) => {
                    println!("{output}");
                    self.next_commands(&output);
                    output.clear();
                }
                State::HasOutput(v) => {
                    if v < 255 {
                        output.push(v as u8 as char);
                    }
                }
                State::Stopped => {
                    println!("{output}");
                    let (_, output) = output
                        .split_once("Oh, hello! You should be able to get in by typing ")
                        .unwrap();
                    let (output, _) = output.split_once(' ').unwrap();
                    return Ok(output.parse::<isize>().unwrap().into());
                }
                x => unreachable!("Unexpected state: {x:?}"),
            }
        }
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Err(Error::Skipped)
    }
}
