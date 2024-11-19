use helper::Error;

const README_HEADER: &str = "[Advent of Code](https://adventofcode.com/)
Michael Conrad

[Helper library](https://github.com/mmitton/helper) which holds generic algorithms and runner
infrastructure shared between AOC and Everybody Codes

";

fn main() -> Result<(), Error> {
    helper::runner::main::<_, 2>(true, README_HEADER, |runners| {
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
    })
}
