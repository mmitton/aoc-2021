const INPUT_FILE: &str = if cfg!(debug_assertions) {
    "../input-sample.txt"
} else {
    "../input.txt"
};

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Index, IndexMut};

type Int = isize;
struct Intcode {
    mem: Vec<Int>,
    pc: usize,
    halted: bool,
}

impl Intcode {
    fn new(mem: &[Int]) -> Self {
        Self {
            mem: Vec::from(mem),
            pc: 0,
            halted: false,
        }
    }

    fn tick(&mut self) -> bool {
        if self.halted {
            return false;
        }
        let op = self[self.pc];
        self.pc += 1;
        match op {
            1 => {
                let a = self[self[self.pc] as usize];
                let b = self[self[self.pc + 1] as usize];
                let c = self[self.pc + 2] as usize;
                self[c] = a + b;
                self.pc += 3;
            }
            2 => {
                let a = self[self[self.pc] as usize];
                let b = self[self[self.pc + 1] as usize];
                let c = self[self.pc + 2] as usize;
                self[c] = a * b;
                self.pc += 3;
            }
            99 => self.halted = true,
            _ => unreachable!(),
        }

        !self.halted
    }
}

impl Index<usize> for Intcode {
    type Output = Int;

    fn index(&self, index: usize) -> &Self::Output {
        self.mem.get(index).unwrap_or(&0)
    }
}

impl IndexMut<usize> for Intcode {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.mem.len() {
            self.mem
                .extend_from_slice(&vec![0; index - self.mem.len() + 1]);
        }
        self.mem.get_mut(index).unwrap()
    }
}

fn main() {
    let lines: Vec<String> = {
        let file = File::open(INPUT_FILE).expect("Cannot open input file");
        BufReader::new(file).lines().flatten().collect()
    };

    for line in lines.iter() {
        if line.is_empty() {
            continue;
        }

        let mut mem = Vec::new();
        for num in line.split(',') {
            let num: Int = num.parse().unwrap();
            mem.push(num);
        }

        mem[1] = 12;
        mem[2] = 2;
        let mut ic = Intcode::new(&mem);
        while ic.tick() {}
        println!("ans: {}", ic.mem[0]);
    }
}
