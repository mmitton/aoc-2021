mod algorithms;
mod error;
mod output;
mod parser;
mod run_output;

pub use algorithms::*;
pub use error::Error;
pub use output::{Output, YearDayPart, OUTPUT};
pub use parser::{find_day_part_files, search_up, Lines, LinesOpt, SearchType};
pub use run_output::RunOutput;

pub trait Runner {
    fn parse(&mut self, path: &str, part1: bool) -> Result<(), Error>;
    fn part1(&mut self) -> Result<RunOutput, Error>;
    fn part2(&mut self) -> Result<RunOutput, Error>;
}

pub type NewRunner = fn() -> Box<dyn Runner>;

pub fn output<F, R>(f: F) -> R
where
    F: Fn(&mut Output) -> R,
{
    output::OUTPUT.with(|output| f(unsafe { &mut *output.get() }))
}

#[macro_export]
macro_rules! print {
    ($($args:tt)*) => {
        $crate::output(|output| {
            use std::fmt::Write;
            let _ = write!(output.mode, $($args)*);
        });
    };
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::output(|output| {
            use std::fmt::Write;
            let _ = writeln!(output.mode);
        });
    };

    ($($args:tt)*) => {
        $crate::output(|output| {
            use std::fmt::Write;
            let _ = writeln!(output.mode, $($args)*);
        });
    };
}
