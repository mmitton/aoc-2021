#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Clone, Debug)]
#[allow(dead_code)]
struct Reg(usize);

#[derive(Clone, Debug)]
enum Value {
    Reg(usize),
    Imm(isize),
}

impl TryFrom<&str> for Reg {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let v: Value = s.try_into()?;
        if let Value::Reg(r) = v {
            return Ok(Reg(r));
        }
        Err(Error::InvalidInput(format!("Not a Reg: {s:?}")))
    }
}

impl Value {
    fn imm(&self) -> isize {
        match self {
            Self::Imm(v) => *v,
            _ => panic!(),
        }
    }
}

impl TryFrom<&str> for Value {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if let Ok(num) = s.parse::<isize>() {
            Ok(Self::Imm(num))
        } else if s.len() == 1 {
            let r = s.chars().nth(0).unwrap();
            if !('w'..='z').contains(&r) {
                Err(Error::InvalidInput(format!("Not a Value: {s:?}")))
            } else {
                Ok(Self::Reg((r as u8 - b'w') as usize))
            }
        } else {
            Err(Error::InvalidInput(format!("Not a Value: {s:?}")))
        }
    }
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
enum Op {
    Inp(Reg),
    Add(Reg, Value),
    Mul(Reg, Value),
    Div(Reg, Value),
    Mod(Reg, Value),
    Eql(Reg, Value),
}

impl TryFrom<&str> for Op {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = s.split(" ").collect();
        match parts[0] {
            "inp" => Ok(Self::Inp(parts[1].try_into()?)),
            "add" => Ok(Self::Add(parts[1].try_into()?, parts[2].try_into()?)),
            "mul" => Ok(Self::Mul(parts[1].try_into()?, parts[2].try_into()?)),
            "div" => Ok(Self::Div(parts[1].try_into()?, parts[2].try_into()?)),
            "mod" => Ok(Self::Mod(parts[1].try_into()?, parts[2].try_into()?)),
            "eql" => Ok(Self::Eql(parts[1].try_into()?, parts[2].try_into()?)),
            _ => Err(Error::InvalidInput(format!("Not an Op: {s:?}"))),
        }
    }
}

pub struct Day24 {
    ops: Vec<Op>,
}

impl Day24 {
    pub fn new() -> Self {
        Self { ops: Vec::new() }
    }

    fn calculate_limits(&self) -> (usize, usize) {
        let mut stack = std::collections::VecDeque::new();

        fn add_const(op: &Op) -> isize {
            match op {
                Op::Add(_, b) => b.imm(),
                _ => panic!("NOT RIGHT"),
            }
        }

        let mut values = vec![(0, 0); 14];

        for i in 0..14 {
            let idx = i * 18;
            let push_pop = &self.ops[idx + 5];
            let constant = &self.ops[idx + 15];

            let push_pop = add_const(push_pop);
            let constant = add_const(constant);

            if push_pop > 0 {
                // PUSH
                stack.push_back((i, constant));
            } else {
                // POP
                let last = stack.pop_back().unwrap();

                values[i] = (last.0, last.1 + push_pop);
                values[last.0] = (i, -(last.1 + push_pop));
            }
        }

        let mut largest = vec![None; 14];
        let mut smallest = vec![None; 14];

        for i in 0..14 {
            if let Some(v) = largest[values[i].0] {
                largest[i] = Some(v + values[i].1);
            } else if values[i].1 >= 0 {
                largest[i] = Some(9);
            } else {
                largest[i] = Some(9 + values[i].1);
            }

            if let Some(v) = smallest[values[i].0] {
                smallest[i] = Some(v + values[i].1);
            } else if values[i].1 >= 0 {
                smallest[i] = Some(1 + values[i].1);
            } else {
                smallest[i] = Some(1);
            }
        }

        fn convert(num: &[Option<isize>]) -> usize {
            let mut result = 0;
            for n in num.iter() {
                result = (result * 10) + n.unwrap() as usize;
            }
            result
        }
        let smallest = convert(&smallest);
        let largest = convert(&largest);

        (smallest, largest)
    }
}

impl Runner for Day24 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            let op: Op = line.try_into()?;
            self.ops.push(op);
        }
        Ok(())
    }

    fn run_part(&mut self, part: u8) -> Result<RunOutput, Error> {
        match part {
            1 => self.part1(),
            2 => self.part2(),
            _ => Err(Error::Skipped),
        }
    }
}

impl Day24 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.calculate_limits().1.into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self.calculate_limits().0.into())
    }
}
