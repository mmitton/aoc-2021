mod error;
mod output;
mod parser;
mod run_output;
mod year_2022;
mod year_2023;

pub use error::Error;
pub use output::Output;
pub use parser::{Lines, LinesOpt};
pub use run_output::RunOutput;

use clap::{arg, Command};
use std::{cmp::Ordering, collections::BTreeMap};

pub trait Runner {
    fn parse(&mut self, path: &str) -> Result<(), Error>;
    fn part1(&mut self) -> Result<RunOutput, Error>;
    fn part2(&mut self) -> Result<RunOutput, Error>;
}

pub type NewRunner = fn() -> Box<dyn Runner>;

#[macro_export]
macro_rules! print {
    ($($args:tt)*) => {
        Output::print(format_args!($($args)*));
    };
}

#[macro_export]
macro_rules! println {
    ($($args:tt)*) => {
        Output::println(format_args!($($args)*));
    };
}

fn run(new_runner: &NewRunner, year: usize, day: usize, part: usize) -> Result<(), Error> {
    for (path, expect_path) in Lines::find_day_part_files(year, day, part)? {
        Output::start_test(year, day, part);
        let mut runner = new_runner();
        let run = |runner: &mut Box<dyn Runner>| {
            println!("Using {path} as input");
            runner.parse(&path)?;
            let output = match part {
                1 => runner.part1()?,
                2 => runner.part2()?,
                _ => unreachable!(),
            };

            let output = output.to_string();
            if !output.contains('\n') {
                println!("Answer: {output}");
            } else {
                println!("Answer: ** Multiline **");
                println!("{output}");
            }
            if let Some(expect_path) = expect_path {
                let expect = std::fs::read_to_string(expect_path)?;
                let expect = expect.trim_end_matches('\n');
                if expect != output {
                    println!("ERROR: Output did not match expected output.");
                    if expect.contains('\n') {
                        println!("Expected: {expect}");
                    } else {
                        println!("Expected: ** Multiline **");
                        println!("{expect}");
                    }
                }
            } else {
                println!("No expected output to compare");
            }
            Ok(())
        };

        let res = run(&mut runner);
        if let Err(e) = res {
            Output::error(e);
        }
        Output::end_test();
    }
    Ok(())
}

fn get_args() -> (Option<usize>, Option<usize>) {
    let matches = Command::new("runner")
        .about("AoC Runner")
        .subcommand(
            Command::new("today").about("Run latest day available.  Will be today during AoC"),
        )
        .subcommand(Command::new("all").about("Run all days"))
        .subcommand(
            Command::new("day")
                .about("Run a given day")
                .arg_required_else_help(true)
                .arg(arg!(<YEAR> "Year").value_parser(clap::value_parser!(usize)))
                .arg(arg!(<DAY> "Day").value_parser(clap::value_parser!(usize))),
        )
        .subcommand(
            Command::new("year")
                .about("Run all days in a given year")
                .arg_required_else_help(true)
                .arg(arg!(<YEAR> "Year").value_parser(clap::value_parser!(usize))),
        )
        .get_matches();

    match matches.subcommand() {
        None | Some(("today", _)) => {
            use chrono::prelude::*;
            let today = Local::now();
            match today.month() {
                12 => match today.day() {
                    1..=25 => (Some(today.year() as usize), Some(today.day() as usize)),
                    _ => (Some(today.year() as usize), Some(25)),
                },
                _ => (Some(today.year() as usize - 1), Some(25)),
            }
        }
        Some(("all", _)) => (None, None),
        Some(("day", submatches)) => {
            let year = submatches.get_one::<usize>("YEAR").copied();
            let day = submatches.get_one::<usize>("DAY").copied();
            (year, day)
        }
        Some(("year", submatches)) => {
            let year = submatches.get_one::<usize>("YEAR").copied();
            (year, None)
        }
        subcommand => unreachable!("{subcommand:?}"),
    }
}

fn main() -> Result<(), Error> {
    let (target_year, target_day) = get_args();

    let mut runners = BTreeMap::new();
    year_2022::register(&mut runners);
    year_2023::register(&mut runners);

    use chrono::prelude::*;
    let today = Local::now();

    for ((year, day), new_runner) in runners.iter() {
        if let Some(target_year) = target_year {
            if target_year != *year {
                continue;
            }
        }
        if let Some(target_day) = target_day {
            if target_day != *day {
                continue;
            }
        }

        match (
            (today.year() as usize).cmp(year),
            (today.month() as usize).cmp(&12),
            (today.day() as usize).cmp(day),
        ) {
            (Ordering::Less, _, _) => continue,
            (Ordering::Equal, Ordering::Less, _) => continue,
            (Ordering::Equal, Ordering::Equal, Ordering::Less) => continue,
            _ => {}
        }

        run(new_runner, *year, *day, 1)?;
        run(new_runner, *year, *day, 2)?;
    }

    Ok(())
}
