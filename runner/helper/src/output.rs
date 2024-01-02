use std::cell::RefCell;
use std::fmt::{Display, Write};

thread_local! {
    pub static OUTPUT: RefCell<Output> = RefCell::new(Output::default());
}

#[derive(Copy, Clone)]
pub struct YearDayPart {
    year: usize,
    day: usize,
    part: usize,
}

impl YearDayPart {
    pub fn new(year: usize, day: usize, part: usize) -> Self {
        Self { year, day, part }
    }
}

impl Display for YearDayPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{year}-{day:02} Part {part}",
            year = self.year,
            day = self.day,
            part = self.part
        )
    }
}

#[derive(Default)]
pub struct Output {
    pub mode: Mode,
}

#[derive(Default)]
pub enum Mode {
    #[default]
    NoOutput,
    Stdout {
        ydp: YearDayPart,
        new_line: bool,
    },
    Capture {
        ydp: YearDayPart,
        new_line: bool,
        capture: String,
    },
}

impl Output {
    pub fn start_run(&mut self, ydp: YearDayPart) {
        self.mode.reset(ydp);
    }
}

impl Mode {
    pub fn get_capture(&self) -> &str {
        match self {
            Self::NoOutput | Self::Stdout { .. } => "",
            Self::Capture { capture, .. } => capture,
        }
    }

    pub fn reset(&mut self, ydp: YearDayPart) {
        match self {
            Self::NoOutput => {}
            Self::Stdout {
                ydp: _ydp,
                new_line,
            } => {
                *_ydp = ydp;
                *new_line = true;
            }
            Self::Capture {
                ydp: _ydp,
                new_line,
                capture,
            } => {
                *_ydp = ydp;
                *new_line = true;
                capture.clear();
            }
        }
    }
}

impl Write for Mode {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        use std::io::Write;
        macro_rules! output {
            ($ydp:ident, $new_line:ident, $w:expr) => {{
                #[allow(unused_mut)]
                let mut w = $w;
                let _ = write!(w, "{s}");
                *$new_line = s.ends_with('\n');
            }};
        }

        match self {
            Self::NoOutput => {}
            Self::Stdout { ydp, new_line } => {
                output!(ydp, new_line, std::io::stdout().lock())
            }
            Self::Capture {
                ydp,
                new_line,
                capture,
            } => output!(ydp, new_line, capture),
        }

        Ok(())
    }

    fn write_fmt(&mut self, args: std::fmt::Arguments<'_>) -> std::fmt::Result {
        match self {
            Self::NoOutput => {}
            Self::Stdout { .. } | Self::Capture { .. } => self.write_str(&args.to_string())?,
        }

        Ok(())
    }

    fn write_char(&mut self, c: char) -> std::fmt::Result {
        match self {
            Self::NoOutput => {}
            Self::Stdout { .. } | Self::Capture { .. } => write!(self, "{c}")?,
        }

        Ok(())
    }
}

