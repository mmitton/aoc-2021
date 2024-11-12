#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

struct Room {
    n: bool,
    s: bool,
    e: bool,
    w: bool,
}

struct Map {
    rooms: HashMap<(isize, isize), Room>,
}

impl Map {
    fn new() -> Self {
        let mut rooms = HashMap::default();
        rooms.insert(
            (0, 0),
            Room {
                n: false,
                s: false,
                e: false,
                w: false,
            },
        );
        Self { rooms }
    }

    fn process(&mut self, inst: &Instruction, heads: &mut Vec<(isize, isize)>) {
        if heads.len() == 0 {
            heads.push((0, 0));
        }

        match inst {
            Instruction::List(list) => {
                for inst in list {
                    self.process(inst, heads);
                }
            }
            Instruction::Path(path) => {
                for head in heads {
                    for c in path {
                        // Update old room
                        {
                            let old_room = self.rooms.get_mut(head).unwrap();
                            match c {
                                'N' => {
                                    old_room.n = true;
                                    head.1 -= 1;
                                }
                                'S' => {
                                    old_room.s = true;
                                    head.1 += 1;
                                }
                                'E' => {
                                    old_room.e = true;
                                    head.0 += 1;
                                }
                                'W' => {
                                    old_room.w = true;
                                    head.0 -= 1;
                                }
                                _ => unreachable!(),
                            }
                        }

                        // Update new room
                        if !self.rooms.contains_key(head) {
                            self.rooms.insert(
                                *head,
                                Room {
                                    n: false,
                                    s: false,
                                    e: false,
                                    w: false,
                                },
                            );
                        }
                        let new_room = self.rooms.get_mut(head).unwrap();
                        match c {
                            'N' => {
                                new_room.s = true;
                            }
                            'S' => {
                                new_room.n = true;
                            }
                            'E' => {
                                new_room.w = true;
                            }
                            'W' => {
                                new_room.e = true;
                            }
                            _ => unreachable!(),
                        }
                    }
                }
            }
            Instruction::Branch(branches) => {
                let cur_heads = heads.clone();
                heads.clear();

                macro_rules! add_heads {
                    ($new_heads:expr) => {{
                        for head in &$new_heads {
                            if !heads.contains(head) {
                                heads.push(*head);
                            }
                        }
                    }};
                }
                for inst in branches {
                    if let Instruction::Empty = inst {
                        add_heads!(cur_heads);
                    } else {
                        let mut h = cur_heads.clone();
                        self.process(inst, &mut h);
                        add_heads!(h);
                    }
                }
            }
            Instruction::Empty => {}
        }
    }

    fn rooms_reached_after(&self, after: usize) -> usize {
        let mut queue: Vec<(usize, (isize, isize))> = Vec::new();
        let mut seen: Vec<(isize, isize)> = Vec::new();

        macro_rules! insert {
            ($dist:expr, $x:expr, $y:expr) => {{
                if !seen.contains(&($x, $y)) {
                    queue.push(($dist, ($x, $y)));
                    seen.push(($x, $y));
                }
            }};
        }

        insert!(0, 0, 0);

        let mut answer = 0;
        let mut i = 0;
        while i < queue.len() {
            let dist = queue[i].0;
            let coord = queue[i].1;
            if dist >= after {
                answer += 1;
            }
            let room = self.rooms.get(&coord).unwrap();
            if room.n {
                insert!(dist + 1, coord.0, coord.1 - 1);
            }
            if room.s {
                insert!(dist + 1, coord.0, coord.1 + 1);
            }
            if room.e {
                insert!(dist + 1, coord.0 + 1, coord.1);
            }
            if room.w {
                insert!(dist + 1, coord.0 - 1, coord.1);
            }

            i += 1;
        }

        answer
    }

