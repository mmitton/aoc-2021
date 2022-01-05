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

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::IO(e)
    }
}

fn load_input(filename: &str) -> Result<Vec<Vec<usize>>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let reader = BufReader::new(f);
    let lines = reader.lines();
    let mut games = Vec::new();

    for line in lines {
        let line = line.unwrap();
        let line = line.trim();

        if line == "" {
            continue;
        }

        let mut game: Vec<usize> = Vec::new();
        for num in line.split(",") {
            game.push(num.parse()?);
        }

        games.push(game);
    }

    Ok(games)
}

fn main() -> Result<(), Error> {
    let mut games = load_input(INPUT_FILE)?;

    for game in &mut games {
        println!("Game: {:?}", game);
        'game_loop: while game.len() != 2020 {
            let last = game[game.len() - 1];
            for j in (0..game.len() - 1).rev() {
                if game[j] == last {
                    game.push(game.len() - 1 - j);
                    continue 'game_loop;
                }
            }
            game.push(0);
        }

        println!("Answer: {}", game[game.len() - 1]);
    }

    Ok(())
}
