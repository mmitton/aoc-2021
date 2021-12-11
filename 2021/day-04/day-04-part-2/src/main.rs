#![feature(drain_filter)]

#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    InvalidInput(String),
    NAN(std::num::ParseIntError),
    NoSolution,
}

#[derive(Debug)]
struct Cell {
    number: usize,
    marked: bool,
}

#[derive(Debug)]
struct Row {
    cols: Vec<Cell>,
}

#[derive(Debug)]
struct Board {
    win: bool,
    rows: Vec<Row>,
}

impl Board {
    fn mark(&mut self, num: usize) -> bool {
        for row in &mut self.rows {
            for col in &mut row.cols {
                if col.number == num {
                    col.marked = true;
                }
            }
        }

        'row_loop: for y in 0..self.rows.len() {
            for x in 0..self.rows[0].cols.len() {
                if !self.rows[y].cols[x].marked {
                    continue 'row_loop;
                }
            }

            self.win = true;
            return true;
        }

        'col_loop: for x in 0..self.rows[0].cols.len() {
            for y in 0..self.rows.len() {
                if !self.rows[y].cols[x].marked {
                    continue 'col_loop;
                }
            }

            self.win = true;
            return true;
        }
        return false;
    }

    fn unmarked(&self) -> Vec<usize> {
        let mut unmarked = Vec::new();

        for row in &self.rows {
            for col in &row.cols {
                if !col.marked {
                    unmarked.push(col.number);
                }
            }
        }

        unmarked
    }
}

#[derive(Debug)]
struct Game {
    numbers: Vec<usize>,
    boards: Vec<Board>,
}

fn load_game(filename: &str) -> Result<Game, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let mut game = Game {
        numbers: Vec::new(),
        boards: Vec::new(),
    };

    let mut lines = BufReader::new(f).lines();

    for number in lines
        .next()
        .unwrap()
        .map_err(|_| Error::InvalidInput("Cannot get numbers line".to_string()))?
        .split(",")
    {
        let number = number.parse::<usize>().map_err(|e| Error::NAN(e))?;
        game.numbers.push(number);
    }

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;

        if line == "" {
            // make new board
            game.boards.push(Board {
                rows: Vec::new(),
                win: false,
            })
        } else {
            let mut row = Row { cols: Vec::new() };
            for number in line.split(" ") {
                if number == "" {
                    continue;
                }
                let number = number.parse::<usize>().map_err(|e| Error::NAN(e))?;
                row.cols.push(Cell {
                    number: number,
                    marked: false,
                });
            }
            let idx = game.boards.len() - 1;
            game.boards[idx].rows.push(row);
        }
    }

    Ok(game)
}

fn main() -> Result<(), Error> {
    let mut game = load_game(INPUT_FILE)?;

    let mut last_winning_board = usize::MAX;
    for i in 0..game.numbers.len() {
        let num_called = game.numbers[i];
        for (board_idx, board) in game.boards.iter_mut().enumerate() {
            if board.mark(num_called) {
                let numbers = board.unmarked();
                let mut ans = 0usize;
                for number in numbers {
                    ans += number;
                }
                ans *= num_called;
                last_winning_board = ans;
                println!("Winning board: {}  Ans: {}", board_idx, ans);
            }
        }

        game.boards.drain_filter(|b| b.win);
    }

    return Ok(());
}
