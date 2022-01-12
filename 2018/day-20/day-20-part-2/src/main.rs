#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

use std::collections::BTreeMap;

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    InvalidInput(String, String),
}

fn load_input(filename: &str) -> Result<Vec<Instruction>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();
    let mut directions = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" || line.starts_with("#") {
            continue;
        }

        let chars: Vec<char> = line.chars().collect();
        let (instruction, consumed) = Instruction::parse(&chars, 0)?;
        if consumed != line.len() {
            return Err(Error::InvalidInput(
                format!("Line not consumed:  {} {}", consumed, line.len()),
                line.to_string(),
            ));
        }

        directions.push(instruction);
    }

    Ok(directions)
}

struct Room {
    n: bool,
    s: bool,
    e: bool,
    w: bool,
}

struct Map {
    rooms: BTreeMap<(isize, isize), Room>,
}

impl Map {
    fn new() -> Self {
        let mut rooms = BTreeMap::new();
        rooms.insert(
            (0, 0),
            Room {
                n: false,
                s: false,
                e: false,
                w: false,
            },
        );
        Self { rooms: rooms }
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

    fn print(&self) {
        let mut x0 = isize::MAX;
        let mut x1 = isize::MIN;
        let mut y0 = isize::MAX;
        let mut y1 = isize::MIN;

        for (coord, _) in &self.rooms {
            if coord.0 < x0 {
                x0 = coord.0;
            }
            if coord.0 > x1 {
                x1 = coord.0;
            }
            if coord.1 < y0 {
                y0 = coord.1;
            }
            if coord.1 > y1 {
                y1 = coord.1;
            }
        }

        let y_size = ((y1 - y0 + 1) as usize * 2) + 1;
        let x_size = ((x1 - x0 + 1) as usize * 2) + 2;
        let mut output = vec![vec![' '; x_size]; y_size];

        for (coord, room) in &self.rooms {
            let x = (coord.0 - x0) as usize * 2;
            let y = (coord.1 - y0) as usize * 2;
            output[y + 1][x + 1] = if coord.0 == 0 && coord.1 == 0 {
                'X'
            } else {
                '.'
            };
            output[y][x] = '#';
            output[y + 2][x] = '#';
            output[y + 2][x + 2] = '#';
            output[y][x + 2] = '#';
            output[y][x + 1] = if room.n { '-' } else { '#' };
            output[y + 2][x + 1] = if room.s { '-' } else { '#' };
            output[y + 1][x] = if room.w { '|' } else { '#' };
            output[y + 1][x + 2] = if room.e { '|' } else { '#' };
        }

        for y in 0..output.len() {
            for x in 0..output[y].len() {
                print!("{}", output[y][x]);
            }
            println!();
        }
    }

    fn answer(&self) -> usize {
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
            if dist >= 1000 {
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
}

#[derive(Debug)]
enum Instruction {
    List(Vec<Instruction>),
    Path(Vec<char>),
    Branch(Vec<Instruction>),
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
                        return Err(Error::InvalidInput(
                            "Invalid Start".to_string(),
                            chars.iter().collect(),
                        ));
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
                                return Err(Error::InvalidInput(
                                    format!("Branch Seperator: {}", chars[idx]),
                                    chars.iter().collect(),
                                ))
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
                    return Err(Error::InvalidInput(
                        format!("Invalid Char: {}", chars[idx]),
                        chars.iter().collect(),
                    ))
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

fn main() -> Result<(), Error> {
    let instructions = load_input(INPUT_FILE)?;

    for instructions in instructions {
        if cfg!(debug_assertions) {
            println!("instructions: {:?}", instructions);
            println!("Building map");
        }
        let mut map = Map::new();
        map.process(&instructions, &mut Vec::new());
        if cfg!(debug_assertions) {
            map.print();
        }
        println!("{} rooms pass through at least 1000 doors", map.answer());
        println!();
    }

    Ok(())
}
