#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NAN(std::num::ParseIntError),
}

#[derive(Debug)]
struct Step {
    write: u8,
    pos_delta: isize,
    next_state: char,
}

#[derive(Debug)]
struct State {
    steps: [Step; 2],
}

fn load_input(filename: &str) -> Result<(char, usize, BTreeMap<char, State>), Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;
    let lines = BufReader::new(f).lines();
    let lines: Vec<String> = lines.map(|l| l.expect("Could not parse line")).collect();

    let mut states: BTreeMap<char, State> = BTreeMap::new();
    let current_state = lines[0].chars().nth(15).unwrap();
    let num_steps = (lines[1].split(" ").collect::<Vec<&str>>())[5]
        .parse()
        .map_err(|e| Error::NAN(e))?;
    println!("Current State: '{}'", current_state);

    let mut line = 3;
    while line < lines.len() {
        let state_name = lines[line].chars().nth(9).unwrap();
        let state = State {
            steps: [
                Step {
                    write: if &lines[line + 2][22..23] == "0" {
                        0
                    } else {
                        1
                    },
                    pos_delta: if &lines[line + 3][27..] == "right." {
                        1
                    } else {
                        -1
                    },
                    next_state: lines[line + 4].chars().nth(26).unwrap(),
                },
                Step {
                    write: if &lines[line + 6][22..23] == "0" {
                        0
                    } else {
                        1
                    },
                    pos_delta: if &lines[line + 7][27..] == "right." {
                        1
                    } else {
                        -1
                    },
                    next_state: lines[line + 8].chars().nth(26).unwrap(),
                },
            ],
        };
        states.insert(state_name, state);

        line += 10;
    }

    Ok((current_state, num_steps, states))
}

fn main() -> Result<(), Error> {
    let (mut cur_state, num_steps, states) = load_input(INPUT_FILE)?;
    let mut pos = 0isize;
    let mut tape: BTreeSet<isize> = BTreeSet::new();

    for _ in 0..num_steps {
        let cur_num = if tape.contains(&pos) { 1 } else { 0 };
        let state = states.get(&cur_state).unwrap();

        if state.steps[cur_num].write == 0 {
            tape.remove(&pos);
        } else {
            tape.insert(pos);
        }
        pos += state.steps[cur_num].pos_delta;
        cur_state = state.steps[cur_num].next_state;
    }

    println!("tape: {}", tape.len());

    Ok(())
}
