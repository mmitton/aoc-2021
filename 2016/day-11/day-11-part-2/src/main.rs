use std::collections::{BTreeSet, HashSet};

#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    InvalidInput(String),
    NoSolution,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    generators: u32,
    microchips: u32,
}

impl State {
    const SHIFTS: [u32; 5] = [0, 7, 14, 21, 28];
    const FLOOR_MASKS: [u32; 4] = [
        0b111_1111 << Self::SHIFTS[0],
        0b111_1111 << Self::SHIFTS[1],
        0b111_1111 << Self::SHIFTS[2],
        0b111_1111 << Self::SHIFTS[3],
    ];
    const ELEVATOR_MASK: u32 = 0b1111 << Self::SHIFTS[4];

    fn new(elevator: usize, generators: u32, microchips: u32) -> Self {
        Self {
            generators,
            microchips: (microchips & !Self::ELEVATOR_MASK)
                | ((elevator as u32) << Self::SHIFTS[4]),
        }
    }

    fn initial(items: &[Item]) -> (Self, Vec<Type>) {
        let mut types = BTreeSet::new();
        for item in items.iter() {
            types.insert(item.kind);
        }

        assert!(types.len() < 8);

        let mut generators: u32 = 0;
        let mut microchips: u32 = 0;

        let types: Vec<Type> = types.iter().copied().collect();

        for item in items.iter() {
            let offset = types.iter().position(|&t| t == item.kind).unwrap();
            let shifted_offset = offset + (7 * (item.floor - 1));
            if item.generator {
                generators |= 1 << shifted_offset;
            } else {
                microchips |= 1 << shifted_offset;
            }
        }

        (
            Self {
                generators,
                microchips,
            },
            types,
        )
    }

    fn pairs(&self) -> u64 {
        let mut pairs = [0x00; 8];

        let floors = [
            1 << Self::SHIFTS[0],
            1 << Self::SHIFTS[1],
            1 << Self::SHIFTS[2],
            1 << Self::SHIFTS[3],
        ];
        for i in 0..7 {
            for f in 0..4 {
                if self.generators & floors[f] << i != 0 {
                    pairs[i] |= f as u8 + 1;
                }
                if self.microchips & floors[f] << i != 0 {
                    pairs[i] |= (f as u8 + 1) << 4;
                }
            }
        }

        pairs.sort();
        u64::from_ne_bytes(pairs)
    }

    fn is_valid(&self) -> bool {
        let lone_microchips = self.microchips & !self.generators;
        let has_generator = if self.generators & Self::FLOOR_MASKS[0] != 0 {
            Self::FLOOR_MASKS[0]
        } else {
            0
        } | if self.generators & Self::FLOOR_MASKS[1] != 0 {
            Self::FLOOR_MASKS[1]
        } else {
            0
        } | if self.generators & Self::FLOOR_MASKS[2] != 0 {
            Self::FLOOR_MASKS[2]
        } else {
            0
        } | if self.generators & Self::FLOOR_MASKS[3] != 0 {
            Self::FLOOR_MASKS[3]
        } else {
            0
        };

        lone_microchips & has_generator == 0
    }

    fn elevator(&self) -> usize {
        (self.microchips & Self::ELEVATOR_MASK) as usize >> Self::SHIFTS[4]
    }

    fn print(&self, step: usize, map: &[Type]) {
        println!(
            "Step: {step}  {}",
            if self.is_valid() {
                "Valid"
            } else {
                "NOT VALID"
            }
        );
        let elevator = self.elevator();
        for floor in (0..4).rev() {
            let mask = Self::FLOOR_MASKS[floor];
            let generators = (self.generators & mask) >> Self::SHIFTS[floor];
            let microchips = (self.microchips & mask) >> Self::SHIFTS[floor];
            print!(
                "F{}  {} ",
                floor + 1,
                if floor == elevator { 'E' } else { '.' }
            );
            for (idx, kind) in map.iter().enumerate() {
                if generators & (1 << idx) != 0 {
                    print!(" {}G", kind.abbrev());
                } else {
                    print!(" . ");
                }
                if microchips & (1 << idx) != 0 {
                    print!(" {}M", kind.abbrev());
                } else {
                    print!(" . ");
                }
            }
            println!();
        }
    }

    fn is_final(&self) -> bool {
        let generators_final = self.generators
            & (Self::FLOOR_MASKS[0] | Self::FLOOR_MASKS[1] | Self::FLOOR_MASKS[2])
            == 0;
        let microchips_final = self.microchips
            & (Self::FLOOR_MASKS[0] | Self::FLOOR_MASKS[1] | Self::FLOOR_MASKS[2])
            == 0;
        generators_final && microchips_final
    }

