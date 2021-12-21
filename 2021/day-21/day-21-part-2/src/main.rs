#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

use std::collections::VecDeque;

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NAN(std::num::ParseIntError),
    InvalidInput(String),
}

fn load_input(filename: &str) -> Result<(usize, usize), Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();

    let mut p1 = 0usize;
    let mut p2 = 0usize;

    for line in lines {
        let line = line.unwrap();
        let line = line.trim();
        match &line[7..8] {
            "1" => p1 = line[28..].parse().map_err(|e| Error::NAN(e))?,
            "2" => p2 = line[28..].parse().map_err(|e| Error::NAN(e))?,
            _ => return Err(Error::InvalidInput(line.to_string())),
        }
    }

    Ok((p1, p2))
}

struct Dice {
    totals: Vec<usize>,
    universes: Vec<usize>,
}

impl Dice {
    fn new() -> Self {
        let mut totals = Vec::new();
        let mut universes = Vec::new();
        for a in 1..=3 {
            for b in 1..=3 {
                for c in 1..=3 {
                    let sum = a + b + c;

                    let mut found = false;
                    for i in 0..totals.len() {
                        if totals[i] == sum {
                            found = true;
                            universes[i] += 1;
                            break;
                        }
                    }

                    if !found {
                        totals.push(sum);
                        universes.push(1);
                    }
                }
            }
        }
        println!("   totals: {:?}", totals);
        println!("universes: {:?}", universes);
        Self {
            totals: totals,
            universes: universes,
        }
    }
}

#[derive(Clone, Debug)]
struct Player {
    pos: usize,
    score: usize,
}

struct Game {
    players: [Player; 2],
}

impl Game {
    fn play(&mut self, dice: Dice) -> [usize; 2] {
        #[derive(Clone, Debug)]
        struct Turn {
            players: [Player; 2],
            universes: usize,
            player: usize,
            turns: usize,
        }

        let mut turns: VecDeque<Turn> = VecDeque::new();
        turns.push_back(Turn {
            players: self.players.clone(),
            universes: 1,
            player: 1,
            turns: 0,
        });

        let mut wins = [0, 0];

        loop {
            if let Some(turn) = turns.pop_front() {
                for i in 0..dice.totals.len() {
                    let mut turn = turn.clone();
                    turn.turns += 1;
                    turn.player = 1 - turn.player;
                    let mut pos = turn.players[turn.player].pos + dice.totals[i];
                    pos = ((pos - 1) % 10) + 1;
                    turn.universes *= dice.universes[i];
                    turn.players[turn.player].pos = pos;
                    turn.players[turn.player].score += pos;

                    if turn.players[turn.player].score >= 21 {
                        wins[turn.player] += turn.universes;
                    } else {
                        turns.push_back(turn);
                    }
                }
            } else {
                break;
            }
        }

        wins
    }
}

fn main() -> Result<(), Error> {
    let (p1, p2) = load_input(INPUT_FILE)?;

    let dice = Dice::new();
    let mut game = Game {
        players: [Player { score: 0, pos: p1 }, Player { score: 0, pos: p2 }],
    };
    let wins = game.play(dice);

    println!("Wins: {:?}", wins,);
    if wins[0] > wins[1] {
        println!("Answer: {}", wins[0]);
    } else {
        println!("Answer: {}", wins[1]);
    }

    Ok(())
}
