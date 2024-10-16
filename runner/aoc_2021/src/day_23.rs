#[allow(unused_imports)]
use helper::{
    print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner, SmallVec,
};
use std::{
    collections::BTreeMap,
    fmt::Debug,
    ops::{Deref, DerefMut},
};

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
    Empty,
    Skip(char),
}

impl Debug for Amphipod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Amber => 'A',
                Self::Bronze => 'B',
                Self::Copper => 'C',
                Self::Desert => 'D',
                Self::Empty => '.',
                Self::Skip(c) => *c,
            }
        )
    }
}

macro_rules! impl_from_amphipod {
    ($ty:ty) => {
        impl From<$ty> for Amphipod {
            fn from(value: $ty) -> Self {
                match value {
                    0 => Amphipod::Amber,
                    1 => Amphipod::Bronze,
                    2 => Amphipod::Copper,
                    3 => Amphipod::Desert,
                    4 => Amphipod::Empty,
                    _ => panic!(),
                }
            }
        }

        impl From<Amphipod> for $ty {
            fn from(value: Amphipod) -> Self {
                match value {
                    Amphipod::Amber => 0,
                    Amphipod::Bronze => 1,
                    Amphipod::Copper => 2,
                    Amphipod::Desert => 3,
                    Amphipod::Empty => 4,
                    Amphipod::Skip(_) => 5,
                }
            }
        }
    };
}

impl_from_amphipod!(u16);
impl_from_amphipod!(u32);
impl_from_amphipod!(usize);

impl Amphipod {
    fn cost(&self) -> usize {
        match self {
            Self::Amber => 1,
            Self::Bronze => 10,
            Self::Copper => 100,
            Self::Desert => 1000,
            Self::Empty | Self::Skip(_) => unreachable!(),
        }
    }

    fn target(&self) -> (usize, usize) {
        match self {
            Self::Amber => (0, 2),
            Self::Bronze => (1, 4),
            Self::Copper => (2, 6),
            Self::Desert => (3, 8),
            Self::Empty | Self::Skip(_) => unreachable!(),
        }
    }
}

impl TryFrom<char> for Amphipod {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Self::Amber),
            'B' => Ok(Self::Bronze),
            'C' => Ok(Self::Copper),
            'D' => Ok(Self::Desert),
            _ => Err(Error::InvalidInput(format!("Invalid Amphipod {value:?}"))),
        }
    }
}

#[derive(Copy, Clone, Debug, Default, Hash, PartialEq, Eq)]
struct Room {
    amphipods: SmallVec<u16, Amphipod, 2>,
}

impl Room {
    fn needs_move(&self, target: Amphipod) -> bool {
        self.amphipods.iter().filter(|a| a != &target).count() != 0
    }

    fn top(&self, len: u8) -> u8 {
        len - self.amphipods.len()
    }

    fn is_valid(&self, target: Amphipod, len: u8) -> bool {
        self.amphipods.len() == len && !self.needs_move(target)
    }
}

impl Deref for Room {
    type Target = SmallVec<u16, Amphipod, 2>;

    fn deref(&self) -> &Self::Target {
        &self.amphipods
    }
}

impl DerefMut for Room {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.amphipods
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct State {
    hall: [Amphipod; 11],
    rooms: [Room; 4],
}

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for amphipod in self.hall.iter() {
            write!(f, "{amphipod:?}")?;
        }
        write!(f, "]")?;
        for (room_idx, room) in self.rooms.iter().enumerate() {
            write!(f, ", Room {room_idx} {:?}", room.amphipods)?;
        }
        Ok(())
    }
}

pub struct Day23 {
    rooms: [Room; 4],
}

impl Day23 {
    pub fn new() -> Self {
        Self {
            rooms: [Room::default(); 4],
        }
    }

