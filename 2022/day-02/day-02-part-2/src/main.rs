#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum RoundResult {
    Win,
    Lose,
    Draw,
}

impl Play {
    fn points(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

impl RoundResult {
    fn points(&self) -> usize {
        match self {
            Self::Win => 6,
            Self::Lose => 0,
            Self::Draw => 3,
        }
    }
}

fn round(p1: Play, p2: Play) -> usize {
    let result = match p1 {
        Play::Rock => match p2 {
            Play::Rock => RoundResult::Draw,
            Play::Paper => RoundResult::Win,
            Play::Scissors => RoundResult::Lose,
        },
        Play::Paper => match p2 {
            Play::Rock => RoundResult::Lose,
            Play::Paper => RoundResult::Draw,
            Play::Scissors => RoundResult::Win,
        },
        Play::Scissors => match p2 {
            Play::Rock => RoundResult::Win,
            Play::Paper => RoundResult::Lose,
            Play::Scissors => RoundResult::Draw,
        },
    };

    let points = result.points() + p2.points();

    println!(
        "p1:{:?}  p2:{:?}  results:{:?}  {}+{} = {}",
        p1,
        p2,
        result,
        result.points(),
        p2.points(),
        points,
    );

    points
}

fn main() {
    let lines = {
        let file = File::open(INPUT_FILE).expect("Cannot open input file");
        BufReader::new(file).lines()
    };

    let mut score = 0;
    for line in lines {
        match line {
            Ok(line) => {
                if line.trim() == "" {
                    continue;
                } else {
                    let (p1, p2) = line.split_once(' ').unwrap();
                    let p1 = match p1 {
                        "A" => Play::Rock,
                        "B" => Play::Paper,
                        "C" => Play::Scissors,
                        _ => unreachable!(),
                    };
                    let p2 = match p2 {
                        "X" => {
                            // Need to lose
                            match p1 {
                                Play::Rock => Play::Scissors,
                                Play::Paper => Play::Rock,
                                Play::Scissors => Play::Paper,
                            }
                        }
                        "Y" => {
                            // Need to draw
                            match p1 {
                                Play::Rock => Play::Rock,
                                Play::Paper => Play::Paper,
                                Play::Scissors => Play::Scissors,
                            }
                        }
                        "Z" => {
                            // Need to win
                            match p1 {
                                Play::Rock => Play::Paper,
                                Play::Paper => Play::Scissors,
                                Play::Scissors => Play::Rock,
                            }
                        }

                        _ => unreachable!(),
                    };
                    score += round(p1, p2);
                }
            }
            Err(e) => panic!("{}", e),
        }
    }

    println!("Score: {}", score);
}
