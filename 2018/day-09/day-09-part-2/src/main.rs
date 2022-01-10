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
struct Game {
    players: usize,
    last_marble_points: usize,
    expected_high_score: Option<usize>,
}

impl Game {
    fn play(&self) -> (usize, usize) {
        let mut scores = vec![0; self.players];

        struct Node {
            v: usize,
            n: usize,
            p: usize,
        }

        let mut linked_list = vec![Node { v: 0, n: 0, p: 0 }];
        let mut cur = 0;
        let mut head = 0;
        let mut player = 0;

        macro_rules! move_cur {
            ($offset:expr) => {
                if $offset < 0 {
                    for _ in $offset..0 {
                        cur = linked_list[cur].p;
                    }
                } else {
                    for _ in 0..$offset {
                        cur = linked_list[cur].n;
                    }
                }
            };
        }

        for i in 1..=self.last_marble_points * 100 {
            if i % 23 != 0 {
                move_cur!(1);
                let next = linked_list[cur].n;
                let prev = cur;

                let new_node = Node {
                    v: i,
                    n: next,
                    p: prev,
                };

                linked_list.push(new_node);
                cur = linked_list.len() - 1;
                linked_list[prev].n = cur;
                linked_list[next].p = cur;
            } else {
                move_cur!(-7);
                scores[player] += i + linked_list[cur].v;

                let next = linked_list[cur].n;
                let prev = linked_list[cur].p;
                linked_list[next].p = prev;
                linked_list[prev].n = next;
                if head == cur {
                    head = next;
                }
                cur = next;
            }

            player = (player + 1) % self.players;
        }

        let mut high_score = 0;
        let mut high_player = 0;
        for player in 0..scores.len() {
            if high_score < scores[player] {
                high_score = scores[player];
                high_player = player + 1;
            }
        }

        (high_player, high_score)
    }
}

fn load_input(filename: &str) -> Result<Vec<Game>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();
    let mut games = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim().to_string();
        if line == "" || line.starts_with("#") {
            continue;
        }

        let parts: Vec<&str> = line.split(" ").collect();
        games.push(Game {
            players: parts[0].parse()?,
            last_marble_points: parts[6].parse()?,
            expected_high_score: if parts.len() > 11 {
                Some(parts[11].parse()?)
            } else {
                None
            },
        });
    }

    Ok(games)
}

fn main() -> Result<(), Error> {
    let games = load_input(INPUT_FILE)?;

    for game in &games {
        println!("Game: {:?}", game);
        let (high_player, high_score) = game.play();
        if let Some(expected_high_score) = game.expected_high_score {
            if high_score != expected_high_score {
                println!(
                    "Err: Player {} got best score of: {}  Expected: {}",
                    high_player, high_score, expected_high_score
                );
            } else {
                println!(
                    "Ok: Player {} got best score of: {}",
                    high_player, high_score
                );
            }
        } else {
            println!("Player {} got best score of: {}", high_player, high_score);
        }

        println!();
    }

    Ok(())
}
