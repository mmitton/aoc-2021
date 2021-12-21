#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

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
    pos: usize,
    rolls: usize,
}

impl Dice {
    fn roll(&mut self) -> usize {
        self.pos += 1;
        self.pos %= 100;
        self.rolls += 1;

        self.pos + 1
    }
}

fn main() -> Result<(), Error> {
    let (p1, p2) = load_input(INPUT_FILE)?;

    let mut dice = Dice { pos: 99, rolls: 0 };

    let mut players = vec![(0, p1), (0, p2)];
    let mut turn = 0;
    loop {
        for _ in 0..3 {
            players[turn].1 += dice.roll()
        }
        players[turn].1 = ((players[turn].1 - 1) % 10) + 1;
        players[turn].0 += players[turn].1;

        if players[turn].0 >= 1000 {
            println!("Player {} wins!", turn);
            println!("Dice Rolls: {}", dice.rolls);
            println!("Answer : {}", players[(turn + 1) % 2].0 * dice.rolls);
            break;
        }

        turn = (turn + 1) % 2;
    }

    println!("p1:{}  p2:{}", p1, p2);

    Ok(())
}