    fn max_dist(&self) -> usize {
        let mut queue: Vec<(usize, (isize, isize))> = Vec::new();
        let mut seen: Vec<(isize, isize)> = Vec::new();

        macro_rules! insert {
            ($dist:expr, $x:expr, $y:expr) => {{
                if !seen.contains(&($x, $y)) {
                    queue.push(($dist, ($x, $y)));
                    seen.push(($x, $y));
                }
            }};
        }

        insert!(0, 0, 0);

        let mut max_dist = 0;
        let mut i = 0;
        while i < queue.len() {
            let dist = queue[i].0;
            let coord = queue[i].1;
            if dist > max_dist {
                max_dist = dist;
            }
            let room = self.rooms.get(&coord).unwrap();
            if room.n {
                insert!(dist + 1, coord.0, coord.1 - 1);
            }
            if room.s {
                insert!(dist + 1, coord.0, coord.1 + 1);
            }
            if room.e {
                insert!(dist + 1, coord.0 + 1, coord.1);
            }
            if room.w {
                insert!(dist + 1, coord.0 - 1, coord.1);
            }

            i += 1;
        }

        max_dist
    }
}

#[derive(Debug, Default)]
enum Instruction {
    List(Vec<Instruction>),
    Path(Vec<char>),
    Branch(Vec<Instruction>),
    #[default]
    Empty,
}

impl Instruction {
    fn parse(chars: &Vec<char>, start_at: usize) -> Result<(Instruction, usize), Error> {
        let mut instructions = Vec::new();
        let mut path = Vec::new();

        let mut idx = start_at;
        loop {
            match chars[idx] {
                '^' => {
                    if instructions.len() != 0 {
                        return Err(Error::InvalidInput(format!(
                            "Invalid Start {:?}",
                            chars.iter().collect::<String>(),
                        )));
                    }
                    idx += 1;
                }
                '|' | ')' => break,
                '$' => {
                    idx += 1;
                    break;
                }
                '(' => {
                    // Process branches
                    if path.len() > 0 {
                        instructions.push(Instruction::Path(path));
                        path = Vec::new();
                    }

                    let mut branches = Vec::new();
                    loop {
                        let (branch, branch_idx) = Self::parse(chars, idx + 1)?;
                        idx = branch_idx;
                        branches.push(branch);
                        match chars[idx] {
                            ')' => {
                                idx += 1;
                                break;
                            }
                            '|' => {}
                            _ => {
                                return Err(Error::InvalidInput(format!(
                                    "Branch Seperator: {} {:?}",
                                    chars[idx],
                                    chars.iter().collect::<String>(),
                                )));
                            }
                        }
                    }

                    instructions.push(Instruction::Branch(branches));
                }
                'N' | 'S' | 'E' | 'W' => {
                    path.push(chars[idx]);
                    idx += 1;
                }
                _ => {
                    return Err(Error::InvalidInput(format!(
                        "Invalid Char: {} {:?}",
                        chars[idx],
                        chars.iter().collect::<String>(),
                    )))
                }
            }
        }

        if path.len() == 0 && instructions.len() == 0 {
            Ok((Instruction::Empty, idx))
        } else {
            if path.len() > 0 {
                instructions.push(Instruction::Path(path));
            }

            if instructions.len() == 1 {
                Ok((instructions.remove(0), idx))
            } else {
                Ok((Instruction::List(instructions), idx))
            }
        }
    }
}

#[derive(Default)]
pub struct Day20 {
    instruction: Instruction,
}

impl Day20 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day20 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        assert_eq!(lines.len(), 1);

        let chars: Vec<char> = lines[0].chars().collect();
        let (instruction, consumed) = Instruction::parse(&chars, 0)?;
        if consumed != lines[0].len() {
            return Err(Error::InvalidInput(format!(
                "Line not consumed:  {} {} {:?}",
                consumed,
                lines[0].len(),
                lines[0]
            )));
        }
        self.instruction = instruction;

        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        let mut map = Map::new();
        map.process(&self.instruction, &mut Vec::new());
        Ok(map.max_dist().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut map = Map::new();
        map.process(&self.instruction, &mut Vec::new());
        Ok(map.rooms_reached_after(1000).into())
    }
}
