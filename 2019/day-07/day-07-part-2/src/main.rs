const INPUT_FILE: &str = if cfg!(debug_assertions) {
    "../input-sample-2.txt"
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
    pre_input: Vec<isize>,
    input: Option<Receiver<isize>>,
    output: Option<Sender<isize>>,
    last_output: Option<isize>,
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
            pre_input: Vec::new(),
            input: None,
            output: None,
            last_output: None,
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

        const LOW: isize = 5;
        const HIGH: isize = 9;

        let mut values = Vec::new();
        let mut ans = isize::MIN;
        for a in LOW..=HIGH {
            values.push(a);
            for b in LOW..=HIGH {
                if values.contains(&b) {
                    continue;
                }
                values.push(b);
                for c in LOW..=HIGH {
                    if values.contains(&c) {
                        continue;
                    }
                    values.push(c);
                    for d in LOW..=HIGH {
                        if values.contains(&d) {
                            continue;
                        }
                        values.push(d);
                        for e in LOW..=HIGH {
                            if values.contains(&e) {
                                continue;
                            }
                            values.push(e);

                            assert!(values.len() == 5);

                            let mut a_prog = Intcode::new("a", line);
                            a_prog.pre_input.push(a);
                            a_prog.pre_input.push(0);
                            let mut b_prog = Intcode::new("b", line);
                            b_prog.pre_input.push(b);
                            let mut c_prog = Intcode::new("c", line);
                            c_prog.pre_input.push(c);
                            let mut d_prog = Intcode::new("d", line);
                            d_prog.pre_input.push(d);
                            let mut e_prog = Intcode::new("e", line);
                            e_prog.pre_input.push(e);

                            b_prog.set_input(&mut a_prog);
                            c_prog.set_input(&mut b_prog);
                            d_prog.set_input(&mut c_prog);
                            e_prog.set_input(&mut d_prog);
                            a_prog.set_input(&mut e_prog);

                            thread::spawn(move || a_prog.run());
                            thread::spawn(move || b_prog.run());
                            thread::spawn(move || c_prog.run());
                            thread::spawn(move || d_prog.run());
                            let e_prog = thread::spawn(move || {
                                e_prog.run();
                                e_prog
                            })
                            .join()
                            .unwrap();
                            let last_output = e_prog.last_output.unwrap();
                            if last_output > ans {
                                ans = last_output
                            }

                            values.remove(values.len() - 1);
                        }
                        values.remove(values.len() - 1);
                    }
                    values.remove(values.len() - 1);
                }
                values.remove(values.len() - 1);
            }
            values.remove(values.len() - 1);
        }
        println!("ans: {ans}");
    }
}
