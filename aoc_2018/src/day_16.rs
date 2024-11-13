#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Clone)]
struct Cpu {
    registers: [usize; 4],
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

#[derive(Default)]
pub struct Day16 {
    part1: Vec<([usize; 4], [usize; 4], [usize; 4])>,
    part2: Vec<[usize; 4]>,
    op_codes: HashMap<usize, OpCode>,
}

impl Day16 {
    pub fn new() -> Self {
        Self::default()
    }

    fn map_op_codes(&mut self) -> usize {
        let operations = [
            Cpu::addr,
            Cpu::addi,
            Cpu::mulr,
            Cpu::muli,
            Cpu::banr,
            Cpu::bani,
            Cpu::borr,
            Cpu::bori,
            Cpu::setr,
            Cpu::seti,
            Cpu::gtir,
            Cpu::gtri,
            Cpu::gtrr,
            Cpu::eqir,
            Cpu::eqri,
            Cpu::eqrr,
        ];

        let mut op_codes = [[(false, false); 16]; 16];

        let mut three_or_more = 0;
        for (before, inst, after) in self.part1.iter() {
            let initial = Cpu { registers: *before };
            let mut num_matches = 0;
            for (op_code, func) in operations.iter().enumerate() {
                let mut test = initial.clone();
                func(&mut test, inst[1], inst[2], inst[3]);
                if test.registers.eq(after) {
                    num_matches += 1;
                    op_codes[op_code][inst[0]].0 = true;
                } else {
                    op_codes[op_code][inst[0]].1 = true;
                }
            }
            assert!(num_matches > 0);
            if num_matches >= 3 {
                three_or_more += 1;
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
                    maybe
                        .iter_mut()
                        .for_each(|maybe| maybe.retain(|opcode| *opcode != mapped));
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
            match mapped_op_codes[op_code as usize] {
                Some(op_num) => {
                    let _ = self.op_codes.insert(op_num, op_code);
                }
                None => unreachable!(),
            }
        }
        three_or_more
    }
}

impl Runner for Day16 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::REMOVE_EMPTY)?;
        let mut lines = lines.iter();
        while let Some(line) = lines.next() {
            if let Some(line) = line.strip_prefix("Before: [") {
                // Part 1
                let line = line.strip_suffix(']').unwrap();
                let parts: Vec<&str> = line.split(", ").collect();
                let before: [usize; 4] = std::array::from_fn(|i| parts[i].parse().unwrap());
                let line = lines.next().unwrap();
                let parts: Vec<&str> = line.split_whitespace().collect();
                let inst: [usize; 4] = std::array::from_fn(|i| parts[i].parse().unwrap());
                let line = lines.next().unwrap();
                let line = line.strip_prefix("After:  [").unwrap();
                let line = line.strip_suffix(']').unwrap();
                let parts: Vec<&str> = line.split(", ").collect();
                let after: [usize; 4] = std::array::from_fn(|i| parts[i].parse().unwrap());
                self.part1.push((before, inst, after));
            } else {
                // Part 2
                let parts: Vec<&str> = line.split_whitespace().collect();
                let inst: [usize; 4] = std::array::from_fn(|i| parts[i].parse().unwrap());
                self.part2.push(inst);
            }
        }

        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.map_op_codes().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        self.map_op_codes();
        let mut cpu = Cpu {
            registers: [0, 0, 0, 0],
        };
        for inst in self.part2.iter() {
            match self.op_codes.get(&inst[0]).unwrap() {
                OpCode::AddR => cpu.addr(inst[1], inst[2], inst[3]),
                OpCode::AddI => cpu.addi(inst[1], inst[2], inst[3]),
                OpCode::MulR => cpu.mulr(inst[1], inst[2], inst[3]),
                OpCode::MulI => cpu.muli(inst[1], inst[2], inst[3]),
                OpCode::BanR => cpu.banr(inst[1], inst[2], inst[3]),
                OpCode::BanI => cpu.bani(inst[1], inst[2], inst[3]),
                OpCode::BorR => cpu.borr(inst[1], inst[2], inst[3]),
                OpCode::BorI => cpu.bori(inst[1], inst[2], inst[3]),
                OpCode::SetR => cpu.setr(inst[1], inst[2], inst[3]),
                OpCode::SetI => cpu.seti(inst[1], inst[2], inst[3]),
                OpCode::GtIR => cpu.gtir(inst[1], inst[2], inst[3]),
                OpCode::GtRI => cpu.gtri(inst[1], inst[2], inst[3]),
                OpCode::GtRR => cpu.gtrr(inst[1], inst[2], inst[3]),
                OpCode::EqIR => cpu.eqir(inst[1], inst[2], inst[3]),
                OpCode::EqRI => cpu.eqri(inst[1], inst[2], inst[3]),
                OpCode::EqRR => cpu.eqrr(inst[1], inst[2], inst[3]),
            }
        }
        Ok(cpu.registers[0].into())
    }
}
