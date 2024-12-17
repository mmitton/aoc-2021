#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt};

#[derive(Default)]
pub struct Day17 {
    cpu: Cpu,
}

#[derive(Clone, Default)]
pub struct Cpu {
    memory: Vec<u8>,
    registers: [usize; 3],
    pc: usize,
    outputs: Vec<u8>,
}

impl Cpu {
    const A: usize = 0;
    const B: usize = 1;
    const C: usize = 2;

    fn combo(&self, v: u8) -> usize {
        match v {
            0..=3 => v as usize,
            4 => self.registers[Self::A],
            5 => self.registers[Self::B],
            6 => self.registers[Self::C],
            _ => panic!(),
        }
    }

    fn step(&mut self) -> bool {
        if let (Some(op), Some(operand)) = (self.memory.get(self.pc), self.memory.get(self.pc + 1))
        {
            self.pc += 2;
            match op {
                0 => self.registers[Self::A] /= 2usize.pow(self.combo(*operand) as u32),
                1 => self.registers[Self::B] ^= *operand as usize,
                2 => self.registers[Self::B] = self.combo(*operand) % 8,
                3 => {
                    if self.registers[Self::A] != 0 {
                        self.pc = *operand as usize;
                    }
                }
                4 => self.registers[Self::B] ^= self.registers[Self::C],
                5 => self.outputs.push((self.combo(*operand) % 8) as u8),
                6 => {
                    self.registers[Self::B] =
                        self.registers[Self::A] / 2usize.pow(self.combo(*operand) as u32)
                }
                7 => {
                    self.registers[Self::C] =
                        self.registers[Self::A] / 2usize.pow(self.combo(*operand) as u32)
                }
                _ => panic!(),
            }
            true
        } else {
            false
        }
    }

    fn output_str(&self) -> String {
        use std::fmt::Write;
        let mut output = String::new();
        for (i, v) in self.outputs.iter().enumerate() {
            if i != 0 {
                output.push(',');
            }
            write!(&mut output, "{v}").unwrap();
        }

        output
    }

    fn _disassemble(&self) {
        for pc in (0..self.memory.len()).step_by(2) {
            let op = self.memory[pc];
            let operand = self.memory[pc + 1];
            let combo = match operand {
                0..=3 => format!("{operand}"),
                4 => "a".into(),
                5 => "b".into(),
                6 => "c".into(),
                _ => "".into(),
            };
            match op {
                0 => println!("a /= 2usize.pow({combo} as u32);"),
                1 => println!("b ^= {operand};"),
                2 => println!("b = {combo} % 8;"),
                3 => println!("jnz {operand};"),
                4 => println!("b ^= c;"),
                5 => println!("outputs.push(({combo} % 8) as u8);"),
                6 => println!("b = a / 2usize.pow({combo} as u32);"),
                7 => println!("c = a / 2usize.pow({combo} as u32);"),
                _ => panic!(),
            }
        }
    }

    fn run(&mut self) {
        while self.step() {}
    }
}

impl Day17 {
    pub fn new() -> Self {
        Self::default()
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        self.cpu.run();

        Ok(self.cpu.output_str().into())
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        let mut inputs = Vec::new();
        for a in 1..1024 {
            self.cpu.registers[Cpu::A] = a;
            self.cpu.registers[Cpu::B] = 0;
            self.cpu.registers[Cpu::C] = 0;
            self.cpu.outputs.clear();
            self.cpu.pc = 0;
            self.cpu.run();
            if self.cpu.outputs[0] == self.cpu.memory[0] {
                inputs.push(a);
            }
        }

        let mut pos = 1;
        while pos < self.cpu.memory.len() {
            let mut next = Vec::new();

            for a in inputs {
                for bit in 0..8 {
                    let a = (bit << (7 + 3 * pos)) | a;
                    self.cpu.registers[Cpu::A] = a;
                    self.cpu.registers[Cpu::B] = 0;
                    self.cpu.registers[Cpu::C] = 0;
                    self.cpu.outputs.clear();
                    self.cpu.pc = 0;
                    self.cpu.run();

                    if self.cpu.outputs.len() > pos && self.cpu.outputs[pos] == self.cpu.memory[pos]
                    {
                        next.push(a);
                    }
                }
            }
            pos += 1;

            inputs = next;
        }
        Ok(inputs.iter().min().copied().unwrap().into())
    }
}

impl helper::Runner for Day17 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::REMOVE_EMPTY)?;
        for line in lines.iter() {
            if let Some(rest) = line.strip_prefix("Register ") {
                let Some((reg, value)) = rest.split_once(": ") else {
                    return Err(Error::InvalidInput(line.into()));
                };
                let reg = (reg.chars().next().unwrap() as u8 - b'A') as usize;
                let value = value.parse()?;
                self.cpu.registers[reg] = value;
            } else if let Some(rest) = line.strip_prefix("Program: ") {
                for v in rest.split(',') {
                    self.cpu.memory.push(v.parse()?);
                }
            } else {
                return Err(Error::InvalidInput(line.into()));
            }
        }
        Ok(())
    }

    fn run_part(&mut self, part: u8) -> Result<helper::RunOutput, Error> {
        match part {
            1 => self.part1(),
            2 => self.part2(),
            _ => Err(Error::Skipped),
        }
    }
}
