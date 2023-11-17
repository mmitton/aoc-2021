use crate::Error;
use std::fmt::Write;
use std::io::{BufRead, BufReader, Cursor};
use std::time::Instant;

pub struct Output {
    year: usize,
    day: usize,
    part: usize,
    output: String,
    start: Instant,
}

impl Output {
    pub fn new(year: usize, day: usize, part: usize) -> Self {
        Self {
            year,
            day,
            part,
            output: String::new(),
            start: Instant::now(),
        }
    }

    pub fn error(&mut self, e: Error) {
        if !self.output.is_empty() && !self.output.ends_with('\n') {
            writeln!(self.output, "\n").expect("Cannot write error to output: {e:?}");
        }
        writeln!(self.output, "Error: {e:?}").expect("Cannot write error to output: {e:?}");
    }
}

impl Write for Output {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.output.write_str(s)
    }
}

impl Drop for Output {
    fn drop(&mut self) {
        let elapsed = self.start.elapsed();
        let year = self.year;
        let day = self.day;
        let part = self.part;
        let cursor = Cursor::new(&self.output);
        println!("{year}-{day:02} Part {part}: Elapsed {elapsed:?}");
        for line in BufReader::new(cursor).lines() {
            print!("{year}-{day:02} Part {part}: ");
            match line {
                Ok(line) => println!("{line}"),
                Err(e) => println!("{e:?}"),
            }
        }
        println!();
    }
}
