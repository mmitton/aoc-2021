use std::collections::BTreeMap;

#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    InvalidImm(String, std::num::ParseIntError),
    InvalidIdent(String),
}

#[derive(Debug)]
enum Ident {
    Imm(u16),
    Name(String),
}

type WireState = BTreeMap<String, u16>;

impl TryFrom<&str> for Ident {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let chars: Vec<char> = s.chars().collect();
        if chars[0] >= '0' && chars[0] <= '9' {
            Ok(Self::Imm(
                s.parse::<u16>()
                    .map_err(|e| Error::InvalidImm(s.to_string(), e))?,
            ))
        } else {
            for c in &chars {
                if *c < 'a' || *c > 'z' {
                    return Err(Error::InvalidIdent(s.to_string()));
                }
            }
            Ok(Self::Name(s.to_string()))
        }
    }
}

#[derive(Debug)]
enum Op {
    Assign(Ident),
    And(Ident, Ident),
    Or(Ident, Ident),
    LShift(Ident, Ident),
    RShift(Ident, Ident),
    Not(Ident),
}

#[derive(Debug)]
struct Instruction {
    name: String,
    op: Op,
}

fn load_input(filename: &str) -> Result<Vec<Instruction>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();

    let mut inputs = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }

        let parts: Vec<&str> = line.split(" -> ").collect();
        let name = parts[1].to_string();

        let op = if parts[0].starts_with("NOT") {
            Op::Not(parts[0][4..].try_into()?)
        } else if parts[0].contains(" AND ") {
            let parts: Vec<&str> = parts[0].split(" AND ").collect();
            Op::And(parts[0].try_into()?, parts[1].try_into()?)
        } else if parts[0].contains(" OR ") {
            let parts: Vec<&str> = parts[0].split(" OR ").collect();
            Op::Or(parts[0].try_into()?, parts[1].try_into()?)
        } else if parts[0].contains(" LSHIFT ") {
            let parts: Vec<&str> = parts[0].split(" LSHIFT ").collect();
            Op::LShift(parts[0].try_into()?, parts[1].try_into()?)
        } else if parts[0].contains(" RSHIFT ") {
            let parts: Vec<&str> = parts[0].split(" RSHIFT ").collect();
            Op::RShift(parts[0].try_into()?, parts[1].try_into()?)
        } else {
            Op::Assign(parts[0].try_into()?)
        };

        inputs.push(Instruction { name: name, op: op });
    }

    Ok(inputs)
}

fn get_value(wire_state: &WireState, ident: &Ident) -> Option<u16> {
    match ident {
        Ident::Imm(v) => Some(*v),
        Ident::Name(name) => match wire_state.get(name) {
            Some(v) => Some(*v),
            None => None,
        },
    }
}

fn eval(inputs: &Vec<Instruction>) -> WireState {
    let mut wire_state: WireState = BTreeMap::new();
    loop {
        let mut updated = false;
        for input in inputs {
            if !wire_state.contains_key(&input.name) {
                // Still need this wire
                match &input.op {
                    Op::Assign(ident) => {
                        if let Some(v) = get_value(&wire_state, &ident) {
                            wire_state.insert(input.name.clone(), v);
                            updated = true;
                        }
                    }
                    Op::Not(ident) => {
                        if let Some(v) = get_value(&wire_state, &ident) {
                            wire_state.insert(input.name.clone(), !v);
                            updated = true;
                        }
                    }
                    Op::And(lhs, rhs) => {
                        match (get_value(&wire_state, &lhs), get_value(&wire_state, &rhs)) {
                            (Some(lhs), Some(rhs)) => {
                                wire_state.insert(input.name.clone(), lhs & rhs);
                                updated = true;
                            }
                            _ => {}
                        }
                    }
                    Op::Or(lhs, rhs) => {
                        match (get_value(&wire_state, &lhs), get_value(&wire_state, &rhs)) {
                            (Some(lhs), Some(rhs)) => {
                                wire_state.insert(input.name.clone(), lhs | rhs);
                                updated = true;
                            }
                            _ => {}
                        }
                    }
                    Op::LShift(v, sht) => {
                        match (get_value(&wire_state, &v), get_value(&wire_state, &sht)) {
                            (Some(v), Some(sht)) => {
                                wire_state.insert(input.name.clone(), v << sht);
                                updated = true;
                            }
                            _ => {}
                        }
                    }
                    Op::RShift(v, sht) => {
                        match (get_value(&wire_state, &v), get_value(&wire_state, &sht)) {
                            (Some(v), Some(sht)) => {
                                wire_state.insert(input.name.clone(), v >> sht);
                                updated = true;
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        if !updated {
            break;
        }
    }

    wire_state
}

fn main() -> Result<(), Error> {
    let mut inputs = load_input(INPUT_FILE)?;

    let wire_state = eval(&inputs);
    for i in 0..inputs.len() {
        if &inputs[i].name == "b" {
            inputs[i].op = Op::Assign(Ident::Imm(*wire_state.get(&"a".to_string()).unwrap()));
            break;
        }
    }

    let wire_state = eval(&inputs);

    println!("a: {:?}", wire_state.get(&"a".to_string()).unwrap());
    Ok(())
}