    fn find_min_cost(&self) -> usize {
        let len = self.rooms[0].len();
        for (idx, room) in self.rooms.iter().enumerate() {
            println!(
                "{room:?}  {}  {}",
                room.needs_move(idx.into()),
                room.top(len)
            );
        }
        let mut initial_state = State {
            hall: [Amphipod::Empty; 11],
            rooms: self.rooms,
        };
        initial_state.hall[2] = Amphipod::Skip('a');
        initial_state.hall[4] = Amphipod::Skip('b');
        initial_state.hall[6] = Amphipod::Skip('c');
        initial_state.hall[8] = Amphipod::Skip('d');

        let mut min_costs = HashMap::default();
        let mut states: BTreeMap<usize, HashSet<State>> = BTreeMap::new();
        states.entry(0).or_default().insert(initial_state);
        min_costs.insert(initial_state, 0);
        let mut best_cost = usize::MAX;

        while let Some((cost, cur_states)) = states.pop_first() {
            if cost > best_cost {
                break;
            }
            for state in cur_states.iter() {
                let mut cost = cost;
                let mut state = *state;
                // println!("{cost} {state:?}");
                // If there are any Amphipods out and they have a free spot in their target, take it
                let mut new_state = state;
                let mut new_cost = cost;
                let mut move_made = true;
                while move_made {
                    move_made = false;
                    for room_idx in 0..4 {
                        let target: Amphipod = room_idx.into();
                        let (_, target_at) = target.target();
                        let room = &mut new_state.rooms[room_idx];
                        if !room.is_valid(target, len) && !room.needs_move(target) {
                            // Look left and right for amphipods to move in
                            macro_rules! move_home {
                                ($at:expr) => {
                                    if matches!(new_state.hall[$at], Amphipod::Empty | Amphipod::Skip(_)) {
                                        continue;
                                    } else if new_state.hall[$at] == target {
                                        // Move home
                                        let moves = ((target_at.max($at) - target_at.min($at)) + room.top(len) as usize);
                                        new_cost += moves * target.cost();
                                        new_state.hall[$at] = Amphipod::Empty;
                                        room.push(target);
                                        move_made = true;
                                    } else {
                                        // This is a different kind of amphipod, cannot move any more home
                                        break;
                                    }
                                };
                            }
                            for at in (0..target_at).rev() {
                                move_home!(at);
                            }
                            for at in target_at + 1..11 {
                                move_home!(at);
                            }
                        }
                    }
                }
                if new_cost != cost {
                    if new_state.rooms[0].is_valid(Amphipod::Amber, len)
                        && new_state.rooms[1].is_valid(Amphipod::Bronze, len)
                        && new_state.rooms[2].is_valid(Amphipod::Copper, len)
                        && new_state.rooms[3].is_valid(Amphipod::Desert, len)
                    {
                        best_cost = best_cost.min(new_cost);
                        continue;
                    }

                    cost = new_cost;
                    state = new_state;
                    // states.entry(cost).or_default().insert(new_state);
                    // continue;
                }

                // No easy moves, now just move each bad amphipod out of the rooms
                for room_idx in 0..4 {
                    let target: Amphipod = room_idx.into();
                    if state.rooms[room_idx].needs_move(target) {
                        let mut new_state = state;
                        let top = new_state.rooms[room_idx].top(len) as usize;
                        let room_at = target.target().1;
                        let amphipod = new_state.rooms[room_idx].pop().unwrap();

                        macro_rules! move_out {
                            ($pos:expr) => {{
                                assert_eq!(new_state.hall[$pos], Amphipod::Empty);
                                let moves = (room_at.max($pos) - room_at.min($pos)) + top + 1;
                                let new_cost = cost + (moves * amphipod.cost());
                                new_state.hall[$pos] = amphipod;
                                let min_cost = min_costs.entry(new_state).or_insert(usize::MAX);
                                if new_cost < *min_cost {
                                    *min_cost = new_cost;
                                    states.entry(new_cost).or_default().insert(new_state);
                                }
                                new_state.hall[$pos] = Amphipod::Empty;
                            }};
                        }

                        for pos in (0..room_at).rev() {
                            match &new_state.hall[pos] {
                                Amphipod::Skip(_) => {}
                                Amphipod::Empty => move_out!(pos),
                                _ => break,
                            }
                        }
                        for pos in room_at + 1..11 {
                            match &new_state.hall[pos] {
                                Amphipod::Skip(_) => {}
                                Amphipod::Empty => move_out!(pos),
                                _ => break,
                            }
                        }
                    }
                }
            }
        }

        best_cost
    }
}

impl Runner for Day23 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        assert_eq!(lines.len(), 5);
        for j in (0..2).rev() {
            let line: Vec<char> = lines[2 + j].chars().collect();
            for i in 0..4 {
                self.rooms[i].push(line[3 + (i * 2)].try_into()?);
            }
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        for i in 0..4 {
            let amphipod: Amphipod = (i as usize).into();
            println!(
                "{:?} {:?} {:?}",
                amphipod,
                amphipod.cost(),
                amphipod.target()
            );
        }
        Ok(self.find_min_cost().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let top = self.rooms[0].pop().unwrap();
        self.rooms[0].push(Amphipod::Desert);
        self.rooms[0].push(Amphipod::Desert);
        self.rooms[0].push(top);

        let top = self.rooms[1].pop().unwrap();
        self.rooms[1].push(Amphipod::Bronze);
        self.rooms[1].push(Amphipod::Copper);
        self.rooms[1].push(top);

        let top = self.rooms[2].pop().unwrap();
        self.rooms[2].push(Amphipod::Amber);
        self.rooms[2].push(Amphipod::Bronze);
        self.rooms[2].push(top);

        let top = self.rooms[3].pop().unwrap();
        self.rooms[3].push(Amphipod::Copper);
        self.rooms[3].push(Amphipod::Amber);
        self.rooms[3].push(top);

        Ok(self.find_min_cost().into())
    }
}
