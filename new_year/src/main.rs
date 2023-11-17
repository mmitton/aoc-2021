use chrono::prelude::*;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    YearExists,
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::IO(e)
    }
}

fn create_year(path: impl AsRef<Path>, year: usize) -> Result<(), Error> {
    let path = path.as_ref();
    if path.exists() {
        return Err(Error::YearExists);
    }

    std::fs::create_dir(path)?;

    let mut mod_path = PathBuf::from(path);
    mod_path.push("mod.rs");
    let mut m = std::fs::File::create(mod_path)?;
    writeln!(m, "use crate::NewRunner;")?;
    writeln!(m, "use std::collections::BTreeMap;")?;
    writeln!(m)?;
    for day in 1..=25 {
        writeln!(m, "mod day_{day:02};")?;

        let mut day_path = PathBuf::from(path);
        day_path.push(format!("day_{day:02}.rs"));
        let mut d = std::fs::File::create(day_path)?;
        writeln!(d, "#[allow(unused_imports)]")?;
        writeln!(
            d,
            "use crate::{{output, output_noln, Error, Lines, LinesOpt, Output, Runner}};"
        )?;
        writeln!(d)?;
        writeln!(d, "#[derive(Debug)]")?;
        writeln!(d, "pub enum RunnerError {{}}")?;
        writeln!(d)?;
        writeln!(d, "impl From<RunnerError> for Error {{")?;
        writeln!(d, "    fn from(e: RunnerError) -> Self {{")?;
        writeln!(d, "        Self::Runner(format!(\"{{e:?}}\"))")?;
        writeln!(d, "    }}")?;
        writeln!(d, "}}")?;
        writeln!(d)?;
        writeln!(d, "pub struct Day{day:02} {{")?;
        writeln!(d, "    output: Output,")?;
        writeln!(d, "}}")?;
        writeln!(d)?;
        writeln!(d, "impl Day{day:02} {{")?;
        writeln!(d, "    pub fn new(part: usize) -> Self {{")?;
        writeln!(d, "        Self {{ ")?;
        writeln!(d, "            output: Output::new({year}, {day}, part),")?;
        writeln!(d, "        }}")?;
        writeln!(d, "    }}")?;
        writeln!(d, "}}")?;
        writeln!(d)?;
        writeln!(d, "impl Runner for Day{day:02} {{")?;
        writeln!(
            d,
            "    fn parse(&mut self, part: usize) -> Result<(), Error> {{"
        )?;
        writeln!(
            d,
            "        let _lines = Lines::find_day_part(&mut self.output, {year}, {day}, part, LinesOpt::RAW)?;"
        )?;
        writeln!(d, "        Ok(())")?;
        writeln!(d, "    }}")?;
        writeln!(d)?;
        writeln!(d, "    fn part1(&mut self) -> Result<(), Error> {{")?;
        writeln!(d, "        Err(Error::Unsolved)")?;
        writeln!(d, "    }}")?;
        writeln!(d)?;
        writeln!(d, "    fn part2(&mut self) -> Result<(), Error> {{")?;
        writeln!(d, "        Err(Error::Unsolved)")?;
        writeln!(d, "    }}")?;
        writeln!(d)?;
        writeln!(d, "    fn output(&mut self) -> &mut Output {{")?;
        writeln!(d, "        &mut self.output")?;
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
            "    runners.insert(({year}, {day}), |part| Box::new(day_{day:02}::Day{day:02}::new(part)));"
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

    if let Err(e) = create_year(format!("runner/src/year_{year}"), year) {
        panic!("{e:?}");
    }
}
