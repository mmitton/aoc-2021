const INPUT_FILE: &str = if cfg!(debug_assertions) {
    "../input-sample.txt"
} else {
    "../input.txt"
};

use std::collections::BTreeSet;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;
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
struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    fn dir_to(&self, rhs: &Self) -> Dir {
        assert!(self.x == rhs.x || self.y == rhs.y);
        assert!(self.x != rhs.x || self.y != rhs.y);

        if self.x < rhs.x {
            Dir::Right
        } else if self.x > rhs.x {
            Dir::Left
        } else if self.y < rhs.y {
            Dir::Down
        } else if self.y > rhs.y {
            Dir::Up
        } else {
            unreachable!();
        }
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
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

#[derive(Debug)]
struct Robot {
    pos: Coord,
    dir: Dir,
}

struct Path {
    start: Coord,
    end: Coord,
    x_range: RangeInclusive<isize>,
    y_range: RangeInclusive<isize>,
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} => {}", self.start, self.end)
    }
}

impl Path {
    fn new(mut start: Coord, mut end: Coord) -> Self {
        assert!(start.x == end.x || start.y == end.y);
        assert!(start.x != end.x || start.y != end.y);
        if start.x > end.x {
            std::mem::swap(&mut start, &mut end);
        }
        if start.y > end.y {
            std::mem::swap(&mut start, &mut end);
        }
        let x_range = RangeInclusive::new(start.x, end.x);
        let y_range = RangeInclusive::new(start.y, end.y);
        Self {
            start,
            end,
            x_range,
            y_range,
        }
    }

    fn contains(&self, c: &Coord) -> bool {
        self.x_range.contains(&c.x) && self.y_range.contains(&c.y)
    }
}

#[derive(Clone, PartialEq, Eq)]
enum Command {
    Left,
    Right,
    Move(usize),
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Left => write!(f, "L"),
            Self::Right => write!(f, "R"),
            Self::Move(cnt) => write!(f, "{cnt}"),
        }
    }
}

#[derive(PartialEq, Eq)]
struct Commands(Vec<Command>);

impl Commands {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn add(&mut self, robot: &mut Robot, next: Coord) {
        let next_dir = robot.pos.dir_to(&next);

        let steps = ((robot.pos.x - next.x).abs() + (robot.pos.y - next.y).abs()) as usize;
        if next_dir != robot.dir {
            // Turn and then move
            self.0.push(match (robot.dir, next_dir) {
                (Dir::Up, Dir::Right) => Command::Right,
                (Dir::Up, Dir::Left) => Command::Left,
                (Dir::Right, Dir::Down) => Command::Right,
                (Dir::Right, Dir::Up) => Command::Left,
                (Dir::Down, Dir::Left) => Command::Right,
                (Dir::Down, Dir::Right) => Command::Left,
                (Dir::Left, Dir::Up) => Command::Right,
                (Dir::Left, Dir::Down) => Command::Left,
                _ => unreachable!(),
            });
        }

        // Move
        if let Some(Command::Move(cmd)) = &mut self.0.last_mut() {
            *cmd += steps;
        } else {
            self.0.push(Command::Move(steps));
        }

        robot.dir = next_dir;
        robot.pos = next;
    }

    fn split(&self) -> [String; 4] {
        let mut groups = Vec::new();
        for i in (0..self.0.len()).step_by(2) {
            for j in (i + 2..self.0.len()).step_by(2) {
                let c = Commands(Vec::from(&self.0[i..j]));
                let s = format!("{c}");
                if s.len() > 20 {
                    break;
                }
                if !groups.contains(&c) {
                    groups.push(c);
                }
            }
        }

        groups.sort_by_key(|v| format!("{v}").len());
        groups.reverse();

        for a in 0..groups.len() {
            for b in a + 1..groups.len() {
                'search_loop: for c in b + 1..groups.len() {
                    let a_cmd = &groups[a].0;
                    let b_cmd = &groups[b].0;
                    let c_cmd = &groups[c].0;
                    let mut res_str = Vec::new();
                    let mut res = Vec::new();
                    while res.len() != self.0.len() {
                        if a_cmd.len() + res.len() <= self.0.len()
                            && &self.0[res.len()..res.len() + a_cmd.len()] == a_cmd
                        {
                            res_str.push("A");
                            res.extend_from_slice(a_cmd);
                            continue;
                        }
                        if b_cmd.len() + res.len() <= self.0.len()
                            && &self.0[res.len()..res.len() + b_cmd.len()] == b_cmd
                        {
                            res_str.push("B");
                            res.extend_from_slice(b_cmd);
                            continue;
                        }
                        if c_cmd.len() + res.len() <= self.0.len()
                            && &self.0[res.len()..res.len() + c_cmd.len()] == c_cmd
                        {
                            res_str.push("C");
                            res.extend_from_slice(c_cmd);
                            continue;
                        }

                        continue 'search_loop;
                    }

                    let res_str = res_str.join(",");
                    if res_str.len() <= 20 {
                        return [
                            res_str,
                            format!("{}", groups[a]),
                            format!("{}", groups[b]),
                            format!("{}", groups[c]),
                        ];
                    }
                }
            }
        }

        panic!();
    }
}

