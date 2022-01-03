#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NAN(std::num::ParseIntError),
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Error {
        Error::NAN(e)
    }
}

#[derive(Debug)]
enum Op {
    SwapPosition(usize, usize),
    SwapLetter(char, char),
    RotateSteps(isize),
    RotateLetterPos(char),
    Reverse(usize, usize),
    Move(usize, usize),
    Check(Vec<char>),
}

fn load_input(filename: &str) -> Result<(Vec<Vec<char>>, Vec<Vec<char>>, Vec<Op>), Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();
    let mut password = Vec::new();
    let mut scrambled = Vec::new();
    let mut ops = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim().to_string();
        if line == "" {
            continue;
        }

        let parts: Vec<&str> = line.split(" ").collect();
        if parts[0] == "password" {
            password.push(parts[1].chars().collect());
        } else if parts[0] == "scrambled" {
            scrambled.push(parts[1].chars().collect());
        } else {
            let op = match parts[0] {
                "check" => Op::Check(parts[1].chars().collect()),
                "swap" => match parts[1] {
                    "position" => Op::SwapPosition(parts[2].parse()?, parts[5].parse()?),
                    "letter" => Op::SwapLetter(
                        parts[2].chars().nth(0).unwrap(),
                        parts[5].chars().nth(0).unwrap(),
                    ),
                    _ => unreachable!(),
                },
                "rotate" => match parts[1] {
                    "left" => Op::RotateSteps(-parts[2].parse()?),
                    "right" => Op::RotateSteps(parts[2].parse()?),
                    "based" => Op::RotateLetterPos(parts[6].chars().nth(0).unwrap()),
                    _ => unreachable!(),
                },
                "reverse" => Op::Reverse(parts[2].parse()?, parts[4].parse()?),
                "move" => Op::Move(parts[2].parse()?, parts[5].parse()?),
                _ => unreachable!(),
            };
            ops.push(op);
        }
    }

    Ok((password, scrambled, ops))
}

fn scramble(password: &Vec<char>, ops: &Vec<Op>) -> String {
    let mut password = password.clone();

    for op in ops {
        match op {
            Op::Check(c) => {
                assert!(c == &password);
            }
            Op::SwapPosition(p0, p1) => {
                password.swap(*p0, *p1);
            }
            Op::SwapLetter(c0, c1) => {
                let mut p0 = usize::MAX;
                let mut p1 = usize::MAX;
                for i in 0..password.len() {
                    if password[i] == *c0 {
                        p0 = i;
                    }
                    if password[i] == *c1 {
                        p1 = i;
                    }
                }

                assert!(p0 != usize::MAX);
                assert!(p1 != usize::MAX);
                password.swap(p0, p1);
            }
            Op::RotateSteps(steps) => {
                if *steps < 0 {
                    password.rotate_left((-(*steps)) as usize);
                } else {
                    password.rotate_right(*steps as usize);
                }
            }
            Op::RotateLetterPos(c) => {
                for i in 0..password.len() {
                    if password[i] == *c {
                        password.rotate_right(i);
                        password.rotate_right(1);
                        if i >= 4 {
                            password.rotate_right(1);
                        }
                        break;
                    }
                }
            }
            Op::Reverse(c0, c1) => {
                let c0 = *c0;
                let c1 = *c1;
                let width = c1 - c0 + 1;

                for i in 0..width / 2 {
                    password.swap(c0 + i, c1 - i);
                }
            }
            Op::Move(from, to) => {
                let c = password.remove(*from);
                password.insert(*to, c);
            }
        }
        // if cfg!(debug_assertions) {
        //     println!("{:?} => {}", op, password.iter().collect::<String>());
        // }
    }

    password.iter().collect::<String>()
}

fn unscramble(password: &Vec<char>, ops: &Vec<Op>) -> String {
    let mut password = password.clone();

    let mut letter_rotations = vec![usize::MAX; password.len()];
    for i in 0..letter_rotations.len() {
        let mut rotate = i + 1;
        if i >= 4 {
            rotate += 1
        };
        rotate %= password.len();
        letter_rotations[(i + rotate) % password.len()] = rotate;
    }

    for op in ops.iter().rev() {
        match op {
            Op::Check(c) => {
                assert!(c == &password);
            }
            Op::SwapPosition(p0, p1) => {
                password.swap(*p0, *p1);
            }
            Op::SwapLetter(c0, c1) => {
                let mut p0 = usize::MAX;
                let mut p1 = usize::MAX;
                for i in 0..password.len() {
                    if password[i] == *c0 {
                        p0 = i;
                    }
                    if password[i] == *c1 {
                        p1 = i;
                    }
                }

                assert!(p0 != usize::MAX);
                assert!(p1 != usize::MAX);
                password.swap(p0, p1);
            }
            Op::RotateSteps(steps) => {
                if *steps < 0 {
                    password.rotate_right((-(*steps)) as usize);
                } else {
                    password.rotate_left(*steps as usize);
                }
            }
            Op::RotateLetterPos(c) => {
                for i in 0..password.len() {
                    if password[i] == *c {
                        password.rotate_left(letter_rotations[i]);
                        break;
                    }
                }
            }
            Op::Reverse(c0, c1) => {
                let c0 = *c0;
                let c1 = *c1;
                let width = c1 - c0 + 1;

                for i in 0..width / 2 {
                    password.swap(c0 + i, c1 - i);
                }
            }
            Op::Move(from, to) => {
                let c = password.remove(*to);
                password.insert(*from, c);
            }
        }
        if cfg!(debug_assertions) {
            println!("{:?} => {}", op, password.iter().collect::<String>());
        }
    }

    password.iter().collect::<String>()
}

fn main() -> Result<(), Error> {
    let (passwords, scrambles, ops) = load_input(INPUT_FILE)?;

    for password in &passwords {
        println!(
            "Password: {} => {}",
            password.iter().collect::<String>(),
            scramble(password, &ops)
        );
    }

    for scrambled in &scrambles {
        println!(
            "Scrambled: {} => {}",
            scrambled.iter().collect::<String>(),
            unscramble(scrambled, &ops)
        );
    }

    Ok(())
}
