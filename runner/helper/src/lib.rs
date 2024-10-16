mod algorithms;
mod bit_grid;
mod error;
mod file_scanner;
mod output;
mod parser;
mod priority_vec;
mod run_output;
mod small_vec;

pub use algorithms::*;
pub use bit_grid::*;
pub use error::Error;
pub use file_scanner::InputFileCache;
pub use output::{Output, YearDayPart, OUTPUT};
pub use parser::{find_day_part_files, Lines, LinesIter, LinesOpt};
pub use priority_vec::PriorityVec;
pub use run_output::RunOutput;
pub use small_vec::SmallVec;

pub type HashMap<K, V> = rustc_hash::FxHashMap<K, V>;
pub type HashSet<K> = rustc_hash::FxHashSet<K>;

pub trait Runner {
    fn parse(&mut self, file: &[u8], part1: bool) -> Result<(), Error>;
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
