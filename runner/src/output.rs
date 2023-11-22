use crate::Error;
use std::fmt::Write;
use std::sync::Mutex;
use std::time::Instant;

pub struct Output {
    year: usize,
    day: usize,
    part: usize,
    new_line: bool,
    start: Option<Instant>,
}

static OUTPUT: Mutex<Output> = Mutex::new(Output {
    year: 0,
    day: 0,
    part: 0,
    new_line: true,
    start: None,
});

impl Output {
    pub fn print(args: std::fmt::Arguments) {
        let mut output = OUTPUT.lock().expect("Could not get output lock");
        output.write_fmt(args).expect("Could not write output");
    }

    pub fn println(args: std::fmt::Arguments) {
        let mut output = OUTPUT.lock().expect("Could not get output lock");
        output.write_fmt(args).expect("Could not write output");
        output.write_char('\n').expect("Could not write output");
    }

    pub fn start_test(year: usize, day: usize, part: usize) {
        let mut output = OUTPUT.lock().expect("Could not get output lock");
        if output.start.is_some() {
            panic!("Test currently running");
        }
        output.year = year;
        output.day = day;
        output.part = part;
        output.new_line = true;
        output.start = Some(Instant::now());
    }

    pub fn end_test() {
        let mut output = OUTPUT.lock().expect("Could not get output lock");
        let elapsed = if let Some(start) = output.start {
            start.elapsed()
        } else {
            panic!("Not in test");
        };
        output.ensure_nl();
        output
            .write_fmt(format_args!("Elapsed {elapsed:?}",))
            .expect("Coult not write output");
        output.year = 0;
        output.day = 0;
        output.part = 0;
        output.new_line = true;
        output.start = None;
        println!();
        println!();
    }

    fn ensure_nl(&mut self) {
        if !self.new_line {
            self.write_char('\n').expect("Could not write output");
            self.new_line = true;
        }
    }

    pub fn error(e: Error) {
        let mut output = OUTPUT.lock().expect("Could not get output lock");
        output.ensure_nl();

        output
            .write_fmt(format_args!("Error: {e:?}"))
            .expect("Could not write output");
    }

    /*
    pub fn write_fmt_nonl(&mut self, args: std::fmt::Arguments) -> Result<(), std::fmt::Error> {
        self.output.write_fmt(args)
    }

    pub fn write_fmt(&mut self, args: std::fmt::Arguments) -> Result<(), std::fmt::Error> {
        self.output.write_fmt(args)?;
        self.output.push('\n');
        Ok(())
    }
    */
}

impl std::fmt::Write for Output {
    fn write_str(&mut self, s: &str) -> Result<(), std::fmt::Error> {
        let (s, ends_with_nl) = if s.ends_with('\n') {
            (&s[0..s.len() - 1], true)
        } else {
            (s, false)
        };
        for (idx, line) in s.split('\n').enumerate() {
            if idx != 0 {
                println!();
            }
            if self.new_line {
                print!(
                    "{year}-{day:02} Part {part}: ",
                    year = self.year,
                    day = self.day,
                    part = self.part
                );
            }
            print!("{line}");

            self.new_line = true;
        }
        if !ends_with_nl {
            self.new_line = false;
        } else {
            println!()
        }
        Ok(())
    }
}

/*
impl Drop for Output {
    fn drop(&mut self) {
        let elapsed = self.start.elapsed();
        let year = self.year;
        let day = self.day;
        let part = self.part;
        let cursor = Cursor::new(&self.output);
        for line in BufReader::new(cursor).lines() {
            print!("{year}-{day:02} Part {part}: ");
            match line {
                Ok(line) => println!("{line}"),
                Err(e) => println!("{e:?}"),
            }
        }
        println!("{year}-{day:02} Part {part}: Elapsed {elapsed:?}");
        println!();
    }
}
*/
