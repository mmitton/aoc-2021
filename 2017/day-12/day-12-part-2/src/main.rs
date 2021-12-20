#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NAN(std::num::ParseIntError),
}

#[derive(Debug)]
struct Program {
    num: usize,
    direct: Vec<usize>,
    connected: Vec<usize>,
}

fn load_input(filename: &str) -> Result<Vec<Program>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();
    let mut programs: Vec<Program> = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }
        if line.starts_with("#") {
            continue;
        }

        let parts: Vec<&str> = line.split(" <-> ").collect();
        let mut program = Program {
            num: parts[0].parse().map_err(|e| Error::NAN(e))?,
            direct: Vec::new(),
            connected: Vec::new(),
        };

        for direct in parts[1].split(", ") {
            let direct = direct.parse().map_err(|e| Error::NAN(e))?;
            program.direct.push(direct);
            program.connected.push(direct);
        }

        programs.push(program);
    }

    Ok(programs)
}

fn main() -> Result<(), Error> {
    let mut programs = load_input(INPUT_FILE)?;

    let mut merged = true;
    while merged {
        merged = false;
        for i in 0..programs.len() {
            for j in 0..programs[i].connected.len() {
                for k in 0..programs[programs[i].connected[j]].connected.len() {
                    let num = programs[programs[i].connected[j]].connected[k];
                    if !programs[i].connected.contains(&num) {
                        if cfg!(debug_assertions) {
                            println!("Adding {} to {}", num, i);
                        }
                        programs[i].connected.push(num);
                        merged = true;
                    }
                }
            }
        }
    }

    let mut groups = Vec::new();
    for p in &mut programs {
        p.connected.sort();
        if !groups.contains(&p.connected) {
            if cfg!(debug_assertions) {
                println!("New Group: {:?}", p.connected);
            }
            groups.push(p.connected.clone());
        }
    }

    println!("Total Groups: {}", groups.len());

    Ok(())
}
