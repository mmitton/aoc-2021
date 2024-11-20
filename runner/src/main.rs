use helper::{Error, NewRunner};
use std::collections::BTreeMap;

const README_HEADER: &str = "[Advent of Code](https://adventofcode.com/)
Michael Conrad

[Helper library](https://github.com/mmitton/helper) which holds generic algorithms and runner
infrastructure shared between AOC and Everybody Codes";

fn register(runners: &mut BTreeMap<(usize, usize), (u8, NewRunner)>) {
    aoc_2015::register(runners);
    aoc_2016::register(runners);
    aoc_2017::register(runners);
    aoc_2018::register(runners);
    aoc_2019::register(runners);
    aoc_2020::register(runners);
    aoc_2021::register(runners);
    aoc_2022::register(runners);
    aoc_2023::register(runners);
    aoc_2024::register(runners);
}

fn today(year: usize, month: usize, day: usize) -> (usize, usize) {
    if month == 12 {
        (year, day.max(25))
    } else {
        (year - 1, 25)
    }
}

fn main() -> Result<(), Error> {
    let mut config = helper::runner::Config::new(register, today);
    config.download_input(true);
    config.readme_header(README_HEADER);
    helper::runner::main::<_, _, 2>(config)
}
