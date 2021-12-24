const INPUT_FILE: &str = "../input.txt";

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

    fn value(&self, registers: &[isize; 8]) -> isize {
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
    Sub(Value, Value),
    Mul(Value, Value),
    Jnz(Value, Value),
}

impl TryFrom<&str> for Instruction {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = s.split(" ").collect();
        match parts[0] {
            "set" => Ok(Self::Set(parts[1].try_into()?, parts[2].try_into()?)),
            "sub" => Ok(Self::Sub(parts[1].try_into()?, parts[2].try_into()?)),
            "mul" => Ok(Self::Mul(parts[1].try_into()?, parts[2].try_into()?)),
            "jnz" => Ok(Self::Jnz(parts[1].try_into()?, parts[2].try_into()?)),
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
    registers: [isize; 8],
    instructions: Vec<Instruction>,
    pc: isize,
}

impl CPU {
    fn state(&self) -> [isize; 9] {
        let mut state = [0; 9];
        for reg in 0..self.registers.len() {
            state[reg] = self.registers[reg];
        }
        state[8] = self.pc;

        state
    }

    fn new(insts: Vec<Instruction>) -> CPU {
        CPU {
            registers: [0; 8],
            instructions: insts,
            pc: 0,
        }
    }

    fn tick(&mut self) -> bool {
        let inst = &self.instructions[self.pc as usize];
        let mut next_pc = self.pc + 1;
        let mut was_mul = false;
        match inst {
            Instruction::Set(x, y) => self.registers[x.as_reg()] = y.value(&self.registers),
            Instruction::Sub(x, y) => self.registers[x.as_reg()] -= y.value(&self.registers),
            Instruction::Mul(x, y) => {
                self.registers[x.as_reg()] *= y.value(&self.registers);
                was_mul = true;
            }
            Instruction::Jnz(x, y) => {
                if x.value(&self.registers) != 0 {
                    next_pc = self.pc + y.value(&self.registers)
                }
            }
        }

        self.pc = next_pc;
        was_mul
    }
}

fn main() -> Result<(), Error> {
    let instructions = load_input(INPUT_FILE)?;

    let mut muls = 0usize;
    let mut coprocessor = CPU::new(instructions);
    let mut states = Vec::new();
    states.push(coprocessor.state());

    loop {
        if coprocessor.tick() {
            muls += 1;
        }
        let state = coprocessor.state();
        if states.contains(&state) {
            break;
        }

        states.push(state);
        println!("{} {}", muls, states.len());
    }

    println!("Entered state cycle.  Saw {} muls", muls);

    Ok(())
}
