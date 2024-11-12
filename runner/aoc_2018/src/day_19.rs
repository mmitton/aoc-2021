#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};
use std::str::FromStr;

#[derive(Default)]
struct Cpu {
    instructions: Vec<Instruction>,
    registers: [usize; 6],
    pc_in: usize,
}

impl Cpu {
    fn addr(&mut self, a: usize, b: usize, c: usize) {
        self.registers[c] = self.registers[a] + self.registers[b];
    }

    fn addi(&mut self, a: usize, b: usize, c: usize) {
        self.registers[c] = self.registers[a] + b;
    }

    fn mulr(&mut self, a: usize, b: usize, c: usize) {
        self.registers[c] = self.registers[a] * self.registers[b];
    }

    fn muli(&mut self, a: usize, b: usize, c: usize) {
        self.registers[c] = self.registers[a] * b;
    }

    fn banr(&mut self, a: usize, b: usize, c: usize) {
        self.registers[c] = self.registers[a] & self.registers[b];
    }

    fn bani(&mut self, a: usize, b: usize, c: usize) {
        self.registers[c] = self.registers[a] & b;
    }

    fn borr(&mut self, a: usize, b: usize, c: usize) {
        self.registers[c] = self.registers[a] | self.registers[b];
    }

    fn bori(&mut self, a: usize, b: usize, c: usize) {
        self.registers[c] = self.registers[a] | b;
    }

    fn setr(&mut self, a: usize, _b: usize, c: usize) {
        self.registers[c] = self.registers[a];
    }

    fn seti(&mut self, a: usize, _b: usize, c: usize) {
        self.registers[c] = a;
    }

    fn gtir(&mut self, a: usize, b: usize, c: usize) {
        self.registers[c] = if a > self.registers[b] { 1 } else { 0 };
    }

    fn gtri(&mut self, a: usize, b: usize, c: usize) {
        self.registers[c] = if self.registers[a] > b { 1 } else { 0 };
    }

    fn gtrr(&mut self, a: usize, b: usize, c: usize) {
        self.registers[c] = if self.registers[a] > self.registers[b] {
            1
        } else {
            0
        };
    }

    fn eqir(&mut self, a: usize, b: usize, c: usize) {
        self.registers[c] = if a == self.registers[b] { 1 } else { 0 };
    }

    fn eqri(&mut self, a: usize, b: usize, c: usize) {
        self.registers[c] = if self.registers[a] == b { 1 } else { 0 };
    }

    fn eqrr(&mut self, a: usize, b: usize, c: usize) {
        self.registers[c] = if self.registers[a] == self.registers[b] {
            1
        } else {
            0
        };
    }

    fn step(&mut self) -> bool {
        if let Some(inst) = self.instructions.get(self.registers[self.pc_in]) {
            let mut operands = inst.operands;
            if self.registers[self.pc_in] == 2 {
                operands[0] = self.registers[5] / self.registers[2];
            }
            if self.registers[self.pc_in] == 8 {
                self.registers[4] = self.registers[5];
            }
            match inst.opcode {
                OpCode::AddR => self.addr(operands[0], operands[1], operands[2]),
                OpCode::AddI => self.addi(operands[0], operands[1], operands[2]),
                OpCode::MulR => self.mulr(operands[0], operands[1], operands[2]),
                OpCode::MulI => self.muli(operands[0], operands[1], operands[2]),
                OpCode::BanR => self.banr(operands[0], operands[1], operands[2]),
                OpCode::BanI => self.bani(operands[0], operands[1], operands[2]),
                OpCode::BorR => self.borr(operands[0], operands[1], operands[2]),
                OpCode::BorI => self.bori(operands[0], operands[1], operands[2]),
                OpCode::SetR => self.setr(operands[0], operands[1], operands[2]),
                OpCode::SetI => self.seti(operands[0], operands[1], operands[2]),
                OpCode::GtIR => self.gtir(operands[0], operands[1], operands[2]),
                OpCode::GtRI => self.gtri(operands[0], operands[1], operands[2]),
                OpCode::GtRR => self.gtrr(operands[0], operands[1], operands[2]),
                OpCode::EqIR => self.eqir(operands[0], operands[1], operands[2]),
                OpCode::EqRI => self.eqri(operands[0], operands[1], operands[2]),
                OpCode::EqRR => self.eqrr(operands[0], operands[1], operands[2]),
            }
            self.registers[self.pc_in] += 1;
            true
        } else {
            false
        }
    }
}

#[repr(usize)]
#[derive(Debug, Copy, Clone)]
enum OpCode {
    AddR,
    AddI,
    MulR,
    MulI,
    BanR,
    BanI,
    BorR,
    BorI,
    SetR,
    SetI,
    GtIR,
    GtRI,
    GtRR,
    EqIR,
    EqRI,
    EqRR,
}

struct Instruction {
    opcode: OpCode,
    operands: [usize; 3],
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" ").collect();

        let opcode = match parts[0] {
            "addr" => OpCode::AddR,
            "addi" => OpCode::AddI,
            "mulr" => OpCode::MulR,
            "muli" => OpCode::MulI,
            "banr" => OpCode::BanR,
            "bani" => OpCode::BanI,
            "borr" => OpCode::BorR,
            "bori" => OpCode::BorI,
            "setr" => OpCode::SetR,
            "seti" => OpCode::SetI,
            "gtir" => OpCode::GtIR,
            "gtri" => OpCode::GtRI,
            "gtrr" => OpCode::GtRR,
            "eqir" => OpCode::EqIR,
            "eqri" => OpCode::EqRI,
            "eqrr" => OpCode::EqRR,
            _ => return Err(Error::InvalidInput(s.into())),
        };
        let operands: [usize; 3] = [parts[1].parse()?, parts[2].parse()?, parts[3].parse()?];
        Ok(Self { opcode, operands })
    }
}

#[derive(Default)]
pub struct Day19 {
    cpu: Cpu,
}

impl Day19 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day19 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            if let Some(line) = line.strip_prefix("#ip ") {
                self.cpu.pc_in = line.parse()?;
            } else {
                self.cpu.instructions.push(line.parse()?);
            }
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        while self.cpu.step() {}
        Ok(self.cpu.registers[0].into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        self.cpu.registers[0] = 1;
        while self.cpu.step() {}
        Ok(self.cpu.registers[0].into())
    }
}
