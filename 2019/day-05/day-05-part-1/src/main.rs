const INPUT_FILE: &str = if cfg!(debug_assertions) {
    "../input-sample.txt"
} else {
    "../input.txt"
};

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

struct Intcode {
    name: &'static str,
    mem: Vec<isize>,
    pc: usize,
    halted: bool,
    input: Option<Receiver<isize>>,
    output: Option<Sender<isize>>,
    debug: bool,
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
            input: None,
            output: None,
            debug: false,
        }
    }

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
                if self.debug {
                    println!(
                        "{}: Reading arg {arg} from memory address {addr}: {}",
                        self.name, self.mem[addr]
                    );
                }
                self.mem[addr]
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
            _ => unreachable!(),
        }
    }

    fn write(&mut self, arg: usize, val: isize) {
        let mut op = self.mem[self.pc] / 100;
        for _ in 1..arg {
            op /= 10;
        }
        let mode = op % 10;
        match mode {
            0 => {
                // Indirect Mem Access
                let addr = self.mem[self.pc + arg] as usize;
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
                let rx = self.input.as_mut().expect("Input not set up");
                let val = rx.recv().expect("Could not read from input");
                println!("Read {val} from input");
                self.write(1, val);
                2
            }
            4 => {
                // Output
                let val = self.read(1);
                if let Some(tx) = self.output.as_mut() {
                    println!("Sending {val} to output");
                    tx.send(val).expect("Could not write value to output");
                } else {
                    println!("Output to screen: {val}");
                }
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

        let mut prog = Intcode::new("prog", line.as_str());
        let mut input = Intcode::new("input", "104,1,99");
        prog.set_input(&mut input);

        thread::spawn(move || input.run());
        let child = thread::spawn(move || prog.run());
        child.join().unwrap();
    }
}
