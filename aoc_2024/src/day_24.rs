use std::collections::VecDeque;

#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Permutations};

#[derive(Default, Clone)]
struct System {
    wires: HashMap<usize, bool>,
    gates: VecDeque<(Gate, usize, usize, usize)>,
}

impl System {
    fn get<'a, F>(&'a self, var: char, translate: F) -> Result<usize, Error>
    where
        F: Fn(usize) -> &'a str,
    {
        let mut ret = 0usize;
        for (id, val) in self.wires.iter() {
            let name = translate(*id);
            if let Some(bit) = name.strip_prefix(var) {
                if *val {
                    let bit: u32 = bit.parse()?;
                    ret |= 1 << bit;
                }
            }
        }
        Ok(ret)
    }

    fn closest_z<'a, F>(&'a self, c: usize, translate: F) -> Option<usize>
    where
        F: Fn(usize) -> &'a str,
    {
        let mut work = VecDeque::new();
        work.push_back(c);

        while let Some(id) = work.pop_front() {
            if translate(id).starts_with('z') {
                return Some(id);
            }
            for (_, a, b, c) in self.gates.iter().copied() {
                if a == id || b == id {
                    work.push_back(c);
                }
            }
        }

        None
    }

    fn swap(&mut self, a: usize, b: usize) {
        let Some(a) = self.gates.iter().position(|(_, _, _, c)| *c == a) else {
            unreachable!();
        };
        let Some(b) = self.gates.iter().position(|(_, _, _, c)| *c == b) else {
            unreachable!();
        };
        let tmp = self.gates[a].3;
        self.gates[a].3 = self.gates[b].3;
        self.gates[b].3 = tmp;
    }

    fn solve<'a, F>(&'a mut self, translate: F) -> Result<usize, Error>
    where
        F: Fn(usize) -> &'a str,
    {
        assert!(!self.gates.is_empty());
        let mut cnt = 0;
        while let Some((gate, a, b, c)) = self.gates.pop_front() {
            match (self.wires.get(&a), self.wires.get(&b)) {
                (Some(a), Some(b)) => {
                    self.wires.insert(
                        c,
                        match gate {
                            Gate::And => *a && *b,
                            Gate::Or => *a || *b,
                            Gate::Xor => (*a && !*b) || (!*a && *b),
                        },
                    );
                    cnt = 0;
                }
                _ => {
                    if cnt > self.gates.len() {
                        break;
                    }
                    cnt += 1;
                    self.gates.push_back((gate, a, b, c));
                }
            }
        }

        if self.gates.is_empty() {
            self.get('z', translate)
        } else {
            Ok(0)
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Gate {
    And,
    Or,
    Xor,
}

#[derive(Default)]
pub struct Day24 {
    system: System,
    names: Vec<String>,
}

impl Day24 {
    pub fn new() -> Self {
        Self::default()
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self.system.solve(|id| self.names[id].as_str())?.into())
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        let x = self.system.get('x', |id| self.names[id].as_str())?;
        let y = self.system.get('y', |id| self.names[id].as_str())?;
        let target_z = x + y;

        let mut final_bad = Vec::new();
        let mut inner_bad = Vec::new();
        let mut bad = Vec::new();
        for (g, a, b, c) in self.system.gates.iter().copied() {
            if self.names[c] == "z45" {
                continue;
            }
            if self.names[c].starts_with('z') {
                if g != Gate::Xor {
                    bad.push(c);
                    final_bad.push(c);
                }
            } else if !(self.names[a].starts_with('x')
                || self.names[a].starts_with('y')
                || self.names[b].starts_with('x')
                || self.names[b].starts_with('y'))
            {
                // Cannot be Xor
                if g == Gate::Xor {
                    bad.push(c);
                    inner_bad.push(c);
                }
            }
        }

        let mut system = self.system.clone();
        'inner: for inner in inner_bad.iter().copied() {
            let Some(closest_z) = self.system.closest_z(inner, |idx| self.names[idx].as_str())
            else {
                return Err(Error::Unsolved);
            };
            let closest_bit = self.names[closest_z]
                .strip_prefix('z')
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let look_for = format!("z{:02}", closest_bit - 1);
            for final_output in final_bad.iter().copied() {
                if self.names[final_output] == look_for {
                    system.swap(inner, final_output);
                    continue 'inner;
                }
            }

            return Err(Error::Unsolved);
        }

        let mut sys = system.clone();
        let z = sys.solve(|idx| self.names[idx].as_str())?;
        let diff = target_z ^ z;
        let bits = diff.trailing_zeros();

        let mut gates = Vec::new();
        let x = format!("x{:02}", bits);
        let y = format!("y{:02}", bits);
        for (_, a, b, c) in system.gates.iter().copied() {
            if self.names[a] == x && self.names[b] == y {
                gates.push(c);
            }
        }

        if gates.len() != 2 {
            return Err(Error::Unsolved);
        }

        let mut sys = system.clone();
        sys.swap(gates[0], gates[1]);
        let z = sys.solve(|idx| self.names[idx].as_str())?;
        let diff = target_z ^ z;
        if diff != 0 {
            return Err(Error::Unsolved);
        }
        bad.append(&mut gates);

        let mut wires: Vec<_> = bad
            .iter()
            .copied()
            .map(|c| self.names[c].as_str())
            .collect();
        wires.sort();
        Ok(wires.join(",").into())
    }
}

impl helper::Runner for Day24 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        let mut lines = lines.iter();

        let mut names: HashMap<String, usize> = HashMap::default();
        macro_rules! get_name_id {
            ($name:expr) => {{
                if let Some(id) = names.get($name) {
                    *id
                } else {
                    let id = names.len();
                    names.insert($name.into(), id);
                    self.names.push($name.into());
                    id
                }
            }};
        }

        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }
            let Some((name, val)) = line.split_once(": ") else {
                return Err(Error::InvalidInput(line.into()));
            };
            self.system.wires.insert(
                get_name_id!(name),
                match val {
                    "0" => false,
                    "1" => true,
                    _ => return Err(Error::InvalidInput(line.into())),
                },
            );
        }

        for line in lines {
            let parts: Vec<_> = line.split_whitespace().collect();
            if parts.len() != 5 {
                return Err(Error::InvalidInput(line.into()));
            }
            let gate = match parts[1] {
                "AND" => Gate::And,
                "OR" => Gate::Or,
                "XOR" => Gate::Xor,
                _ => return Err(Error::InvalidInput(line.into())),
            };
            self.system.gates.push_back((
                gate,
                get_name_id!(parts[0]),
                get_name_id!(parts[2]),
                get_name_id!(parts[4]),
            ));
        }
        Ok(())
    }

    fn run_part(&mut self, part: u8) -> Result<helper::RunOutput, Error> {
        match part {
            1 => self.part1(),
            2 => self.part2(),
            _ => Err(Error::Skipped),
        }
    }
}
