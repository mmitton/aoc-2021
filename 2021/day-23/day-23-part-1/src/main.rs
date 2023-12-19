#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

use std::collections::{BTreeMap, BinaryHeap};
const MAX_Y: u8 = 3;

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NoSolution,
}

#[derive(Debug, Clone)]
struct Amphipod {
    a: char,
    x: u8,
    y: u8,
    target: u8,
}

#[derive(Debug, Clone)]
struct State {
    cost: usize,
    key: u128,
    amphipods: Vec<Amphipod>,
    home: usize,
}

impl std::cmp::PartialOrd for State {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        rhs.cost.partial_cmp(&self.cost)
    }
}

impl std::cmp::Ord for State {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        rhs.cost.cmp(&self.cost)
    }
}

impl std::cmp::PartialEq for State {
    fn eq(&self, rhs: &Self) -> bool {
        self.cost == rhs.cost && self.key == rhs.key
    }
}
impl std::cmp::Eq for State {}

impl State {
    fn get_offset(x: u8, y: u8) -> usize {
        3 * (if y == 1 {
            x - 1
        } else {
            11 + ((x - 3) / 2) + (4 * (y - 2))
        }) as usize
    }

    fn update_key(&mut self) {
        self.key = 0;
        for amphipod in &self.amphipods {
            let offset = Self::get_offset(amphipod.x, amphipod.y);
            let v = amphipod.a as u8 - 'A' as u8;

            self.key |= (v as u128 + 1) << offset;
        }
    }

    fn amphipod_at(&self, x: u8, y: u8) -> Option<&Amphipod> {
        for amphipod in &self.amphipods {
            if amphipod.x == x && amphipod.y == y {
                return Some(&amphipod);
            }
        }
        None
    }

    fn amphipod_at_mut<'a>(&'a mut self, x: u8, y: u8) -> Option<&'a mut Amphipod> {
        for amphipod in &mut self.amphipods {
            if amphipod.x == x && amphipod.y == y {
                return Some(amphipod);
            }
        }
        None
    }

    fn find_possible_moves(&self, moves: &mut Vec<State>) {
        moves.clear();

        let hallway_pos = [1, 2, 4, 6, 8, 10, 11];
        'big_loop: for i in 0..self.amphipods.len() {
            if self.amphipods[i].target == 0 {
                // Already home
                continue;
            }

            let cost_per_move = match self.amphipods[i].a {
                'A' => 1,
                'B' => 10,
                'C' => 100,
                'D' => 1000,
                _ => unreachable!(),
            };

            if self.amphipods[i].y > 1 {
                // Need to move out of room

                // Check to see if someone is blocking
                for y in 2..self.amphipods[i].y {
                    if let Some(_) = self.amphipod_at(self.amphipods[i].x, y) {
                        continue 'big_loop;
                    }
                }

                // Consider all positions to the left
                for x in (1..=self.amphipods[i].x).rev() {
                    if !hallway_pos.contains(&x) {
                        continue;
                    }
                    if let Some(_) = self.amphipod_at(x, 1) {
                        break;
                    }
                    let mut new_state = self.clone();
                    new_state.amphipods[i].x = x;
                    new_state.amphipods[i].y = 1;
                    new_state.cost += ((self.amphipods[i].y - 1) + (self.amphipods[i].x - x))
                        as usize
                        * cost_per_move;
                    new_state.update_key();
                    moves.push(new_state);
                }

                // Consider all positions to the right
                for x in self.amphipods[i].x..=11 {
                    if !hallway_pos.contains(&x) {
                        continue;
                    }
                    if let Some(_) = self.amphipod_at(x, 1) {
                        break;
                    }
                    let mut new_state = self.clone();
                    new_state.amphipods[i].x = x;
                    new_state.amphipods[i].y = 1;
                    new_state.cost += ((self.amphipods[i].y - 1) + (x - self.amphipods[i].x))
                        as usize
                        * cost_per_move;
                    new_state.update_key();
                    moves.push(new_state);
                }
            } else {
                // Need to move around the hallway

                // Consider all positions to the left
                for x in (1..=self.amphipods[i].x - 1).rev() {
                    if x == self.amphipods[i].target {
                        // Check to see if we can go home
                        let mut y = 0;
                        for target_y in 2..=MAX_Y {
                            y = target_y;
                            if let Some(a) = self.amphipod_at(x, y) {
                                if a.target == 0 {
                                    y -= 1;
                                } else {
                                    y = 0;
                                }
                                break;
                            }
                        }

                        if y > 0 {
                            let mut new_state = self.clone();
                            new_state.amphipods[i].x = x;
                            new_state.amphipods[i].y = y;
                            new_state.amphipods[i].target = 0;
                            new_state.cost +=
                                ((y - self.amphipods[i].y) + (self.amphipods[i].x - x)) as usize
                                    * cost_per_move;
                            new_state.home += 1;
                            moves.push(new_state);
                        }
                    }

                    if !hallway_pos.contains(&x) {
                        continue;
                    }
                    if let Some(_) = self.amphipod_at(x, 1) {
                        break;
                    }
                }

                // Consider all positions to the right
                for x in self.amphipods[i].x + 1..=11 {
                    if x == self.amphipods[i].target {
                        // Check to see if we can go home
                        let mut y = 0;
                        for target_y in 2..=MAX_Y {
                            y = target_y;
                            if let Some(a) = self.amphipod_at(x, y) {
                                if a.target == 0 {
                                    y -= 1;
                                } else {
                                    y = 0;
                                }
                                break;
                            }
                        }

                        if y > 0 {
                            let mut new_state = self.clone();
                            new_state.amphipods[i].x = x;
                            new_state.amphipods[i].y = y;
                            new_state.amphipods[i].target = 0;
                            new_state.cost +=
                                ((y - self.amphipods[i].y) + (x - self.amphipods[i].x)) as usize
                                    * cost_per_move;
                            new_state.home += 1;
                            moves.push(new_state);
                        }
                    }

                    if !hallway_pos.contains(&x) {
                        continue;
                    }
                    if let Some(_) = self.amphipod_at(x, 1) {
                        break;
                    }
                }
            }
        }
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(
            fmt,
            "#############  Home: {}  Cost: {}",
            self.home, self.cost
        )?;

        write!(fmt, "#")?;
        for x in 1..=11 {
            if let Some(amphipod) = self.amphipod_at(x, 1) {
                if amphipod.target == 0 {
                    write!(fmt, "{}", amphipod.a.to_lowercase())?;
                } else {
                    write!(fmt, "{}", amphipod.a)?;
                }
            } else {
                write!(fmt, " ")?;
            }
        }
        writeln!(fmt, "#")?;

        for y in 2..=MAX_Y {
            if y == 2 {
                write!(fmt, "###")?;
            } else {
                write!(fmt, "  #")?;
            }
            for x in 3..=9 {
                if x % 2 == 0 {
                    write!(fmt, "#")?;
                } else if let Some(amphipod) = self.amphipod_at(x, y) {
                    if amphipod.target == 0 {
                        write!(fmt, "{}", amphipod.a.to_lowercase())?;
                    } else {
                        write!(fmt, "{}", amphipod.a)?;
                    }
                } else {
                    write!(fmt, " ")?;
                }
            }
            if y == 2 {
                writeln!(fmt, "###")?;
            } else {
                writeln!(fmt, "#")?;
            }
        }

        writeln!(fmt, "  #########")?;

        Ok(())
    }
}

