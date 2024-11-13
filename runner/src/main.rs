use helper::Error;
use std::{cmp::Ordering, collections::BTreeMap, time::Duration};

mod args;
mod run;

struct TimesCacheEntry {
    day: usize,
    part1: Result<Duration, Error>,
    part2: Result<Duration, Error>,
}

fn print_times(md: bool, run_count: usize, times_cache: &BTreeMap<usize, Vec<TimesCacheEntry>>) {
    if times_cache.len() > 1 {
        if md {
            println!("| Year Totals | Part 1 | Part 2 | Both |");
            println!("| ---: | ---: | ---: | ---: |");
        } else {
            println!("+------+------------+------------+-------------+");
            println!("| Year |     Part 1 |     Part 2 |      Both % |");
            println!("+------+------------+------------+-------------+");
        }
        for (year, times_cache) in times_cache.iter().rev() {
            let mut total = Duration::new(0, 0);
            let mut part1_total = Duration::new(0, 0);
            let mut part2_total = Duration::new(0, 0);
            for entry in times_cache.iter() {
                if let Ok(dur) = entry.part1 {
                    part1_total += dur;
                    total += dur;
                }
                if let Ok(dur) = entry.part2 {
                    part2_total += dur;
                    total += dur;
                }
            }
            let prt1 = format!("{elapsed:0.5} s", elapsed = part1_total.as_secs_f64());
            let prt2 = format!("{elapsed:0.5} s", elapsed = part2_total.as_secs_f64());
            let total = format!("{elapsed:0.5} s", elapsed = total.as_secs_f64());
            if md {
                println!("| {year} | {prt1} | {prt2} | {total} |");
            } else {
                println!("| {year} | {prt1:>10} | {prt2:>10} | {total:>11} |");
            }
        }
        if !md {
            println!("+------+------------+------------+-------------+");
        }
        println!();
    }

    for (year, times_cache) in times_cache.iter().rev() {
        let mut total = Duration::new(0, 0);
        let mut part1_total = Duration::new(0, 0);
        let mut part2_total = Duration::new(0, 0);
        for entry in times_cache.iter() {
            if let Ok(dur) = entry.part1 {
                part1_total += dur;
                total += dur;
            }
            if let Ok(dur) = entry.part2 {
                part2_total += dur;
                total += dur;
            }
        }
        if run_count > 1 {
            println!("Year: {year}  Averaged over {run_count} runs.");
        } else {
            println!("Year: {year}");
        }
        if md {
            println!("| Day | Part 1 | Part 2 | P1 % | P2 % |");
            println!("| ---: | ---: | ---: | ---: | ---: |");
        } else {
            println!("+-------+------------+------------+---------+---------+");
            println!("|   Day |     Part 1 |     Part 2 |    P1 % |    P2 % |");
            println!("+-------+------------+------------+---------+---------+");
        }
        for TimesCacheEntry { day, part1, part2 } in times_cache.iter() {
            if part1.is_err() && part2.is_err() {
                continue;
            }
            let (prt1, per1) = if let Ok(prt1) = part1 {
                (
                    format!("{:0.5} s", prt1.as_secs_f64()),
                    format!("{:0.2}%", prt1.as_secs_f64() / total.as_secs_f64() * 100.),
                )
            } else {
                (String::new(), String::new())
            };
            let (prt2, per2) = if let Ok(prt2) = part2 {
                (
                    format!("{:0.5} s", prt2.as_secs_f64()),
                    format!("{:0.2}%", prt2.as_secs_f64() / total.as_secs_f64() * 100.),
                )
            } else {
                (String::new(), String::new())
            };
            if md {
                println!("| {day} | {prt1} | {prt2} | {per1} | {per2} |");
            } else {
                println!("| {day:>5} | {prt1:>10} | {prt2:>10} | {per1:>7} | {per2:>7} |");
            }
        }
        let prt1 = format!("{elapsed:0.5} s", elapsed = part1_total.as_secs_f64());
        let prt2 = format!("{elapsed:0.5} s", elapsed = part2_total.as_secs_f64());
        let total = format!("{elapsed:0.5} s", elapsed = total.as_secs_f64());
        if md {
            println!("| Total | {prt1} | {prt2} | Both | {total} |");
        } else {
            println!("+-------+------------+------------+-------------------+");
            println!("| Total | {prt1:>10} | {prt2:>10} | Both  {total:>11} |");
            println!("+-------+------------+------------+-------------------+");
        }
        println!();
    }
}

fn main() -> Result<(), Error> {
    let (sample_data, no_capture, times, md, target_year, target_day) = args::get();

    let mut runners = BTreeMap::new();
    aoc_2015::register(&mut runners);
    aoc_2016::register(&mut runners);
    aoc_2017::register(&mut runners);
    aoc_2018::register(&mut runners);
    aoc_2019::register(&mut runners);
    aoc_2020::register(&mut runners);
    aoc_2021::register(&mut runners);
    aoc_2022::register(&mut runners);
    aoc_2023::register(&mut runners);

    if times.is_some() {
        helper::output(|output| output.no_output());
    } else if cfg!(debug_assertions) || no_capture {
        helper::output(|output| output.stdout());
    } else {
        helper::output(|output| output.capture());
    }

    use chrono::prelude::*;
    let today = Local::now();

    let mut times_cache: BTreeMap<usize, Vec<TimesCacheEntry>> = BTreeMap::new();
    let run_count = times.unwrap_or(1);

    if let (Some(year), Some(day)) = (target_year, target_day) {
        helper::download_input(year, day)?;
    }

    let input_file_cache = helper::InputFileCache::new()?;
    for ((year, day), new_runner) in runners.iter().rev() {
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

        let part1 = run::run(
            sample_data,
            new_runner,
            times.is_none(),
            run_count,
            *year,
            *day,
            1,
            &input_file_cache,
        );
        let part2 = if *day != 25 {
            run::run(
                sample_data,
                new_runner,
                times.is_none(),
                run_count,
                *year,
                *day,
                2,
                &input_file_cache,
            )
        } else {
            Err(Error::Skipped)
        };

        if times.is_some() {
            times_cache.entry(*year).or_default().push(TimesCacheEntry {
                day: *day,
                part1,
                part2,
            });
        }
    }

    if times.is_some() && !times_cache.is_empty() {
        print_times(md, run_count, &times_cache);
    }

    Ok(())
}
