#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

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

fn load_input(filename: &str) -> Result<(usize, Vec<(OpCode, [usize; 3])>), Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();
    let mut instructions = Vec::new();
    let mut pc_in = 0;

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim().to_string();
        if line == "" {
            continue;
        }

        if line.starts_with("#ip ") {
            pc_in = line[4..].parse()?;
        } else {
            let parts: Vec<&str> = line.split(" ").collect();
            let a = parts[1].parse()?;
            let b = parts[2].parse()?;
            let c = parts[3].parse()?;

            let op_code = match parts[0] {
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
                _ => unreachable!(),
            };
            instructions.push((op_code, [a, b, c]));
        }
    }

    Ok((pc_in, instructions))
}

#[derive(Clone)]
struct CPU {
    pc_in: usize,
    registers: [usize; 6],
    instructions: Vec<(OpCode, [usize; 3])>,
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

    fn running(&self) -> bool {
        self.registers[self.pc_in] < self.instructions.len()
    }

    fn step(&mut self) {
        let mut inst = self.instructions[self.registers[self.pc_in]];
        if self.registers[self.pc_in] == 2 {
            inst.1[0] = self.registers[5] / self.registers[2];
        }
        if self.registers[self.pc_in] == 8 {
            self.registers[4] = self.registers[5];
        }
        // println!(
        //     "ip={:02} {:9?} {:?} {} {} {} ",
        //     self.registers[self.pc_in], self.registers, inst.0, inst.1[0], inst.1[1], inst.1[2]
        // );
        match inst.0 {
            OpCode::AddR => self.addr(inst.1[0], inst.1[1], inst.1[2]),
            OpCode::AddI => self.addi(inst.1[0], inst.1[1], inst.1[2]),
            OpCode::MulR => self.mulr(inst.1[0], inst.1[1], inst.1[2]),
            OpCode::MulI => self.muli(inst.1[0], inst.1[1], inst.1[2]),
            OpCode::BanR => self.banr(inst.1[0], inst.1[1], inst.1[2]),
            OpCode::BanI => self.bani(inst.1[0], inst.1[1], inst.1[2]),
            OpCode::BorR => self.borr(inst.1[0], inst.1[1], inst.1[2]),
            OpCode::BorI => self.bori(inst.1[0], inst.1[1], inst.1[2]),
            OpCode::SetR => self.setr(inst.1[0], inst.1[1], inst.1[2]),
            OpCode::SetI => self.seti(inst.1[0], inst.1[1], inst.1[2]),
            OpCode::GtIR => self.gtir(inst.1[0], inst.1[1], inst.1[2]),
            OpCode::GtRI => self.gtri(inst.1[0], inst.1[1], inst.1[2]),
            OpCode::GtRR => self.gtrr(inst.1[0], inst.1[1], inst.1[2]),
            OpCode::EqIR => self.eqir(inst.1[0], inst.1[1], inst.1[2]),
            OpCode::EqRI => self.eqri(inst.1[0], inst.1[1], inst.1[2]),
            OpCode::EqRR => self.eqrr(inst.1[0], inst.1[1], inst.1[2]),
        }
        self.registers[self.pc_in] += 1;
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
    let (pc_in, instructions) = load_input(INPUT_FILE)?;

    let mut cpu = CPU {
        pc_in: pc_in,
        registers: [0; 6],
        instructions: instructions,
    };
    cpu.registers[0] = 1;

    while cpu.running() {
        cpu.step();
    }

    println!("Answer: {}", cpu.registers[0]);

    Ok(())
}
