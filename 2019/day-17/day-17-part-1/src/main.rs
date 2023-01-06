const INPUT_FILE: &str = if cfg!(debug_assertions) {
    "../input-sample.txt"
} else {
    "../input.txt"
};

use std::collections::BTreeSet;
use std::fmt;
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

#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
struct Coord {
    x: isize,
    y: isize,
}

#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl fmt::Display for Dir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Up => write!(f, "^"),
            Self::Right => write!(f, ">"),
            Self::Down => write!(f, "v"),
            Self::Left => write!(f, "<"),
        }
    }
}

struct Robot {
    pos: Coord,
    dir: Dir,
}

struct Scanner {
    input: Receiver<isize>,
    // output: Sender<isize>,
}

impl Scanner {
    fn new(prog: &mut Intcode) -> Self {
        let (in_tx, in_rx) = mpsc::channel();
        // let (out_tx, out_rx) = mpsc::channel();

        prog.link_input_output(None, Some(in_tx));
        Self {
            input: in_rx,
            // output: out_tx,
        }
    }

    fn read_map(&mut self) -> (BTreeSet<Coord>, Robot) {
        let mut y: isize = 0;
        let mut x: isize = -1;
        let mut map: BTreeSet<Coord> = BTreeSet::new();
        let mut robot: Option<Robot> = None;
        while let Ok(c) = self.input.recv() {
            x += 1;
            let c = c as u8 as char;
            print!("{c}");

            let pos = Coord { x, y };
            match c {
                '\n' => {
                    y += 1;
                    x = -1;
                }
                '#' => {
                    map.insert(pos);
                }
                '^' => {
                    map.insert(pos);
                    robot = Some(Robot { pos, dir: Dir::Up });
                }
                '>' => {
                    map.insert(pos);
                    robot = Some(Robot {
                        pos,
                        dir: Dir::Right,
                    });
                }
                'v' => {
                    map.insert(pos);
                    robot = Some(Robot {
                        pos,
                        dir: Dir::Down,
                    });
                }
                '<' => {
                    map.insert(pos);
                    robot = Some(Robot {
                        pos,
                        dir: Dir::Left,
                    });
                }
                '.' => {}
                _ => unreachable!(),
            }
        }

        (map, robot.unwrap())
    }
}

fn print_map(map: &BTreeSet<Coord>, robot: &Robot) -> isize {
    let mut min = Coord {
        x: isize::MAX,
        y: isize::MAX,
    };
    let mut max = Coord {
        x: isize::MIN,
        y: isize::MIN,
    };

    for c in map.iter() {
        if c.x < min.x {
            min.x = c.x
        };
        if c.y < min.y {
            min.y = c.y
        };
        if c.x > max.x {
            max.x = c.x
        };
        if c.y > max.y {
            max.y = c.y
        };
    }

    let mut ans = 0;
    for y in min.y..=max.y {
        for x in min.x..=max.x {
            let pos = Coord { x, y };
            if pos == robot.pos {
                print!("{}", robot.dir);
            } else if map.contains(&pos) {
                if map.contains(&Coord { x: x - 1, y })
                    && map.contains(&Coord { x: x + 1, y })
                    && map.contains(&Coord { x, y: y - 1 })
                    && map.contains(&Coord { x, y: y + 1 })
                {
                    print!("O");
                    ans += x * y;
                } else {
                    print!("#");
                }
            } else {
                print!(" ");
            }
        }
        println!();
    }

    ans
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

        let mut prog = Intcode::new("repair_robot", line);
        let mut scanner = Scanner::new(&mut prog);
        let scanner = std::thread::spawn(move || scanner.read_map());
        prog.run();

        let (map, robot) = scanner.join().unwrap();

        let ans = print_map(&map, &robot);
        println!("ans: {ans}");
    }
}
