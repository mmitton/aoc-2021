#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

use std::collections::VecDeque;

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NotAValue(String),
    InvalidInstruction(String),
}

#[derive(Clone, Debug)]
enum Value {
    Reg(usize),
    Imm(isize),
}

impl Value {
    fn as_reg(&self) -> usize {
        if let Self::Reg(r) = self {
            return *r;
        }
        panic!("Not a register");
    }

    fn value(&self, registers: &[isize; 26]) -> isize {
        match self {
            Self::Reg(r) => registers[*r],
            Self::Imm(v) => *v,
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
            if r < 'a' || r > 'z' {
                Err(Error::NotAValue(s.to_string()))
            } else {
                Ok(Self::Reg((r as u8 - 'a' as u8) as usize))
            }
        } else {
            Err(Error::NotAValue(s.to_string()))
        }
    }
}

#[derive(Clone, Debug)]
enum Instruction {
    Set(Value, Value),
    Add(Value, Value),
    Mul(Value, Value),
    Mod(Value, Value),
    Jgz(Value, Value),
    Snd(Value),
    Rcv(Value),
}

impl TryFrom<&str> for Instruction {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = s.split(" ").collect();
        match parts[0] {
            "set" => Ok(Self::Set(parts[1].try_into()?, parts[2].try_into()?)),
            "add" => Ok(Self::Add(parts[1].try_into()?, parts[2].try_into()?)),
            "mul" => Ok(Self::Mul(parts[1].try_into()?, parts[2].try_into()?)),
            "mod" => Ok(Self::Mod(parts[1].try_into()?, parts[2].try_into()?)),
            "jgz" => Ok(Self::Jgz(parts[1].try_into()?, parts[2].try_into()?)),
            "snd" => Ok(Self::Snd(parts[1].try_into()?)),
            "rcv" => Ok(Self::Rcv(parts[1].try_into()?)),
            _ => Err(Error::InvalidInstruction(s.to_string())),
        }
    }
}

fn load_input(filename: &str) -> Result<Vec<Instruction>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;
    let lines = BufReader::new(f).lines();

    let mut instructions: Vec<Instruction> = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" || line.starts_with("#") {
            continue;
        }

        instructions.push(line.try_into()?);
    }

    Ok(instructions)
}

struct CPU {
    id: isize,
    registers: [isize; 26],
    instructions: Vec<Instruction>,
    queue: VecDeque<isize>,
    pc: isize,
}

impl CPU {
    fn new(id: isize, insts: Vec<Instruction>) -> CPU {
        let mut cpu = CPU {
            id: id,
            registers: [0; 26],
            instructions: insts,
            queue: VecDeque::new(),
            pc: 0,
        };
        cpu.registers['p' as usize - 'a' as usize] = id;
        cpu
    }

    fn tick(&mut self) -> (bool, Option<isize>) {
        let inst = &self.instructions[self.pc as usize];
        let mut next_pc = self.pc + 1;
        let mut send_value = None;
        println!("[{}]{}: {:?}", self.id, self.pc, inst);
        match inst {
            Instruction::Set(x, y) => self.registers[x.as_reg()] = y.value(&self.registers),
            Instruction::Add(x, y) => self.registers[x.as_reg()] += y.value(&self.registers),
            Instruction::Mul(x, y) => self.registers[x.as_reg()] *= y.value(&self.registers),
            Instruction::Mod(x, y) => self.registers[x.as_reg()] %= y.value(&self.registers),
            Instruction::Jgz(x, y) => {
                if x.value(&self.registers) > 0 {
                    next_pc = self.pc + y.value(&self.registers)
                }
            }
            Instruction::Snd(x) => send_value = Some(x.value(&self.registers)),
            Instruction::Rcv(x) => {
                if self.queue.len() > 0 {
                    self.registers[x.as_reg()] = self.queue.pop_front().unwrap();
                } else {
                    return (true, None);
                }
            }
        }

        self.pc = next_pc;
        return (false, send_value);
    }
}

fn main() -> Result<(), Error> {
    let instructions = load_input(INPUT_FILE)?;

    let mut cpus = vec![
        CPU::new(0, instructions.clone()),
        CPU::new(1, instructions.clone()),
    ];

    let mut cpu_0_sends = 0;
    let mut cpu_1_sends = 0;

    loop {
        let (waiting_0, send_0) = cpus[0].tick();
        let (waiting_1, send_1) = cpus[1].tick();

        if let Some(send_0) = send_0 {
            cpus[1].queue.push_back(send_0);
            cpu_0_sends += 1;
        }
        if let Some(send_1) = send_1 {
            cpus[0].queue.push_back(send_1);
            cpu_1_sends += 1;
        }

        if waiting_0 && waiting_1 && cpus[0].queue.len() == 0 && cpus[1].queue.len() == 0 {
            break;
        }
    }

    println!(
        "CPU 0 sent {} values, CPU 1 send {} values",
        cpu_0_sends, cpu_1_sends
    );
    Ok(())
}