fn load_input(filename: &str) -> Result<State, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();

    let mut state = State {
        amphipods: Vec::new(),
        cost: 0,
        home: 0,
        key: 0,
    };

    for (y, line) in lines.enumerate() {
        if y < 2 || y > MAX_Y as usize {
            continue;
        }
        let chars: Vec<char> = line.unwrap().chars().collect();

        for x in (3..=9).step_by(2) {
            state.amphipods.push(Amphipod {
                a: chars[x],
                x: x as u8,
                y: y as u8,
                target: ((chars[x] as u8 - 'A' as u8) * 2) + 3,
            });
        }
    }

    println!("amphipods: {:?}", state.amphipods);

    let mut home = 0usize;
    for (c, x) in [('A', 3), ('B', 5), ('C', 7), ('D', 9)] {
        let mut bottom_good = true;
        for y in (2..=MAX_Y).rev() {
            if bottom_good {
                bottom_good = false;
                if let Some(amphipod) = state.amphipod_at_mut(x, y) {
                    if amphipod.a == c {
                        home += 1;
                        amphipod.target = 0;
                        bottom_good = true;
                    }
                }
            }
        }
    }
    state.home = home;
    state.update_key();

    Ok(state)
}

fn main() -> Result<(), Error> {
    let state = load_input(INPUT_FILE)?;

    let mut states = BinaryHeap::new();
    let mut costs: BTreeMap<u128, usize> = BTreeMap::new();
    let mut moves = Vec::new();

    costs.insert(state.key, state.cost);
    states.push(state);

    let mut iters = 0;
    loop {
        match states.pop() {
            Some(state) => {
                if state.home == state.amphipods.len() {
                    println!("{}", state);
                    println!("Found!");
                    return Ok(());
                }
                if iters % 1000 == 0 {
                    println!("States: {}", states.len());
                    println!("{}", state);
                }
                state.find_possible_moves(&mut moves);
                for mut state in moves.drain(..) {
                    state.update_key();
                    if let Some(cost) = costs.get(&state.key) {
                        if *cost > state.cost {
                            states.retain(|s| s.key != state.key);
                            costs.insert(state.key, state.cost);
                            states.push(state);
                        }
                    } else {
                        costs.insert(state.key, state.cost);
                        states.push(state);
                    }
                }
            }
            None => break,
        }
        iters += 1;
    }

    Err(Error::NoSolution)
}
