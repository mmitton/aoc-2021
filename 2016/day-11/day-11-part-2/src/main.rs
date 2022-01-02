#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input-2.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    InvalidInput(String),
    NoSolution,
}

fn load_input(filename: &str) -> Result<Vec<Item>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();
    let mut items = Vec::new();
    let mut floor = 0;

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim().to_string();
        if line == "" {
            continue;
        }

        floor += 1;
        if line.contains("nothing relevant") {
            continue;
        }

        let line = line.replace(", and a ", ", a ");
        let line = line.replace(" and a ", ", ");
        let line = line.replace(" a ", " ");

        let start = line.find("contains ").unwrap();
        for part in line[start + "contains ".len()..line.len() - 1].split(", ") {
            let microchip = part.find("-compatible microchip");
            let generator = part.find(" generator");
            let item = match (microchip, generator) {
                (Some(microchip), None) => Item {
                    floor: floor,
                    kind: part[0..microchip].to_string(),
                    generator: false,
                },
                (None, Some(generator)) => Item {
                    floor: floor,
                    kind: part[0..generator].to_string(),
                    generator: true,
                },
                _ => return Err(Error::InvalidInput(line.to_string())),
            };
            items.push(item);
        }
    }

    Ok(items)
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Item {
    kind: String,
    generator: bool,
    floor: usize,
}

#[derive(Clone)]
struct State {
    elevator: usize,
    steps: usize,
    items: Vec<Item>,
}

impl State {
    fn print(&self) {
        use std::collections::BTreeMap;

        let mut kinds = Vec::new();
        let mut kinds_map = BTreeMap::new();
        for item in &self.items {
            let abbr = format!(
                "{}{}",
                &item.kind[0..1].to_uppercase(),
                if item.generator { "G" } else { "M" }
            );
            kinds.push(abbr.clone());
            kinds_map.insert(abbr, item.floor);
        }
        kinds.sort();
        println!("State:  steps:{}", self.steps);
        for floor in (1..=4).rev() {
            print!("F{} ", floor);
            if self.elevator == floor {
                print!(" E ");
            } else {
                print!(" . ");
            }
            for kind in &kinds {
                if *kinds_map.get(kind).unwrap() == floor {
                    print!(" {}", kind);
                } else {
                    print!(" . ");
                }
            }
            println!();
        }
    }

    fn is_final(&self) -> bool {
        for item in &self.items {
            if item.floor != 4 {
                return false;
            }
        }
        true
    }

    fn is_valid(&self) -> bool {
        let mut microchips = vec![Vec::new(); 4];
        let mut generators = vec![Vec::new(); 4];
        for item in &self.items {
            if item.generator {
                generators[item.floor - 1].push(item.kind.clone());
            } else {
                microchips[item.floor - 1].push(item.kind.clone());
            }
        }

        for floor in 0..4 {
            for m in &microchips[floor] {
                if generators[floor].contains(m) {
                    continue;
                }
                if generators[floor].len() > 0 {
                    return false;
                }
            }
        }
        true
    }

    fn next_steps(&self) -> Vec<State> {
        let mut in_floor = Vec::new();
        let mut min_floor = 4;
        for i in 0..self.items.len() {
            if self.items[i].floor == self.elevator {
                in_floor.push(i);
            }
            if self.items[i].floor < min_floor {
                min_floor = self.items[i].floor;
            }
        }

        let mut next_states = Vec::new();

        for next_floor in [self.elevator - 1, self.elevator + 1] {
            if next_floor < min_floor || next_floor == 5 {
                continue;
            }

            for i in 0..in_floor.len() {
                for j in i..in_floor.len() {
                    let mut next_state = self.clone();
                    next_state.items[in_floor[i]].floor = next_floor;
                    next_state.items[in_floor[j]].floor = next_floor;
                    next_state.elevator = next_floor;
                    next_state.steps += 1;
                    if next_state.is_valid() {
                        next_states.push(next_state);
                    }
                }
            }
        }

        next_states
    }
}

fn main() -> Result<(), Error> {
    let items = load_input(INPUT_FILE)?;

    let mut seen = std::collections::BTreeSet::new();
    seen.insert((1, items.clone()));
    let mut states = std::collections::VecDeque::new();
    states.push_front(State {
        elevator: 1,
        steps: 0,
        items: items,
    });

    let mut last_step = 0;
    while states.len() > 0 {
        let state = states.pop_front().unwrap();
        if last_step != state.steps {
            last_step = state.steps;
            println!(
                "At step: {}  {} in queue, {} seen",
                last_step,
                states.len(),
                seen.len()
            );
        }
        // state.print();
        'insert_loop: for next_state in state.next_steps() {
            let seen_item = (next_state.elevator, next_state.items.clone());
            if seen.contains(&seen_item) {
                continue 'insert_loop;
            }
            seen.insert(seen_item);

            if next_state.is_final() {
                println!("Final!");
                next_state.print();
                return Ok(());
            }
            states.push_back(next_state);
        }
    }

    Err(Error::NoSolution)
}
