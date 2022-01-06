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

fn play_game(players: &mut Vec<VecDeque<usize>>, max_game: &mut usize) -> usize {
    *max_game += 1;
    let cur_game = *max_game;
    if cfg!(debug_assertions) {
        println!("=== Game {} ===", cur_game);
    }
    let mut seen = Vec::new();

    for round in 1..usize::MAX {
        if cfg!(debug_assertions) {
            println!("-- Round {} --", round);
            for i in 0..players.len() {
                println!("Player {}'s deck: {:?}", i + 1, players[i]);
            }
        }

        if seen.contains(players) {
            if cfg!(debug_assertions) {
                println!("The winner of game {} is player 1!", cur_game);
            }
            return 0;
        }
        seen.push(players.clone());

        let p1 = players[0].pop_front().unwrap();
        let p2 = players[1].pop_front().unwrap();
        if cfg!(debug_assertions) {
            println!("Player 1 plays: {}", p1);
            println!("Player 2 plays: {}", p2);
        }

        let winner = if p1 <= players[0].len() && p2 <= players[1].len() {
            if cfg!(debug_assertions) {
                println!("Playing a sub-game to determine the winner...\n");
            }
            let mut player1 = players[0].clone();
            let mut player2 = players[1].clone();
            player1.truncate(p1);
            player2.truncate(p2);
            let mut sub_players = vec![player1, player2];

            let winner = play_game(&mut sub_players, max_game);

            if cfg!(debug_assertions) {
                println!("\n...anyway, back to game {}.", cur_game);
            }
            winner
        } else {
            if p1 > p2 {
                0
            } else {
                1
            }
        };

        if winner == 0 {
            if cfg!(debug_assertions) {
                println!("Player 1 wins the round!");
            }
            players[0].push_back(p1);
            players[0].push_back(p2);
        } else {
            if cfg!(debug_assertions) {
                println!("Player 2 wins the round!");
            }
            players[1].push_back(p2);
            players[1].push_back(p1);
        }
        if cfg!(debug_assertions) {
            println!();
        }

        if players[0].len() == 0 || players[1].len() == 0 {
            break;
        }
    }

    let winner = if players[0].len() == 0 { 1 } else { 0 };
    if cfg!(debug_assertions) {
        println!("The winner of game {} is player {}!", cur_game, winner + 1);
    }
    winner
}

fn main() -> Result<(), Error> {
    let mut players = load_input(INPUT_FILE)?;

    let mut max_game = 0;
    let winner = play_game(&mut players, &mut max_game);

    println!("== Post-game results ==");
    let mut answer = 0;
    for i in 0..players.len() {
        println!("Player {}'s deck: {:?}", i + 1, players[i]);
        if i == winner {
            for j in 0..players[i].len() {
                answer += players[i][j] * (players[i].len() - j);
            }
        }
    }
    println!("Answer: {}", answer);

    Ok(())
}
