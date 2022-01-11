const INPUT_FILE: &str = "../input-2.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NAN(std::num::ParseIntError),
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Error {
        Error::NAN(e)
    }
}

fn load_input(filename: &str) -> Result<Vec<(OpCode, [usize; 3])>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();
    let mut instructions = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim().to_string();
        if line == "" || line.starts_with("#") {
            continue;
        }

        let parts: Vec<&str> = line.split(" ").collect();
        let op_code = parts[0].parse()?;
        let a = parts[1].parse()?;
        let b = parts[2].parse()?;
        let c = parts[3].parse()?;

        let op_code = match op_code {
            5 => OpCode::AddR,
            14 => OpCode::AddI,
            3 => OpCode::MulR,
            10 => OpCode::MulI,
            12 => OpCode::BanR,
            9 => OpCode::BanI,
            1 => OpCode::BorR,
            0 => OpCode::BorI,
            4 => OpCode::SetR,
            2 => OpCode::SetI,
            6 => OpCode::GtIR,
            8 => OpCode::GtRI,
            11 => OpCode::GtRR,
            7 => OpCode::EqIR,
            13 => OpCode::EqRI,
            15 => OpCode::EqRR,
            _ => unreachable!(),
        };

        instructions.push((op_code, [a, b, c]));
    }

    Ok(instructions)
}

#[derive(Clone)]
struct CPU {
    registers: [usize; 4],
}

impl CPU {
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
}

#[repr(usize)]
#[derive(Debug, Copy, Clone)]
enum OpCode {
    AddR = 0,
    AddI = 1,
    MulR = 2,
    MulI = 3,
    BanR = 4,
    BanI = 5,
    BorR = 6,
    BorI = 7,
    SetR = 8,
    SetI = 9,
    GtIR = 10,
    GtRI = 11,
    GtRR = 12,
    EqIR = 13,
    EqRI = 14,
    EqRR = 15,
}

fn main() -> Result<(), Error> {
    let instructions = load_input(INPUT_FILE)?;

    let mut cpu = CPU {
        registers: [0, 0, 0, 0],
    };

    for inst in &instructions {
        match inst.0 {
            OpCode::AddR => cpu.addr(inst.1[0], inst.1[1], inst.1[2]),
            OpCode::AddI => cpu.addi(inst.1[0], inst.1[1], inst.1[2]),
            OpCode::MulR => cpu.mulr(inst.1[0], inst.1[1], inst.1[2]),
            OpCode::MulI => cpu.muli(inst.1[0], inst.1[1], inst.1[2]),
            OpCode::BanR => cpu.banr(inst.1[0], inst.1[1], inst.1[2]),
            OpCode::BanI => cpu.bani(inst.1[0], inst.1[1], inst.1[2]),
            OpCode::BorR => cpu.borr(inst.1[0], inst.1[1], inst.1[2]),
            OpCode::BorI => cpu.bori(inst.1[0], inst.1[1], inst.1[2]),
            OpCode::SetR => cpu.setr(inst.1[0], inst.1[1], inst.1[2]),
            OpCode::SetI => cpu.seti(inst.1[0], inst.1[1], inst.1[2]),
            OpCode::GtIR => cpu.gtir(inst.1[0], inst.1[1], inst.1[2]),
            OpCode::GtRI => cpu.gtri(inst.1[0], inst.1[1], inst.1[2]),
            OpCode::GtRR => cpu.gtrr(inst.1[0], inst.1[1], inst.1[2]),
            OpCode::EqIR => cpu.eqir(inst.1[0], inst.1[1], inst.1[2]),
            OpCode::EqRI => cpu.eqri(inst.1[0], inst.1[1], inst.1[2]),
            OpCode::EqRR => cpu.eqrr(inst.1[0], inst.1[1], inst.1[2]),
        }
    }

    println!("Answer: {}", cpu.registers[0]);

    Ok(())
}
