use clap::{arg, Arg, Command};
use std::{cmp::Ordering, collections::BTreeMap};

use helper::{find_day_part_files, println, Error, NewRunner, Output, Runner};

fn run(
    capture: bool,
    sample_data: bool,
    new_runner: &NewRunner,
    year: usize,
    day: usize,
    part: usize,
) -> Result<(), Error> {
    for (path, expect_path) in find_day_part_files(year, day, part, sample_data)? {
        Output::start_test(year, day, part);
        let mut runner = new_runner();
        let run = |runner: &mut Box<dyn Runner>| {
            println!("Using {path} as input");
            if capture {
                Output::start_capture();
            }
            runner.parse(&path, part == 1)?;
            let output = match part {
                1 => runner.part1()?,
                2 => runner.part2()?,
                _ => unreachable!(),
            };

            if capture {
                let _ = Output::end_capture();
            }
            let output = output.to_string();
            if let Some(expect_path) = expect_path {
                let expect = std::fs::read_to_string(expect_path)?;
                let expect = expect.trim_end_matches('\n');
                if expect == output {
                    let prev_color = Output::green();
                    if !output.contains('\n') {
                        println!("Answer: {output}");
                    } else {
                        println!("Answer: ** Multiline **");
                        println!("{output}");
                    }
                    Output::color(prev_color);
                } else {
                    let prev_color = Output::red();
                    if !output.contains('\n') {
                        println!("Answer: {output}");
                    } else {
                        println!("Answer: ** Multiline **");
                        println!("{output}");
                    }
                    println!("ERROR: Output did not match expected output.");
                    if !expect.contains('\n') {
                        println!("Expected: {expect}");
                    } else {
                        println!("Expected: ** Multiline **");
                        println!("{expect}");
                    }
                    Output::color(prev_color);
                }
            } else {
                let prev_color = Output::yellow();
                if !output.contains('\n') {
                    println!("Answer: {output}");
                } else {
                    println!("Answer: ** Multiline **");
                    println!("{output}");
                }
                println!("No expected output to compare");
                Output::color(prev_color);
            }
            Ok(())
        };

        let res = run(&mut runner);
        match res {
            Err(Error::Skipped) => {
                if capture {
                    let _ = Output::end_capture();
                }
                let prev_color = Output::yellow();
                println!("Skipped");
                Output::color(prev_color);
            }
            Err(e) => Output::error(e),
            _ => {}
        }
        Output::end_test();
    }
    Ok(())
}

fn get_args() -> (bool, bool, Option<usize>, Option<usize>) {
    let matches = Command::new("runner")
        .about("AoC Runner")
        .arg(
            Arg::new("sample-data")
                .long("sample-data")
                .visible_alias("sample")
                .num_args(0)
                .required(false)
                .help("Run Sample Data"),
        )
        .arg(
            Arg::new("real-data")
                .long("real-data")
                .visible_alias("real")
                .num_args(0)
                .required(false)
                .help("Run Real Data"),
        )
        .arg(
            Arg::new("capture")
                .long("capture")
                .num_args(0)
                .required(false)
                .help("Capture output"),
        )
        .arg(
            Arg::new("no-capture")
                .long("no-capture")
                .num_args(0)
                .required(false)
                .help("Do not capture output"),
        )
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

    let sample_data = matches
        .get_one::<bool>("sample-data")
        .copied()
        .unwrap_or_default();
    let real_data = matches
        .get_one::<bool>("real-data")
        .copied()
        .unwrap_or_default();

    let sample_data = match (sample_data, real_data) {
        (true, true) => panic!("Cannot use both sample-data and real-data"),
        (true, false) => true,
        (false, true) => false,
        (false, false) => cfg!(debug_assertions),
    };

    let capture = matches
        .get_one::<bool>("capture")
        .copied()
        .unwrap_or_default();
    let no_capture = matches
        .get_one::<bool>("no-capture")
        .copied()
        .unwrap_or_default();

    let capture = match (capture, no_capture) {
        (true, true) => panic!("Cannot use both capture and no-capture"),
        (true, false) => true,
        (false, true) => false,
        (false, false) => !sample_data,
    };

    let (year, day) = match matches.subcommand() {
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
    };

    (capture, sample_data, year, day)
}

fn main() -> Result<(), Error> {
    let (capture, sample_data, target_year, target_day) = get_args();

    let mut runners = BTreeMap::new();
    aoc_2022::register(&mut runners);
    aoc_2023::register(&mut runners);

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

        run(capture, sample_data, new_runner, *year, *day, 1)?;
        run(capture, sample_data, new_runner, *year, *day, 2)?;
    }

    Ok(())
}
