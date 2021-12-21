#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NAN(std::num::ParseIntError),
    NotAnInstruction(String),
}

#[derive(Debug)]
enum Instruction {
    Spin(usize),
    Exchange(usize, usize),
    Partner(usize, usize),
}

impl TryFrom<&str> for Instruction {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.starts_with("s") {
            Ok(Instruction::Spin(
                s[1..].parse().map_err(|e| Error::NAN(e))?,
            ))
        } else if s.starts_with("x") {
            let parts: Vec<&str> = s[1..].split("/").collect();
            Ok(Instruction::Exchange(
                parts[0].parse().map_err(|e| Error::NAN(e))?,
                parts[1].parse().map_err(|e| Error::NAN(e))?,
            ))
        } else if s.starts_with("p") {
            let chars: Vec<char> = s.chars().collect();
            Ok(Instruction::Partner(
                (chars[1] as u8 - 'a' as u8) as usize,
                (chars[3] as u8 - 'a' as u8) as usize,
            ))
        } else {
            Err(Error::NotAnInstruction(s.to_string()))
        }
    }
}

fn load_input(filename: &str) -> Result<(Vec<usize>, Vec<Instruction>), Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;
    let buf = BufReader::new(f);
    let lines: Vec<String> = buf
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    let mut instructions: Vec<Instruction> = Vec::new();
    for instruction in lines[1].split(",") {
        instructions.push(instruction.try_into()?);
    }

    let mut programs = Vec::new();
    for p in lines[0].chars() {
        programs.push((p as u8 - 'a' as u8) as usize);
    }

    Ok((programs, instructions))
}

fn print_order(programs: &Vec<usize>, start_idx: usize) {
    for i in 0..programs.len() {
        print!(
            "{}",
            (programs[(start_idx + i) % programs.len()] as u8 + 'a' as u8) as char
        );
    }
}

fn program_str(programs: &Vec<usize>, start_idx: usize) -> String {
    let mut program_str = String::new();
    for i in 0..programs.len() {
        program_str.push((programs[(start_idx + i) % programs.len()] as u8 + 'a' as u8) as char);
    }
    program_str
}

fn main() -> Result<(), Error> {
    let (mut programs, instructions) = load_input(INPUT_FILE)?;
    let mut start_idx = 0usize;

    print_order(&programs, start_idx);
    println!();

    if cfg!(debug_assertions) {
        println!("instructions: {:?}", instructions);
    }
    let programs_count = programs.len();

    let mut program_at = Vec::new();
    for i in 0..programs_count {
        program_at.push(i);
    }

    const ITERS: usize = 1000000000;
    const DEBUG_PRINT: usize = ITERS / 100000;

    let mut previous: Vec<String> = Vec::new();
    previous.push(program_str(&programs, start_idx));

    let mut iter = 0;
    loop {
        iter += 1;
        if iter % DEBUG_PRINT == 0 {
            println!("{}: {} left", iter, ITERS - iter);
        }
        for instruction in &instructions {
            match instruction {
                Instruction::Spin(size) => {
                    start_idx += programs_count - *size;
                }
                Instruction::Exchange(p1, p2) => {
                    let programs_1 = (*p1 + start_idx) % programs_count;
                    let programs_2 = (*p2 + start_idx) % programs_count;
                    let program_at_1 = programs[programs_1];
                    let program_at_2 = programs[programs_2];

                    programs.swap(programs_1, programs_2);
                    program_at.swap(program_at_1, program_at_2);
                }
                Instruction::Partner(c1, c2) => {
                    let program_at_1 = *c1;
                    let program_at_2 = *c2;
                    let programs_1 = program_at[program_at_1];
                    let programs_2 = program_at[program_at_2];

                    programs.swap(programs_1, programs_2);
                    program_at.swap(program_at_1, program_at_2);
                }
            }
        }

        let new_previous = program_str(&programs, start_idx);

        for i in 0..previous.len() {
            if previous[i] == new_previous {
                let cycle = &previous[i..];
                println!("cur: {}", new_previous);
                println!("cycle: {:?}", cycle);
                println!("Found cycle from {} .. {}", i, iter);
                let idx = (ITERS - i) % cycle.len();
                println!("{}", cycle[idx]);
                return Ok(());
            }
        }

        previous.push(new_previous);
    }
}
