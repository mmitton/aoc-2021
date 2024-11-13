use std::{
    collections::VecDeque,
    ops::{Deref, DerefMut},
};

#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Clone, Debug)]
enum Ident {
    Imm(u16),
    Name(u16),
}

struct WireState(HashMap<u16, u16>);

impl Deref for WireState {
    type Target = HashMap<u16, u16>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for WireState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl WireState {
    fn get_value(&self, ident: &Ident) -> Option<u16> {
        match ident {
            Ident::Imm(v) => Some(*v),
            Ident::Name(name) => self.get(name).copied(),
        }
    }
}

impl Ident {
    fn try_new<F>(s: &str, mut f: F) -> Result<Self, Error>
    where
        F: FnMut(&str) -> u16,
    {
        let chars: Vec<char> = s.chars().collect();
        if chars[0] >= '0' && chars[0] <= '9' {
            Ok(Self::Imm(s.parse::<u16>()?))
        } else {
            for c in &chars {
                if *c < 'a' || *c > 'z' {
                    return Err(Error::InvalidInput(format!("Invalid wire name: {s:?}")));
                }
            }
            Ok(Self::Name(f(s)))
        }
    }
}

#[derive(Clone, Debug)]
enum Op {
    Assign(Ident),
    And(Ident, Ident),
    Or(Ident, Ident),
    LShift(Ident, Ident),
    RShift(Ident, Ident),
    Not(Ident),
}

#[derive(Clone, Debug)]
struct Gate {
    name: u16,
    op: Op,
}

#[derive(Default)]
pub struct Day07 {
    gates: Vec<Gate>,
    names: HashMap<String, u16>,
}

impl Day07 {
    pub fn new() -> Self {
        Self::default()
    }

    fn get_name_id(&mut self, s: &str) -> u16 {
        if let Some(id) = self.names.get(s) {
            *id
        } else {
            let next = self.names.len() as u16;
            self.names.insert(s.into(), next);
            next
        }
    }

    fn eval(&mut self) -> WireState {
        let mut wire_state: WireState = WireState(HashMap::default());
        let mut gates: VecDeque<Gate> = self.gates.clone().into();
        while let Some(gate) = gates.pop_front() {
            if !wire_state.contains_key(&gate.name) {
                // Still need this wire
                match &gate.op {
                    Op::Assign(ident) => {
                        if let Some(v) = wire_state.get_value(ident) {
                            wire_state.insert(gate.name, v);
                        } else {
                            gates.push_back(gate);
                        }
                    }
                    Op::Not(ident) => {
                        if let Some(v) = wire_state.get_value(ident) {
                            wire_state.insert(gate.name, !v);
                        } else {
                            gates.push_back(gate);
                        }
                    }
                    Op::And(lhs, rhs) => {
                        if let (Some(lhs), Some(rhs)) =
                            (wire_state.get_value(lhs), wire_state.get_value(rhs))
                        {
                            wire_state.insert(gate.name, lhs & rhs);
                        } else {
                            gates.push_back(gate);
                        }
                    }
                    Op::Or(lhs, rhs) => {
                        if let (Some(lhs), Some(rhs)) =
                            (wire_state.get_value(lhs), wire_state.get_value(rhs))
                        {
                            wire_state.insert(gate.name, lhs | rhs);
                        } else {
                            gates.push_back(gate);
                        }
                    }
                    Op::LShift(v, sht) => {
                        if let (Some(v), Some(sht)) =
                            (wire_state.get_value(v), wire_state.get_value(sht))
                        {
                            wire_state.insert(gate.name, v << sht);
                        } else {
                            gates.push_back(gate);
                        }
                    }
                    Op::RShift(v, sht) => {
                        if let (Some(v), Some(sht)) =
                            (wire_state.get_value(v), wire_state.get_value(sht))
                        {
                            wire_state.insert(gate.name, v >> sht);
                        } else {
                            gates.push_back(gate);
                        }
                    }
                }
            }
        }

        wire_state
    }
}

impl Runner for Day07 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::ALL)?;
        for line in lines.iter() {
            let parts: Vec<&str> = line.split(" -> ").collect();
            let name = self.get_name_id(parts[1]);

            let op = if parts[0].starts_with("NOT") {
                Op::Not(Ident::try_new(&parts[0][4..], |s| self.get_name_id(s))?)
            } else if parts[0].contains(" AND ") {
                let parts: Vec<&str> = parts[0].split(" AND ").collect();
                Op::And(
                    Ident::try_new(parts[0], |s| self.get_name_id(s))?,
                    Ident::try_new(parts[1], |s| self.get_name_id(s))?,
                )
            } else if parts[0].contains(" OR ") {
                let parts: Vec<&str> = parts[0].split(" OR ").collect();
                Op::Or(
                    Ident::try_new(parts[0], |s| self.get_name_id(s))?,
                    Ident::try_new(parts[1], |s| self.get_name_id(s))?,
                )
            } else if parts[0].contains(" LSHIFT ") {
                let parts: Vec<&str> = parts[0].split(" LSHIFT ").collect();
                Op::LShift(
                    Ident::try_new(parts[0], |s| self.get_name_id(s))?,
                    Ident::try_new(parts[1], |s| self.get_name_id(s))?,
                )
            } else if parts[0].contains(" RSHIFT ") {
                let parts: Vec<&str> = parts[0].split(" RSHIFT ").collect();
                Op::RShift(
                    Ident::try_new(parts[0], |s| self.get_name_id(s))?,
                    Ident::try_new(parts[1], |s| self.get_name_id(s))?,
                )
            } else {
                Op::Assign(Ident::try_new(parts[0], |s| self.get_name_id(s))?)
            };

            self.gates.push(Gate { name, op });
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        let a = self.get_name_id("a");

        let wire_state = self.eval();
        if let Some(a) = wire_state.get(&a).copied() {
            Ok(a.into())
        } else {
            Err(Error::Unsolved)
        }
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let a = self.get_name_id("a");
        let b = self.get_name_id("b");

        let wire_state = self.eval();
        for i in 0..self.gates.len() {
            if self.gates[i].name == b {
                self.gates[i].op = Op::Assign(Ident::Imm(*wire_state.get(&a).unwrap()));
                break;
            }
        }

        let wire_state = self.eval();
        if let Some(a) = wire_state.get(&a).copied() {
            Ok(a.into())
        } else {
            Err(Error::Unsolved)
        }
    }
}
