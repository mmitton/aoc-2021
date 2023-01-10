const INPUT_FILE: &str = if cfg!(debug_assertions) {
    "../input-sample.txt"
} else {
    "../input.txt"
};

use std::collections::BTreeMap;
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
    fn link_input_program(&mut self, input: &mut Self) {
        assert!(self.input.is_none());
        assert!(input.output.is_none());

        let (tx, rx) = mpsc::channel();
        self.input = Some(rx);
        input.output = Some(tx);
    }

    #[allow(dead_code)]
    fn link_input_output(&mut self, input: Option<Receiver<isize>>, output: Option<Sender<isize>>) {
        self.input = input;
        self.output = output;
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
        // Drop input / output channels
        self.input.take();
        self.output.take();
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

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
enum Color {
    Black,
    White,
}

#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn turn(&mut self, turn: isize) -> Self {
        match (self, turn) {
            (Self::Up, 0) => Self::Left,
            (Self::Up, 1) => Self::Right,
            (Self::Right, 0) => Self::Up,
            (Self::Right, 1) => Self::Down,
            (Self::Down, 0) => Self::Right,
            (Self::Down, 1) => Self::Left,
            (Self::Left, 0) => Self::Down,
            (Self::Left, 1) => Self::Up,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
struct Point(isize, isize);

impl Point {
    fn move_step(&mut self, d: Dir) {
        match d {
            Dir::Up => self.1 -= 1,
            Dir::Right => self.0 += 1,
            Dir::Down => self.1 += 1,
            Dir::Left => self.0 -= 1,
        }
    }
}

impl From<isize> for Color {
    fn from(value: isize) -> Self {
        match value {
            0 => Self::Black,
            1 => Self::White,
            _ => unreachable!(),
        }
    }
}

impl From<Color> for isize {
    fn from(value: Color) -> Self {
        match value {
            Color::Black => 0,
            Color::White => 1,
        }
    }
}

struct Hull {
    input: Receiver<isize>,
    output: Sender<isize>,
    painted_tiles: BTreeMap<Point, Color>,
    pos: Point,
    dir: Dir,
}

impl Hull {
    fn new(prog: &mut Intcode) -> Self {
        let (in_tx, in_rx) = mpsc::channel();
        let (out_tx, out_rx) = mpsc::channel();

        prog.link_input_output(Some(out_rx), Some(in_tx));
        Self {
            input: in_rx,
            output: out_tx,
            painted_tiles: BTreeMap::new(),
            pos: Point(0, 0),
            dir: Dir::Up,
        }
    }

    fn run_loop(&mut self) {
        self.output.send(Color::White.into()).unwrap();

        while let Ok(c) = self.input.recv() {
            let c: Color = c.into();
            self.painted_tiles.insert(self.pos, c);
            let turn = self.input.recv().unwrap();
            self.dir = self.dir.turn(turn);
            self.pos.move_step(self.dir);

            let c = *self.painted_tiles.get(&self.pos).unwrap_or(&Color::Black);
            if self.output.send(c.into()).is_err() {
                break;
            }
        }

        let mut min = (isize::MAX, isize::MAX);
        let mut max = (isize::MIN, isize::MIN);
        for k in self.painted_tiles.keys() {
            if k.0 < min.0 {
                min.0 = k.0;
            }
            if k.0 > max.0 {
                max.0 = k.0;
            }
            if k.1 < min.1 {
                min.1 = k.1;
            }
            if k.1 > max.1 {
                max.1 = k.1;
            }
        }

        for y in min.1..=max.1 {
            for x in min.0..=max.0 {
                let c = *self
                    .painted_tiles
                    .get(&Point(x, y))
                    .unwrap_or(&Color::Black);
                match c {
                    Color::White => print!("#"),
                    Color::Black => print!(" "),
                }
            }
            println!();
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

        let mut prog = Intcode::new("hull_painter", line);
        let mut hull = Hull::new(&mut prog);
        let hull = std::thread::spawn(move || hull.run_loop());
        prog.run();

        hull.join().unwrap();
    }
}