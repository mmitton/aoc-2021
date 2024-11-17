#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};
use std::collections::VecDeque;
use std::str::FromStr;

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

impl FromStr for Value {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(num) = s.parse::<isize>() {
            Ok(Self::Imm(num))
        } else if s.len() == 1 {
            let r = s.chars().nth(0).unwrap();
            if !r.is_ascii_lowercase() {
                Err(Error::InvalidInput(s.to_string()))
            } else {
                Ok(Self::Reg((r as u8 - b'a') as usize))
            }
        } else {
            Err(Error::InvalidInput(s.to_string()))
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

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" ").collect();
        match parts[0] {
            "set" => Ok(Self::Set(parts[1].parse()?, parts[2].parse()?)),
            "add" => Ok(Self::Add(parts[1].parse()?, parts[2].parse()?)),
            "mul" => Ok(Self::Mul(parts[1].parse()?, parts[2].parse()?)),
            "mod" => Ok(Self::Mod(parts[1].parse()?, parts[2].parse()?)),
            "jgz" => Ok(Self::Jgz(parts[1].parse()?, parts[2].parse()?)),
            "snd" => Ok(Self::Snd(parts[1].parse()?)),
            "rcv" => Ok(Self::Rcv(parts[1].parse()?)),
            _ => Err(Error::InvalidInput(s.to_string())),
        }
    }
}

struct Cpu<'a> {
    id: isize,
    registers: [isize; 26],
    instructions: &'a [Instruction],
    queue: VecDeque<isize>,
    waiting: bool,
    pc: isize,
}

impl<'a> Cpu<'a> {
    fn new(id: isize, instructions: &'a [Instruction]) -> Self {
        Self {
            id,
            registers: [0; 26],
            instructions,
            queue: VecDeque::new(),
            waiting: false,
            pc: 0,
        }
    }

    fn part1(&mut self) -> isize {
        let mut last_freq: isize = 0;
        while let Some(inst) = self.instructions.get(self.pc as usize) {
            let mut next_pc = self.pc + 1;
            match inst {
                Instruction::Set(x, y) => self.registers[x.as_reg()] = y.value(&self.registers),
                Instruction::Add(x, y) => self.registers[x.as_reg()] += y.value(&self.registers),
                Instruction::Mul(x, y) => self.registers[x.as_reg()] *= y.value(&self.registers),
                Instruction::Mod(x, y) => self.registers[x.as_reg()] %= y.value(&self.registers),
                Instruction::Jgz(x, y) => {
                    if self.registers[x.as_reg()] > 0 {
                        next_pc = self.pc + y.value(&self.registers)
                    }
                }
                Instruction::Snd(x) => last_freq = x.value(&self.registers),
                Instruction::Rcv(x) => {
                    if x.value(&self.registers) != 0 {
                        println!("Last Sound Played: {:?}", last_freq);
                        break;
                    }
                }
            }

            self.pc = next_pc;
        }

        last_freq
    }

    fn part2(&mut self) -> Option<isize> {
        if let Some(inst) = self.instructions.get(self.pc as usize) {
            let mut next_pc = self.pc + 1;
            let mut send_value = None;
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
                    if !self.queue.is_empty() {
                        self.waiting = false;
                        self.registers[x.as_reg()] = self.queue.pop_front().unwrap();
                    } else {
                        self.waiting = true;
                        return None;
                    }
                }
            }

            self.pc = next_pc;
            send_value
        } else {
            self.waiting = true;
            None
        }
    }

    fn can_run(&self) -> bool {
        !self.waiting || !self.queue.is_empty()
    }
}

#[derive(Default)]
pub struct Day18 {
    instructions: Vec<Instruction>,
}

impl Day18 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day18 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            self.instructions.push(line.parse()?);
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

impl Day18 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        let mut cpu = Cpu::new(0, &self.instructions);
        Ok(cpu.part1().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut cpus = [
            Cpu::new(0, &self.instructions),
            Cpu::new(1, &self.instructions),
        ];
        for cpu in cpus.iter_mut() {
            cpu.registers[(b'p' - b'a') as usize] = cpu.id;
        }
        let mut sends = [0, 0];

        loop {
            let send_0 = cpus[0].part2();
            let send_1 = cpus[1].part2();

            if let Some(send_0) = send_0 {
                cpus[1].queue.push_back(send_0);
                sends[0] += 1;
            }
            if let Some(send_1) = send_1 {
                cpus[0].queue.push_back(send_1);
                sends[1] += 1;
            }

            if !cpus[0].can_run() && !cpus[1].can_run() {
                break;
            }
        }

        Ok(sends[1].into())
    }
}
