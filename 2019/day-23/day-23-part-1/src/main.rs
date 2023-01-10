const INPUT_FILE: &str = if cfg!(debug_assertions) {
    "../input-sample.txt"
} else {
    "../input.txt"
};

use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Intcode {
    name: String,
    mem: Vec<isize>,
    pc: usize,
    halted: bool,
    input: VecDeque<isize>,
    output: VecDeque<isize>,
    debug: bool,
    relative_offset_base: isize,
}

impl Intcode {
    fn new(name: String, code: &str) -> Self {
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
            input: VecDeque::new(),
            output: VecDeque::new(),
            debug: false,
            relative_offset_base: 0,
        }
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
                let val = if let Some(val) = self.input.pop_front() {
                    val
                } else {
                    -1
                };
                self.write(1, val);
                2
            }
            4 => {
                // Output
                let val = self.read(1);
                self.output.push_back(val);
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

struct Switch {
    devices: Vec<Intcode>,
}

impl Switch {
    fn new(mut devices: Vec<Intcode>) -> Self {
        for (idx, device) in devices.iter_mut().enumerate() {
            device.input.push_back(idx as isize);
        }

        Self { devices }
    }

    fn run(&mut self) -> isize {
        loop {
            for i in 0..self.devices.len() {
                self.devices[i].tick();
                while self.devices[i].output.len() >= 3 {
                    let addr = self.devices[i].output.pop_front().unwrap() as usize;
                    let x = self.devices[i].output.pop_front().unwrap();
                    let y = self.devices[i].output.pop_front().unwrap();
                    if addr == 255 {
                        println!("x: {x}  y: {y}");
                        return y;
                    } else {
                        println!("Sending packet from {i} to {addr}  x:{x} y:{y}");
                        self.devices[addr].input.push_back(x);
                        self.devices[addr].input.push_back(y);
                    }
                }
            }
        }
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

        let mut devices = Vec::new();
        for addr in 0..50 {
            let device = Intcode::new(format!("device {addr}"), line);
            devices.push(device);
        }

        let mut switch = Switch::new(devices);
        let ans = switch.run();

        println!("ans: {ans}");
    }
}
