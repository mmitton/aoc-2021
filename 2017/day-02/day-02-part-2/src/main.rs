#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NAN(std::num::ParseIntError),
}

#[derive(Debug)]
struct Spreadsheet {
    rows: Vec<Vec<isize>>,
}

impl Spreadsheet {
    fn checksum(&self) -> isize {
        let mut sum = 0;
        'row_loop: for row in &self.rows {
            for i in 0..row.len() {
                for j in i + 1..row.len() {
                    if row[i] > row[j] && row[i] % row[j] == 0 {
                        println!("{} / {} = {}", row[i], row[j], row[i] / row[j]);
                        sum += row[i] / row[j];
                        continue 'row_loop;
                    }
                    if row[j] > row[i] && row[j] % row[i] == 0 {
                        println!("{} / {} = {}", row[j], row[i], row[j] / row[i]);
                        sum += row[j] / row[i];
                        continue 'row_loop;
                    }
                }
            }
        }

        sum
    }
}

fn load_input(filename: &str) -> Result<Spreadsheet, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();

    let mut spreadsheet = Spreadsheet { rows: Vec::new() };

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }
        if line.starts_with("#") {
            continue;
        }

        let mut values: Vec<isize> = Vec::new();
        for val in line.split("\t") {
            values.push(val.parse().map_err(|e| Error::NAN(e))?);
        }

        spreadsheet.rows.push(values);
    }

    Ok(spreadsheet)
}

fn main() -> Result<(), Error> {
    let spreadsheet = load_input(INPUT_FILE)?;

    println!("Checksum: {}", spreadsheet.checksum());

    Ok(())
}
