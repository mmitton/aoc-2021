const INPUT_FILE: &str = if cfg!(debug_assertions) {
    "../input-sample.txt"
} else {
    "../input.txt"
};

use std::collections::{BTreeMap, BTreeSet, VecDeque};
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

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    OxygenSystem,
    Unknown,
    Start,
}

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
struct Coord {
    x: isize,
    y: isize,
}

struct Scanner {
    map: BTreeMap<Coord, Tile>,
    input: Receiver<isize>,
    output: Sender<isize>,
}

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
enum Dir {
    North,
    South,
    West,
    East,
}

impl Dir {
    fn delta(&self) -> Coord {
        match self {
            Self::North => Coord { x: 0, y: -1 },
            Self::South => Coord { x: 0, y: 1 },
            Self::West => Coord { x: -1, y: 0 },
            Self::East => Coord { x: 1, y: 0 },
        }
    }
}

impl From<&Dir> for isize {
    fn from(value: &Dir) -> Self {
        match value {
            Dir::North => 1,
            Dir::South => 2,
            Dir::West => 3,
            Dir::East => 4,
        }
    }
}

impl Scanner {
    fn new(prog: &mut Intcode) -> Self {
        let (in_tx, in_rx) = mpsc::channel();
        let (out_tx, out_rx) = mpsc::channel();

        prog.link_input_output(Some(out_rx), Some(in_tx));
        Self {
            map: BTreeMap::new(),
            input: in_rx,
            output: out_tx,
        }
    }

    fn find_path(&self, from: Coord, look_for: Tile, closest: bool) -> Option<Vec<(Coord, Dir)>> {
        let mut seen = BTreeSet::new();
        seen.insert(from);

        let mut work: VecDeque<(Coord, Vec<(Coord, Dir)>)> = VecDeque::new();
        work.push_front((from, Vec::new()));

        let mut paths = VecDeque::new();

        'work_loop: while let Some((at, path)) = work.pop_front() {
            for dir in [Dir::North, Dir::South, Dir::West, Dir::East] {
                let delta = dir.delta();
                let p = Coord {
                    x: at.x + delta.x,
                    y: at.y + delta.y,
                };

                if seen.contains(&p) {
                    continue;
                }
                seen.insert(p);

                let mut path = path.clone();
                path.push((p, dir));

                match self.map.get(&p) {
                    None => unreachable!(),
                    Some(t) => {
                        if *t == look_for {
                            paths.push_back(path);
                            if closest {
                                break 'work_loop;
                            }
                        } else if *t != Tile::Wall {
                            work.push_back((p, path));
                        }
                    }
                }
            }
        }

        paths.pop_back()
    }

    fn print_map(&self, at: Coord) {
        let mut min = Coord { x: 0, y: 0 };
        let mut max = Coord { x: 0, y: 0 };

        for coord in self.map.keys() {
            if coord.x < min.x {
                min.x = coord.x;
            }
            if coord.x > max.x {
                max.x = coord.x;
            }
            if coord.y < min.y {
                min.y = coord.y;
            }
            if coord.y > max.y {
                max.y = coord.y;
            }
        }

        for y in min.y..=max.y {
            for x in min.x..=max.x {
                if at.x == x && at.y == y {
                    print!("X");
                } else {
                    match self.map.get(&Coord { x, y }) {
                        Some(Tile::Start) => print!("S"),
                        Some(Tile::Empty) => print!("."),
                        Some(Tile::Wall) => print!("#"),
                        Some(Tile::OxygenSystem) => print!("O"),
                        Some(Tile::Unknown) => print!("?"),
                        None => print!(" "),
                    }
                }
            }
            println!();
        }
    }

    fn run_loop(&mut self) {
        macro_rules! insert_unknown {
            ($pos:expr) => {{
                let pos = $pos;
                insert_unknown!(CHECK Coord{x: pos.x - 1, y: pos.y});
                insert_unknown!(CHECK Coord{x: pos.x + 1, y: pos.y});
                insert_unknown!(CHECK Coord{x: pos.x, y: pos.y - 1});
                insert_unknown!(CHECK Coord{x: pos.x, y: pos.y + 1});
            }};

            (CHECK $pos:expr) => {{
                let pos = $pos;
                self.map.entry(pos).or_insert(Tile::Unknown);
            }}
        }

        let mut pos = Coord { x: 0, y: 0 };
        self.map.insert(pos, Tile::Start);
        insert_unknown!(pos);

        while let Some(path) = self.find_path(pos, Tile::Unknown, true) {
            let path_len = path.len();
            for (idx, (coord, dir)) in path.iter().enumerate() {
                self.output.send(dir.into()).unwrap();
                let existing_tile = self.map.get_mut(coord).unwrap();
                let result = self.input.recv().unwrap();

                match result {
                    0 => {
                        assert!(idx == path_len - 1);
                        *existing_tile = Tile::Wall;
                    }
                    1 => {
                        pos = *coord;
                        if *existing_tile == Tile::Unknown {
                            *existing_tile = Tile::Empty;
                            insert_unknown!(coord);
                        }
                    }
                    2 => {
                        pos = *coord;
                        *existing_tile = Tile::OxygenSystem;
                    }
                    _ => unreachable!(),
                }
            }
        }

        self.print_map(pos);
        let path = self
            .find_path(Coord { x: 0, y: 0 }, Tile::OxygenSystem, true)
            .unwrap();
        println!("ans: {}", path.len());
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

        let mut prog = Intcode::new("repair_robot", line);
        let mut scanner = Scanner::new(&mut prog);
        let scanner = std::thread::spawn(move || scanner.run_loop());
        prog.run();

        scanner.join().unwrap();
    }
}
