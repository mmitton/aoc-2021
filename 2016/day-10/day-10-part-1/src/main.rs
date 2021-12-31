#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    InvalidInst(String),
    NAN(std::num::ParseIntError),
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Error {
        Error::NAN(e)
    }
}

fn load_input(filename: &str) -> Result<(usize, usize, usize, Vec<Inst>), Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();
    let mut insts = Vec::new();
    let mut max_bot = 0;
    let mut max_input = 0;
    let mut max_output = 0;

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim().to_string();
        if line == "" {
            continue;
        }

        let parts: Vec<&str> = line.split(" ").collect();

        let inst = match parts[0] {
            "value" => {
                assert!(parts[4] == "bot");
                let input = parts[1].parse()?;
                if input > max_input {
                    max_input = input;
                }
                let bot = parts[5].parse()?;
                if bot > max_bot {
                    max_bot = bot;
                }
                Inst::Input(input, bot)
            }
            "bot" => {
                assert!(parts[3] == "low");
                assert!(parts[8] == "high");
                let bot = parts[1].parse()?;
                if bot > max_bot {
                    max_bot = bot;
                }

                let low = match parts[5] {
                    "bot" => {
                        let bot = parts[6].parse()?;
                        if bot > max_bot {
                            max_bot = bot;
                        }
                        GiveTo::Bot(bot)
                    }
                    "output" => {
                        let output = parts[6].parse()?;
                        if output > max_output {
                            max_output = output;
                        }
                        GiveTo::Output(output)
                    }
                    _ => return Err(Error::InvalidInst(line.to_string())),
                };
                let high = match parts[10] {
                    "bot" => {
                        let bot = parts[11].parse()?;
                        if bot > max_bot {
                            max_bot = bot;
                        }
                        GiveTo::Bot(bot)
                    }
                    "output" => {
                        let output = parts[11].parse()?;
                        if output > max_output {
                            max_output = output;
                        }
                        GiveTo::Output(output)
                    }
                    _ => return Err(Error::InvalidInst(line.to_string())),
                };
                Inst::Bot(bot, low, high)
            }
            _ => return Err(Error::InvalidInst(line.to_string())),
        };

        insts.push(inst);
    }

    Ok((max_bot, max_input, max_output, insts))
}

#[derive(Debug, Clone)]
enum GiveTo {
    None,
    Bot(usize),
    Output(usize),
}

#[derive(Debug)]
enum Inst {
    Input(usize, usize),
    Bot(usize, GiveTo, GiveTo),
}

#[derive(Debug, Clone)]
struct Bot {
    holding: Vec<usize>,
    low: GiveTo,
    high: GiveTo,
}

fn main() -> Result<(), Error> {
    let (max_bot, max_input, max_output, insts) = load_input(INPUT_FILE)?;

    let mut bots = vec![
        Bot {
            holding: Vec::new(),
            low: GiveTo::None,
            high: GiveTo::None,
        };
        max_bot + 1
    ];

    let mut outputs: Vec<Vec<usize>> = vec![Vec::new(); max_output + 1];

    for inst in &insts {
        match inst {
            Inst::Input(value, bot) => {
                bots[*bot].holding.push(*value);
                bots[*bot].holding.sort();
            }
            Inst::Bot(bot, low, high) => {
                bots[*bot].low = low.clone();
                bots[*bot].high = high.clone();
            }
        }
    }

    let mut work_queue = std::collections::VecDeque::new();
    for i in 0..bots.len() {
        if bots[i].holding.len() == 2 {
            work_queue.push_back(i);
        }
    }

    let mut answer = None;
    while work_queue.len() > 0 {
        let idx = work_queue.pop_front().unwrap();
        println!("Working on {} {:?}", idx, bots[idx]);
        assert!(bots[idx].holding.len() == 2);
        let low = bots[idx].holding[0];
        let high = bots[idx].holding[1];

        if low == 17 && high == 61 {
            answer = Some(idx);
            break;
        }

        match bots[idx].low {
            GiveTo::None => unreachable!(),
            GiveTo::Bot(to) => {
                bots[to].holding.push(low);
                bots[to].holding.sort();
                if bots[to].holding.len() >= 2 {
                    work_queue.push_back(to);
                }
            }
            GiveTo::Output(to) => {
                outputs[to].push(low);
                outputs[to].sort();
            }
        }
        match bots[idx].high {
            GiveTo::None => unreachable!(),
            GiveTo::Bot(to) => {
                bots[to].holding.push(high);
                bots[to].holding.sort();
                if bots[to].holding.len() >= 2 {
                    work_queue.push_back(to);
                }
            }
            GiveTo::Output(to) => {
                outputs[to].push(high);
                outputs[to].sort();
            }
        }
    }

    match answer {
        None => println!("Answer: None"),
        Some(answer) => println!("Answer: {}", answer),
    }

    Ok(())
}