/*

static OUTPUT: Mutex<Output> = Mutex::new(Output {
    year: 0,
    day: 0,
    part: 0,
    new_line: true,
    start: None,
    capture: None,
    color: None,
    times_mode: false,
    times: BTreeMap::new(),
});

impl Output {
    pub fn green() -> Option<colored::Color> {
        let mut output = OUTPUT.lock().expect("Could not get output lock");
        output.color.replace(colored::Color::BrightGreen)
    }

    pub fn red() -> Option<colored::Color> {
        let mut output = OUTPUT.lock().expect("Could not get output lock");
        output.color.replace(colored::Color::BrightRed)
    }

    pub fn yellow() -> Option<colored::Color> {
        let mut output = OUTPUT.lock().expect("Could not get output lock");
        output.color.replace(colored::Color::BrightYellow)
    }

    pub fn color(c: Option<colored::Color>) -> Option<colored::Color> {
        let mut output = OUTPUT.lock().expect("Could not get output lock");
        if let Some(c) = c {
            output.color.replace(c)
        } else {
            output.color.take()
        }
    }

    pub fn print(args: std::fmt::Arguments) {
        let mut output = OUTPUT.lock().expect("Could not get output lock");
        if output.times_mode {
            return;
        }
        output.write_fmt(args).expect("Could not write output");
    }

    pub fn println(args: std::fmt::Arguments) {
        let mut output = OUTPUT.lock().expect("Could not get output lock");
        if output.times_mode {
            return;
        }
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

    pub fn start_times_mode() {
        let mut output = OUTPUT.lock().expect("Could not get output lock");
        output.times_mode = true;
    }

    pub fn end_times_mode() {
        let mut output = OUTPUT.lock().expect("Could not get output lock");
        output.times_mode = false;
        let mut prt1 = Duration::new(0, 0);
        let mut prt2 = Duration::new(0, 0);
        let mut total = Duration::new(0, 0);
        println!("+-------+------------+------------+--------+--------+");
        println!("| Day   | Part 1     | Part 2     | P1 %   | P2 %   |");
        println!("+-------+------------+------------+--------+--------+");
        for parts in output.times.values() {
            if let Some(part1) = parts[0] {
                prt1 += part1;
                total += part1;
            }
            if let Some(part2) = parts[1] {
                prt2 += part2;
                total += part2;
            }
        }
        for (day, parts) in output.times.iter() {
            let (prt1, per1) = if let Some(prt1) = parts[0] {
                (
                    format!("{:0.5} s", prt1.as_secs_f64()),
                    format!("{:0.2}%", prt1.as_secs_f64() / total.as_secs_f64() * 100.),
                )
            } else {
                ("".to_string(), "".to_string())
            };
            let (prt2, per2) = if let Some(prt2) = parts[1] {
                (
                    format!("{:0.5} s", prt2.as_secs_f64()),
                    format!("{:0.2}%", prt2.as_secs_f64() / total.as_secs_f64() * 100.),
                )
            } else {
                ("".to_string(), "".to_string())
            };
            println!("| {day:>5} | {prt1:>10} | {prt2:>10} | {per1:>6} | {per2:>6} |");
        }
        let prt1 = format!("{elapsed:0.5} s", elapsed = prt1.as_secs_f64());
        let prt2 = format!("{elapsed:0.5} s", elapsed = prt2.as_secs_f64());
        let total = format!("{elapsed:0.5} s", elapsed = total.as_secs_f64());
        println!("+-------+------------+------------+-----------------+");
        println!("| Total | {prt1:>10} | {prt2:>10} | Both {total:>10} |");
        println!("+-------+------------+------------+-----------------+");
    }

    pub fn start_capture() {
        let mut output = OUTPUT.lock().expect("Could not get output lock");
        if output.start.is_none() {
            panic!("Test not running");
        }
        if output.capture.is_some() {
            panic!("Capture already in progress");
        }
        output.capture = Some(String::new());
    }

    pub fn end_capture() -> String {
        let mut output = OUTPUT.lock().expect("Could not get output lock");
        if let Some(mut capture) = output.capture.take() {
            if !capture.ends_with('\n') {
                capture.push('\n');
            }
            output.new_line = true;
            capture
        } else {
            panic!("Capture not in progress");
        }
    }

    pub fn end_test() {
        let mut output = OUTPUT.lock().expect("Could not get output lock");
        if output.times_mode {
            let elapsed = if let Some(parsed) = output.start.take() {
                parsed.elapsed()
            } else {
                panic!("Not in test");
            };
            let day = output.day;
            let part = output.part - 1;
            let day = output.times.entry(day).or_default();
            day[part] = Some(elapsed);
        } else {
            let start = if let Some(start) = output.start.take() {
                start
            } else {
                panic!("Not in test");
            };
            let run = start.elapsed();
            output.ensure_nl();
            output
                .write_fmt(format_args!("Run {run:?}",))
                .expect("Coult not write output");
            println!();
            println!();
        }
        output.year = 0;
        output.day = 0;
        output.part = 0;
        output.new_line = true;
        output.start = None;
    }

    fn ensure_nl(&mut self) {
        if !self.new_line {
            self.write_char('\n').expect("Could not write output");
            self.new_line = true;
        }
    }

    pub fn error(e: Error) {
        let prev_color = Self::red();
        let mut output = OUTPUT.lock().expect("Could not get output lock");
        output.ensure_nl();

        if let Some(capture) = output.capture.take() {
            if !capture.is_empty() {
                println!("{capture}");
            }
        }

        output
            .write_fmt(format_args!("Error: {e:?}"))
            .expect("Could not write output");
        drop(output);
        Self::color(prev_color);
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
                if let Some(capture) = &mut self.capture {
                    capture.push('\n');
                } else {
                    println!();
                }
            }
            if self.new_line {
                if let Some(capture) = &mut self.capture {
                    write!(
                        capture,
                        "{year}-{day:02} Part {part}: ",
                        year = self.year,
                        day = self.day,
                        part = self.part
                    )
                    .expect("Could not capture output");
                } else {
                    print!(
                        "{year}-{day:02} Part {part}: ",
                        year = self.year,
                        day = self.day,
                        part = self.part
                    );
                }
            }
            if let Some(capture) = &mut self.capture {
                capture.push_str(line);
            } else if let Some(color) = &self.color {
                use colored::Colorize;
                print!("{}", Colorize::color(line, *color));
            } else {
                print!("{line}");
            }

            self.new_line = true;
        }
        if !ends_with_nl {
            self.new_line = false;
        } else if let Some(capture) = &mut self.capture {
            capture.push('\n');
        } else {
            println!();
        }
        Ok(())
    }
}
*/
