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

fn load_input(filename: &str) -> Result<Vec<[[usize; 4]; 3]>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();
    let mut inputs = Vec::new();

    let mut before = [0; 4];
    let mut instruction = [0; 4];
    let mut after = [0; 4];

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim().to_string();
        if line == "" || line.starts_with("#") {
            continue;
        }

        let line = line.replace("[", "");
        let line = line.replace("]", "");
        let line = line.replace(",", "");
        let line = line.replace("  ", " ");
        let parts: Vec<&str> = line.split(" ").collect();

        if parts[0] == "Before:" {
            before[0] = parts[1].parse()?;
            before[1] = parts[2].parse()?;
            before[2] = parts[3].parse()?;
            before[3] = parts[4].parse()?;
        } else if parts[0] == "After:" {
            after[0] = parts[1].parse()?;
            after[1] = parts[2].parse()?;
            after[2] = parts[3].parse()?;
            after[3] = parts[4].parse()?;
            inputs.push([before.clone(), instruction.clone(), after.clone()]);
        } else {
            instruction[0] = parts[0].parse()?;
            instruction[1] = parts[1].parse()?;
            instruction[2] = parts[2].parse()?;
            instruction[3] = parts[3].parse()?;
        }
    }

    Ok(inputs)
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
    let inputs = load_input(INPUT_FILE)?;
    let operations = [
        CPU::addr,
        CPU::addi,
        CPU::mulr,
        CPU::muli,
        CPU::banr,
        CPU::bani,
        CPU::borr,
        CPU::bori,
        CPU::setr,
        CPU::seti,
        CPU::gtir,
        CPU::gtri,
        CPU::gtrr,
        CPU::eqir,
        CPU::eqri,
        CPU::eqrr,
    ];

    let mut op_codes = [[(false, false); 16]; 16];

    let mut answer = 0;
    for input in &inputs {
        let initial = CPU {
            registers: input[0],
        };
        let mut num_matches = 0;
        for (op_code, func) in operations.iter().enumerate() {
            let mut test = initial.clone();
            func(&mut test, input[1][1], input[1][2], input[1][3]);
            if test.registers.eq(&input[2]) {
                num_matches += 1;
                op_codes[op_code][input[1][0]].0 = true;
            } else {
                op_codes[op_code][input[1][0]].1 = true;
            }
        }
        assert!(num_matches > 0);
        if num_matches >= 3 {
            answer += 1;
        }
    }

    let mut maybe = vec![Vec::new(); 16];
    for i in 0..16 {
        for (code, (did_match, did_not_match)) in op_codes[i].iter().enumerate() {
            if *did_match && !*did_not_match {
                maybe[i].push(code);
            }
        }
    }
    let mut mapped_op_codes = vec![None; 16];
    let mut mapped = true;
    while mapped {
        mapped = false;
        for i in 0..16 {
            if maybe[i].len() == 1 {
                mapped = true;
                let mapped = maybe[i][0];
                mapped_op_codes[i] = Some(mapped);
                for j in 0..16 {
                    maybe[j].retain(|op_code| *op_code != mapped);
                }
            }
        }
    }
    for op_code in [
        OpCode::AddR,
        OpCode::AddI,
        OpCode::MulR,
        OpCode::MulI,
        OpCode::BanR,
        OpCode::BanI,
        OpCode::BorR,
        OpCode::BorI,
        OpCode::SetR,
        OpCode::SetI,
        OpCode::GtIR,
        OpCode::GtRI,
        OpCode::GtRR,
        OpCode::EqIR,
        OpCode::EqRI,
        OpCode::EqRR,
    ] {
        println!(
            "{:?} - {:?} - {:?}",
            op_code, mapped_op_codes[op_code as usize], maybe[op_code as usize]
        );
    }
    println!("Answer: {}", answer);

    Ok(())
}
