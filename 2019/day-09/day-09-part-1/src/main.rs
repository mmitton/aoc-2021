const INPUT_FILE: &str = if cfg!(debug_assertions) {
    "../input-sample.txt"
} else {
    "../input.txt"
};

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::mpsc::{self, Receiver, Sender};

struct Intcode {
    name: &'static str,
    mem: Vec<isize>,
    pc: usize,
    halted: bool,
    pre_input: Vec<isize>,
    input: Option<Receiver<isize>>,
    output: Option<Sender<isize>>,
    last_output: Option<isize>,
    debug: bool,
    relative_offset_base: isize,
}

impl Intcode {
    fn new(name: &'static str, code: &str) -> Self {
        let mut mem = Vec::new();
        for num in code.split(',') {
            let num: isize = num.parse().unwrap();
            mem.push(num);
        }

        Self {
            name,
            mem,
            pc: 0,
            halted: false,
            pre_input: Vec::new(),
            input: None,
            output: None,
            last_output: None,
            debug: false,
            relative_offset_base: 0,
        }
    }

    #[allow(dead_code)]
    fn set_input(&mut self, input: &mut Self) {
        assert!(self.input.is_none());
        assert!(input.output.is_none());

        let (tx, rx) = mpsc::channel();
        self.input = Some(rx);
        input.output = Some(tx);
    }

    fn read(&self, arg: usize) -> isize {
        let mut op = self.mem[self.pc] / 100;
        for _ in 1..arg {
            op /= 10;
        }
        let mode = op % 10;
        match mode {
            0 => {
                // Indirect Mem Access
                let addr = self.mem[self.pc + arg] as usize;
                let val = if addr >= self.mem.len() {
                    0
                } else {
                    self.mem[addr]
                };
                if self.debug {
                    println!(
                        "{}: Reading arg {arg} from memory address {addr}: {val}",
                        self.name
                    );
                }
                val
            }
            1 => {
                // Immediate
                if self.debug {
                    println!(
                        "{}: Reading arg {arg} as an immediate: {}",
                        self.name,
                        self.mem[self.pc + arg]
                    );
                }
                self.mem[self.pc + arg]
            }
            2 => {
                // Relative
                let addr = (self.relative_offset_base + self.mem[self.pc + arg]) as usize;
                let val = if addr >= self.mem.len() {
                    0
                } else {
                    self.mem[addr]
                };
                if self.debug {
                    println!(
                        "{}: Reading arg {arg} from memory address {addr}: {val}",
                        self.name
                    );
                }
                val
            }
            _ => unreachable!(),
        }
    }

    fn write(&mut self, arg: usize, val: isize) {
        let mut op = self.mem[self.pc] / 100;
        for _ in 1..arg {
            op /= 10;
        }
        let mode = op % 10;

        macro_rules! expand_mem {
            ($addr:expr) => {{
                if $addr >= self.mem.len() {
                    self.mem
                        .extend_from_slice(&vec![0; $addr - self.mem.len() + 1]);
                }
            }};
        }

        match mode {
            0 => {
                // Indirect Mem Access
                let addr = self.mem[self.pc + arg] as usize;
                expand_mem!(addr);
                if self.debug {
                    println!("{}: Writing {val} to memory address {addr}", self.name);
                }
                self.mem[addr] = val;
            }
            2 => {
                // Relative
                let addr = (self.relative_offset_base + self.mem[self.pc + arg]) as usize;
                expand_mem!(addr);
                if self.debug {
                    println!("{}: Writing {val} to memory address {addr}", self.name);
                }
                self.mem[addr] = val;
            }
            _ => unreachable!(),
        }
    }

    fn run(&mut self) {
        while self.tick() {}
    }

    fn tick(&mut self) -> bool {
        if self.halted {
            return false;
        }
        if self.debug {
            println!("{}: PC {}", self.name, self.pc);
        }
        let op = self.mem[self.pc] % 100;
        self.pc += match op {
            1 => {
                // Add
                let a = self.read(1);
                let b = self.read(2);
                self.write(3, a + b);
                4
            }
            2 => {
                // Mul
                let a = self.read(1);
                let b = self.read(2);
                self.write(3, a * b);
                4
            }
            3 => {
                // Input
                let val = if !self.pre_input.is_empty() {
                    self.pre_input.remove(0)
                } else {
                    let rx = self.input.as_mut().expect("Input not set up");
                    rx.recv().expect("Could not read from input")
                };
                if self.debug {
                    println!("Read {val} from input");
                }
                self.write(1, val);
                2
            }
            4 => {
                // Output
                let val = self.read(1);
                self.last_output = Some(val);
                if let Some(tx) = self.output.as_mut() {
                    if self.debug {
                        println!("Sending {val} to output");
                    }
                    if let Err(e) = tx.send(val) {
                        if self.debug {
                            println!("{}: {e:?}", self.name);
                        }
                        self.halted = true;
                    }
                } else if self.debug {
                    println!("Output to screen: {val}");
                }
                2
            }
            5 => {
                // Jump if True
                let a = self.read(1);
                let b = self.read(2);
                if a != 0 {
                    self.pc = b as usize;
                    0
                } else {
                    3
                }
            }
            6 => {
                // Jump if False
                let a = self.read(1);
                let b = self.read(2);
                if a == 0 {
                    self.pc = b as usize;
                    0
                } else {
                    3
                }
            }
            7 => {
                // Set Less Than
                let a = self.read(1);
                let b = self.read(2);
                self.write(3, if a < b { 1 } else { 0 });
                4
            }
            8 => {
                // Set Equals
                let a = self.read(1);
                let b = self.read(2);
                self.write(3, if a == b { 1 } else { 0 });
                4
            }
            9 => {
                // Adjust Relative Offset Base
                let a = self.read(1);
                self.relative_offset_base += a;
                2
            }
            99 => {
                self.halted = true;
                0
            }
            _ => unreachable!(),
        };

        !self.halted
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

        let mut prog = Intcode::new("boost", line);
        prog.pre_input.push(1);
        prog.run();

        println!("ans: {}", prog.last_output.unwrap());
    }
}
