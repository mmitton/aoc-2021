#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

use std::collections::BTreeMap;

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NAN(std::num::ParseIntError),
}

#[derive(Debug)]
struct Op {
    dest_reg: String,
    inc: bool,
    delta: isize,
    comp_reg: String,
    comp_op: String,
    comp_imm: isize,
}

fn load_input(filename: &str) -> Result<Vec<Op>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();
    let mut ops: Vec<Op> = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }
        if line.starts_with("#") {
            continue;
        }

        let parts: Vec<&str> = line.split(" ").collect();
        ops.push(Op {
            dest_reg: parts[0].to_string(),
            inc: parts[1] == "inc",
            delta: parts[2].parse().map_err(|e| Error::NAN(e))?,
            comp_reg: parts[4].to_string(),
            comp_op: parts[5].to_string(),
            comp_imm: parts[6].parse().map_err(|e| Error::NAN(e))?,
        });
    }

    Ok(ops)
}

fn main() -> Result<(), Error> {
    let ops = load_input(INPUT_FILE)?;
    let mut registers: BTreeMap<String, isize> = BTreeMap::new();

    let mut max = isize::MIN;
    for op in &ops {
        let mut dest_reg = *registers.get(&op.dest_reg).unwrap_or(&0);
        let comp_reg = *registers.get(&op.comp_reg).unwrap_or(&0);
        let delta = if !op.inc { -op.delta } else { op.delta };

        match op.comp_op.as_str() {
            ">" => {
                if comp_reg > op.comp_imm {
                    dest_reg += delta
                }
            }
            ">=" => {
                if comp_reg >= op.comp_imm {
                    dest_reg += delta
                }
            }
            "<" => {
                if comp_reg < op.comp_imm {
                    dest_reg += delta
                }
            }
            "<=" => {
                if comp_reg <= op.comp_imm {
                    dest_reg += delta
                }
            }
            "==" => {
                if comp_reg == op.comp_imm {
                    dest_reg += delta
                }
            }
            "!=" => {
                if comp_reg != op.comp_imm {
                    dest_reg += delta
                }
            }
            _ => unimplemented!("What op is this? '{}'", op.comp_op),
        }

        if dest_reg > max {
            max = dest_reg;
        }
        registers.insert(op.dest_reg.clone(), dest_reg);
        if cfg!(debug_assertions) {
            println!("Registers : {:?}", registers);
        }
    }

    println!("Max during run: {}", max);

    Ok(())
}
