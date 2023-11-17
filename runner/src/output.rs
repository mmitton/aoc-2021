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

    pub fn write_fmt_nonl(&mut self, args: std::fmt::Arguments) -> Result<(), std::fmt::Error> {
        self.output.write_fmt(args)
    }

    pub fn write_fmt(&mut self, args: std::fmt::Arguments) -> Result<(), std::fmt::Error> {
        self.output.write_fmt(args)?;
        self.output.push('\n');
        Ok(())
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
