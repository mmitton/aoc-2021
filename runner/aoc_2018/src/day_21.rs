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
            match inst.opcode {
                OpCode::AddR => self.addr(inst.a, inst.b, inst.c),
                OpCode::AddI => self.addi(inst.a, inst.b, inst.c),
                OpCode::MulR => self.mulr(inst.a, inst.b, inst.c),
                OpCode::MulI => self.muli(inst.a, inst.b, inst.c),
                OpCode::BanR => self.banr(inst.a, inst.b, inst.c),
                OpCode::BanI => self.bani(inst.a, inst.b, inst.c),
                OpCode::BorR => self.borr(inst.a, inst.b, inst.c),
                OpCode::BorI => self.bori(inst.a, inst.b, inst.c),
                OpCode::SetR => self.setr(inst.a, inst.b, inst.c),
                OpCode::SetI => self.seti(inst.a, inst.b, inst.c),
                OpCode::GtIR => self.gtir(inst.a, inst.b, inst.c),
                OpCode::GtRI => self.gtri(inst.a, inst.b, inst.c),
                OpCode::GtRR => self.gtrr(inst.a, inst.b, inst.c),
                OpCode::EqIR => self.eqir(inst.a, inst.b, inst.c),
                OpCode::EqRI => self.eqri(inst.a, inst.b, inst.c),
                OpCode::EqRR => self.eqrr(inst.a, inst.b, inst.c),
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
    a: usize,
    b: usize,
    c: usize,
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
        let a = parts[1].parse()?;
        let b = parts[2].parse()?;
        let c = parts[3].parse()?;
        Ok(Self { opcode, a, b, c })
    }
}

#[derive(Default)]
pub struct Day21 {
    cpu: Cpu,
}

impl Day21 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day21 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            if let Some(pc_in) = line.strip_prefix("#ip ") {
                self.cpu.pc_in = pc_in.parse()?;
            } else {
                self.cpu.instructions.push(line.parse()?);
            }
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        while self.cpu.step() {
            if self.cpu.registers[self.cpu.pc_in] == 28 {
                return Ok(self.cpu.registers[5].into());
            }
        }
        Err(Error::Unsolved)
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut last = 0;
        let mut seen = HashSet::default();
        while self.cpu.step() {
            if self.cpu.registers[self.cpu.pc_in] == 28 {
                if !seen.insert(self.cpu.registers[5]) {
                    return Ok(last.into());
                }
                last = self.cpu.registers[5];
            }
        }
        Err(Error::Unsolved)
    }
}