    fn next_states(
        &self,
        steps: usize,
        in_floor: &mut Vec<(u32, u32)>,
        seen: &mut HashSet<(usize, u64)>,
        work: &mut Vec<State>,
    ) -> Option<State> {
        in_floor.clear();

        let elevator = self.elevator();
        let can_move_down = match elevator {
            0 => false,
            1 => (self.generators | self.microchips) & (Self::FLOOR_MASKS[0]) != 0,
            2 => {
                (self.generators | self.microchips) & (Self::FLOOR_MASKS[0] | Self::FLOOR_MASKS[1])
                    != 0
            }
            3 => true,
            _ => unreachable!(),
        };
        // let can_move_down = elevator != 0;

        for i in 0..7 {
            let typ = 1 << i;
            if (self.generators >> Self::SHIFTS[elevator]) & typ != 0 {
                in_floor.push((typ, 0));
            }
            if (self.microchips >> Self::SHIFTS[elevator]) & typ != 0 {
                in_floor.push((0, typ));
            }
        }
        for (skip, (g1, m1)) in in_floor.iter().enumerate() {
            for (g2, m2) in in_floor.iter().skip(skip) {
                macro_rules! check_move {
                    ($new_floor:expr) => {{
                        let g_mask_off = !((g1 | g2) << Self::SHIFTS[elevator]);
                        let m_mask_off = !((m1 | m2) << Self::SHIFTS[elevator]);
                        let g_mask_on = (g1 | g2) << Self::SHIFTS[$new_floor];
                        let m_mask_on = (m1 | m2) << Self::SHIFTS[$new_floor];
                        let generators = (self.generators & g_mask_off) | g_mask_on;
                        let microchips = (self.microchips & m_mask_off) | m_mask_on;

                        let next_state = State::new($new_floor, generators, microchips);
                        if next_state.is_valid() && seen.insert((steps, next_state.pairs())) {
                            if next_state.is_final() {
                                return Some(next_state);
                            }
                            work.push(next_state);
                        }
                    }};
                }
                if can_move_down {
                    // Move down
                    check_move!(elevator - 1);
                }
                if elevator < 3 {
                    // Move up
                    check_move!(elevator + 1);
                }
            }
        }
        None
    }
}

fn load_input(filename: &str) -> Result<Vec<Item>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(Error::IO)?;

    let lines = BufReader::new(f).lines();
    let mut items = Vec::new();
    let mut floor = 0;

    for line in lines {
        let line = line.map_err(Error::IO)?;
        let line = line.trim().to_string();
        if line.is_empty() {
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
                    floor,
                    kind: part[0..microchip].parse().unwrap(),
                    generator: false,
                },
                (None, Some(generator)) => Item {
                    floor,
                    kind: part[0..generator].parse().unwrap(),
                    generator: true,
                },
                _ => return Err(Error::InvalidInput(line.to_string())),
            };
            items.push(item);
        }
    }

    Ok(items)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Type {
    Hydrogen,
    Lithium,
    Strontium,
    Plutonium,
    Elerium,
    Dilithium,
    Thulium,
    Ruthenium,
    Curium,
}

impl Type {
    fn abbrev(&self) -> char {
        match self {
            Self::Hydrogen => 'H',
            Self::Lithium => 'L',
            Self::Strontium => 'S',
            Self::Plutonium => 'P',
            Self::Elerium => 'E',
            Self::Dilithium => 'D',
            Self::Thulium => 'T',
            Self::Ruthenium => 'R',
            Self::Curium => 'C',
        }
    }
}

impl std::str::FromStr for Type {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "hydrogen" => Ok(Self::Hydrogen),
            "lithium" => Ok(Self::Lithium),
            "strontium" => Ok(Self::Strontium),
            "plutonium" => Ok(Self::Plutonium),
            "elerium" => Ok(Self::Elerium),
            "dilithium" => Ok(Self::Dilithium),
            "thulium" => Ok(Self::Thulium),
            "ruthenium" => Ok(Self::Ruthenium),
            "curium" => Ok(Self::Curium),
            _ => Err(format!("Unknown type '{s}'")),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Item {
    kind: Type,
    generator: bool,
    floor: usize,
}

fn main() -> Result<(), Error> {
    let mut items = load_input(INPUT_FILE)?;
    if !cfg!(debug_assertions) {
        items.push(Item {
            kind: Type::Elerium,
            generator: false,
            floor: 1,
        });
        items.push(Item {
            kind: Type::Elerium,
            generator: true,
            floor: 1,
        });
        items.push(Item {
            kind: Type::Dilithium,
            generator: false,
            floor: 1,
        });
        items.push(Item {
            kind: Type::Dilithium,
            generator: true,
            floor: 1,
        });
    }

    let (initial_state, map) = State::initial(&items);
    println!("Initial State:");
    initial_state.print(0, &map);

    let mut in_floor = Vec::new();

    let mut work = Vec::new();
    let mut next_work = Vec::new();
    work.push(initial_state);
    let mut seen = HashSet::new();
    seen.insert((0, initial_state.pairs()));
    let mut steps = 0;
    loop {
        steps += 1;
        next_work.clear();
        while let Some(state) = work.pop() {
            if let Some(final_state) =
                state.next_states(steps, &mut in_floor, &mut seen, &mut next_work)
            {
                println!("Final!");
                final_state.print(steps, &map);
                return Ok(());
            }
        }
        if next_work.is_empty() {
            break;
        }
        std::mem::swap(&mut work, &mut next_work);
    }

    Err(Error::NoSolution)
}
