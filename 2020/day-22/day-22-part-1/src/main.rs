#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

use std::collections::VecDeque;

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

fn load_input(filename: &str) -> Result<Vec<VecDeque<usize>>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();
    let mut players = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim().to_string();
        if line == "" {
            continue;
        }

        if line.starts_with("Player") {
            players.push(VecDeque::new());
        } else {
            let idx = players.len() - 1;
            players[idx].push_back(line.parse()?);
        }
    }

    Ok(players)
}

fn main() -> Result<(), Error> {
    let mut players = load_input(INPUT_FILE)?;

    for round in 1..usize::MAX {
        println!("-- Round {} --", round);
        for i in 0..players.len() {
            println!("Player {}'s deck: {:?}", i + 1, players[i]);
        }
        let p1 = players[0].pop_front().unwrap();
        let p2 = players[1].pop_front().unwrap();
        println!("Player 1 plays: {}", p1);
        println!("Player 2 plays: {}", p2);

        if p1 > p2 {
            println!("Player 1 wins the round!");
            players[0].push_back(p1);
            players[0].push_back(p2);
        } else {
            println!("Player 2 wins the round!");
            players[1].push_back(p2);
            players[1].push_back(p1);
        }
        println!();

        if players[0].len() == 0 || players[1].len() == 0 {
            break;
        }
    }

    println!("== Post-game results ==");
    let mut answer = 0;
    for i in 0..players.len() {
        println!("Player {}'s deck: {:?}", i + 1, players[i]);
        for j in 0..players[i].len() {
            answer += players[i][j] * (players[i].len() - j);
        }
    }
    println!("Answer: {}", answer);

    Ok(())
}
