use helper::Error;

fn main() -> Result<(), Error> {
    helper::runner::main(true, |runners| {
        aoc_2015::register(runners);
        aoc_2016::register(runners);
        aoc_2017::register(runners);
        aoc_2018::register(runners);
        aoc_2019::register(runners);
        aoc_2020::register(runners);
        aoc_2021::register(runners);
        aoc_2022::register(runners);
        aoc_2023::register(runners);
    })
}
