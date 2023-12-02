mod error;
mod output;
mod parser;
mod run_output;

pub use error::Error;
pub use output::Output;
pub use parser::{find_day_part_files, search_up, Lines, LinesOpt, SearchType};
pub use run_output::RunOutput;

pub trait Runner {
    fn parse(&mut self, path: &str) -> Result<(), Error>;
    fn part1(&mut self) -> Result<RunOutput, Error>;
    fn part2(&mut self) -> Result<RunOutput, Error>;
}

pub type NewRunner = fn() -> Box<dyn Runner>;

#[macro_export]
macro_rules! print {
    () => {};

    ($($args:tt)*) => {
        Output::print(format_args!($($args)*));
    };
}

#[macro_export]
macro_rules! println {
    () => {
        Output::println(format_args!(""));
    };

    ($($args:tt)*) => {
        Output::println(format_args!($($args)*));
    };
}
