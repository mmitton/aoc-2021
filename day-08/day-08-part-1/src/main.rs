#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
}

const NUM_SEGMENTS_0: u8 = 6;
const NUM_SEGMENTS_1: u8 = 2;
const NUM_SEGMENTS_2: u8 = 5;
const NUM_SEGMENTS_3: u8 = 5;
const NUM_SEGMENTS_4: u8 = 4;
const NUM_SEGMENTS_5: u8 = 5;
const NUM_SEGMENTS_6: u8 = 6;
const NUM_SEGMENTS_7: u8 = 3;
const NUM_SEGMENTS_8: u8 = 7;
const NUM_SEGMENTS_9: u8 = 6;

#[derive(Debug)]
struct Number([bool; 7]);

impl Number {
    fn num_lit(&self) -> u8 {
        let mut num_lit = 0u8;

        for num in &self.0 {
            if *num {
                num_lit += 1
            }
        }

        num_lit
    }
}

#[derive(Debug)]
struct Display {
    numbers: Vec<Number>,
}

fn load_input(filename: &str) -> Result<Vec<Display>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();

    let mut displays = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }

        let mut display = Display {
            numbers: Vec::new(),
        };
        for digit in line.replace(" | ", " ").split(" ") {
            let mut number = Number([false; 7]);
            for i in 0..7 {
                if digit.contains(('a' as u8 + i) as char) {
                    number.0[i as usize] = true;
                }
            }
            display.numbers.push(number);
        }
        displays.push(display);
    }

    Ok(displays)
}

fn main() -> Result<(), Error> {
    let displays = load_input(INPUT_FILE)?;

    let mut answer = 0;
    for display in &displays {
        for number in &display.numbers[10..] {
            let num_lit = number.num_lit();
            match num_lit {
                NUM_SEGMENTS_1 | NUM_SEGMENTS_4 | NUM_SEGMENTS_7 | NUM_SEGMENTS_8 => {
                    answer += 1;
                }
                _ => {}
            }
        }
    }

    println!("answer: {}", answer);

    Ok(())
}