impl fmt::Display for Commands {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (idx, cmd) in self.0.iter().enumerate() {
            if idx != 0 {
                write!(f, ",{cmd}")?;
            } else {
                write!(f, "{cmd}")?;
            }
        }
        Ok(())
    }
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

    fn read_map(&mut self) -> (Robot, Vec<Path>) {
        let mut max_y: isize = 0;
        let mut max_x: isize = -1;
        let mut y: isize = 0;
        let mut x: isize = -1;
        let mut map: BTreeSet<Coord> = BTreeSet::new();
        let mut robot: Option<Robot> = None;
        while let Ok(c) = self.input.recv() {
            x += 1;
            let c = c as u8 as char;
            print!("{c}");

            if max_x < x {
                max_x = x;
            }
            if max_y < y {
                max_y = y;
            }

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

        let mut paths: Vec<Path> = Vec::new();
        // Find horizontal paths
        for y in 0..=max_y {
            let mut start = None;
            let mut end = None;
            for x in 0..=max_x {
                let pos = Coord { x, y };
                if map.contains(&pos) {
                    if start.is_none() {
                        start = Some(pos);
                    } else {
                        end = Some(pos);
                        // Check to see if we are at an intersection
                        if map.contains(&Coord { x, y: y - 1 })
                            || map.contains(&Coord { x, y: y + 1 })
                        {
                            paths.push(Path::new(start.unwrap(), pos));
                            start = Some(pos);
                            end = None;
                        }
                    }
                } else if let (Some(start), Some(end)) = (start.take(), end.take()) {
                    paths.push(Path::new(start, end));
                }
            }
        }
        // Find vertical paths
        for x in 0..=max_x {
            let mut start = None;
            let mut end = None;
            for y in 0..=max_y {
                let pos = Coord { x, y };
                if map.contains(&pos) {
                    if start.is_none() {
                        start = Some(pos);
                    } else {
                        end = Some(pos);
                        // Check to see if we are at an intersection
                        if map.contains(&Coord { x: x - 1, y })
                            || map.contains(&Coord { x: x + 1, y })
                        {
                            paths.push(Path::new(start.unwrap(), pos));
                            start = Some(pos);
                            end = None;
                        }
                    }
                } else if let (Some(start), Some(end)) = (start.take(), end.take()) {
                    paths.push(Path::new(start, end));
                }
            }
        }

        (robot.unwrap(), paths)
    }
}

fn print_map(paths: &[Path], robot: &Robot) -> isize {
    let mut min = Coord {
        x: isize::MAX,
        y: isize::MAX,
    };
    let mut max = Coord {
        x: isize::MIN,
        y: isize::MIN,
    };

    for path in paths.iter() {
        if path.start.x < min.x {
            min.x = path.start.x
        };
        if path.start.y < min.y {
            min.y = path.start.y
        };
        if path.end.x > max.x {
            max.x = path.end.x
        };
        if path.end.y > max.y {
            max.y = path.end.y
        };
    }

    let mut ans = 0;
    for y in min.y..=max.y {
        for x in min.x..=max.x {
            let pos = Coord { x, y };
            if pos == robot.pos {
                print!("{}", robot.dir);
            } else {
                let mut path_cnt = 0;
                for path in paths.iter() {
                    if path.contains(&pos) {
                        path_cnt += 1;
                    }
                }
                if path_cnt == 0 {
                    print!(" ");
                } else if path_cnt <= 2 {
                    print!("#");
                } else {
                    print!("O");
                    ans += x * y;
                }
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

        let mut prog = Intcode::new("cleaning_robot", line);
        let mut scanner = Scanner::new(&mut prog);
        let scanner = std::thread::spawn(move || scanner.read_map());
        prog.run();

        let (mut robot, paths) = scanner.join().unwrap();

        print_map(&paths, &robot);

        let mut used_paths = Vec::new();
        let mut commands = Commands::new();
        while used_paths.len() != paths.len() {
            // Find the next path to travel down
            let mut next = None;
            for (idx, path) in paths.iter().enumerate() {
                if used_paths.contains(&idx) {
                    continue;
                }
                if path.start == robot.pos
                    && (next.is_none() || robot.pos.dir_to(&path.end) == robot.dir)
                {
                    next = Some((idx, path.end));
                }
                if path.end == robot.pos
                    && (next.is_none() || robot.pos.dir_to(&path.start) == robot.dir)
                {
                    next = Some((idx, path.start));
                }
            }

            if let Some((idx, next)) = next {
                used_paths.push(idx);
                commands.add(&mut robot, next);
            } else {
                panic!("Could not find next path to take");
            }
        }
        println!("Commands: {commands}");

        let commands = commands.split();
        let mut input: Vec<isize> = Vec::new();
        for command in commands {
            println!("{:>2} : {command}", command.len());
            for c in command.chars() {
                input.push(c as u32 as isize);
            }
            input.push('\n' as u32 as isize);
        }
        input.push('n' as u32 as isize);
        input.push('\n' as u32 as isize);

        let mut prog = Intcode::new("cleaning_robot", line);
        prog.mem[0] = 2;
        prog.pre_input = input;
        prog.run();
        println!("ans: {}", prog.last_output.unwrap());
    }
}
