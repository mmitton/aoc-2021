use chrono::prelude::*;
use std::io::Write;
use std::path::{Path, PathBuf};

use helper::{search_up, Error, SearchType};

fn exec(cmd: &str, args: &[&str]) -> Result<(), Error> {
    use std::process::Command;

    let mut cmd = Command::new(cmd);
    cmd.args(args);
    let status = cmd.status()?;
    assert!(status.success());
    Ok(())
}

fn create_year(year: usize) -> Result<(), Error> {
    // Find runner crate
    let runner_path = search_up("runner", SearchType::Dir)?;
    std::env::set_current_dir(runner_path)?;

    // Check to see if crate exists
    let crate_path_str = format!("aoc_{year}");
    let crate_path = Path::new(crate_path_str.as_str());
    if crate_path.exists() {
        return Err(Error::YearExists(year));
    }

    // Create crate library and add it as a dependency to runner
    exec("cargo", &["new", "--lib", &crate_path_str])?;
    exec("cargo", &["add", "--path", &crate_path_str])?;

    // Change in to crate folder and build files
    std::env::set_current_dir(&crate_path_str)?;
    exec("cargo", &["add", "--path", "../helper"])?;

    let mut mod_path = PathBuf::from("src");
    mod_path.push("lib.rs");
    let mut m = std::fs::File::create(mod_path)?;
    writeln!(m, "use helper::NewRunner;")?;
    writeln!(m, "use std::collections::BTreeMap;")?;
    writeln!(m)?;
    for day in 1..=25 {
        writeln!(m, "mod day_{day:02};")?;

        let mut day_path = PathBuf::from("src");
        day_path.push(format!("day_{day:02}.rs"));
        let mut d = std::fs::File::create(day_path)?;
        writeln!(d, "#[allow(unused_imports)]")?;
        writeln!(
            d,
            "use helper::{{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner}};"
        )?;
        writeln!(d)?;
        writeln!(d, "#[derive(Default)]")?;
        writeln!(d, "pub struct Day{day:02} {{}}")?;
        writeln!(d)?;
        writeln!(d, "impl Day{day:02} {{")?;
        writeln!(d, "    pub fn new() -> Self {{")?;
        writeln!(d, "        Self::default()")?;
        writeln!(d, "    }}")?;
        writeln!(d, "}}")?;
        writeln!(d)?;
        writeln!(d, "impl Runner for Day{day:02} {{")?;
        writeln!(
            d,
            "    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {{"
        )?;
        writeln!(
            d,
            "        let _lines = Lines::from_bufread(file, LinesOpt::RAW)?;"
        )?;
        writeln!(d, "        Ok(())")?;
        writeln!(d, "    }}")?;
        writeln!(d)?;
        writeln!(d, "    fn part1(&mut self) -> Result<RunOutput, Error> {{")?;
        writeln!(d, "        Err(Error::Unsolved)")?;
        writeln!(d, "    }}")?;
        writeln!(d)?;
        writeln!(d, "    fn part2(&mut self) -> Result<RunOutput, Error> {{")?;
        if day == 25 {
            writeln!(d, "        Err(Error::Skipped)")?;
        } else {
            writeln!(d, "        Err(Error::Unsolved)")?;
        }
        writeln!(d, "    }}")?;
        writeln!(d, "}}")?;
    }

    writeln!(m)?;
    writeln!(
        m,
        "pub fn register(runners: &mut BTreeMap<(usize, usize), NewRunner>) {{"
    )?;
    for day in 1..=25 {
        writeln!(
            m,
            "    runners.insert(({year}, {day}), || Box::new(day_{day:02}::Day{day:02}::new()));"
        )?;
    }
    writeln!(m, "}}")?;
    Ok(())
}

fn main() {
    let env: Vec<String> = std::env::args().collect();
    if env.len() != 2 {
        println!("Usage: {} year", env[0]);
    }

    let now = Local::now();
    let cur_year = now.year() as usize;
    let year: usize = env[1].parse().expect("Unable to parse year");
    if year < 2015 || year > cur_year {
        panic!("Year {year} out of range.  2015..={cur_year}");
    }

    println!("Making new year for {year}");

    if let Err(e) = create_year(year) {
        panic!("{e:?}");
    }
}
