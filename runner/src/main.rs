mod error;
mod output;
mod parser;
// mod year_2015;
// mod year_2016;
// mod year_2017;
// mod year_2018;
// mod year_2019;
// mod year_2020;
mod year_2021;
mod year_2022;
mod year_2023;

pub use error::Error;
pub use output::Output;
pub use parser::{Lines, LinesOpt};

use clap::{arg, Command};
use std::{cmp::Ordering, collections::BTreeMap};

pub trait Runner {
    fn parse(&mut self, part: usize) -> Result<(), Error>;
    fn part1(&mut self) -> Result<(), Error>;
    fn part2(&mut self) -> Result<(), Error>;
    fn output(&mut self) -> &mut Output;
}

pub type NewRunner = fn(part: usize) -> Box<dyn Runner>;

#[macro_export]
macro_rules! output_noln {
    ($output:expr, $($args:tt)*) => {
        $output.write_fmt_noln(format_args!($($args)*))
    };

    ($output:expr) => {
        $output.write_fmt_noln(format_args!(""))
    };
}

#[macro_export]
macro_rules! output {
    ($output:expr, $($args:tt)*) => {
        $output.write_fmt(format_args!($($args)*))
    };

    ($output:expr) => {
        $output.write_fmt(format_args!(""))
    };
}

macro_rules! run {
    ($runner:expr) => {{
        run!($runner, 1, part1);
        run!($runner, 2, part2);
    }};

    ($runner:expr, $part_num:expr, $part_fn:ident) => {{
        let mut runner = $runner($part_num);
        let run = |runner: &mut Box<dyn Runner>| {
            runner.parse($part_num)?;
            runner.$part_fn()?;
            Ok(())
        };

        let res = run(&mut runner);
        let output = runner.output();
        if let Err(e) = res {
            output.error(e);
        }
        drop(runner);
    }};
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
        Some(("day", submatches)) | Some(("year", submatches)) => {
            let year = submatches.get_one::<usize>("YEAR").copied();
            let day = submatches.get_one::<usize>("DAY").copied();
            (year, day)
        }
        subcommand => unreachable!("{subcommand:?}"),
    }
}

fn main() {
    let (target_year, target_day) = get_args();

    let mut runners = BTreeMap::new();
    // year_2015::register(&mut runners);
    // year_2016::register(&mut runners);
    // year_2017::register(&mut runners);
    // year_2018::register(&mut runners);
    // year_2019::register(&mut runners);
    // year_2020::register(&mut runners);
    year_2021::register(&mut runners);
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

        run!(new_runner);
    }
}
