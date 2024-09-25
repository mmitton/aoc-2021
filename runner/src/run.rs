use std::time::{Duration, Instant};

use colored::Colorize;
use helper::{find_day_part_files, Error, InputFileCache, NewRunner};

fn run_part(
    new_runner: &NewRunner,
    part: usize,
    input: impl AsRef<[u8]>,
    expect: Option<impl AsRef<[u8]>>,
) -> Result<String, Error> {
    let mut runner = new_runner();
    runner.parse(input.as_ref(), part == 1)?;
    let output = match part {
        1 => runner.part1()?,
        2 => runner.part2()?,
        _ => unreachable!(),
    };

    let output = output.to_string();
    let output = output.trim_end_matches('\n');
    if let Some(expect) = expect {
        let expect = std::str::from_utf8(expect.as_ref())?;
        let expect = expect.trim_end_matches('\n');
        if expect == output {
            Ok(output.to_string())
        } else {
            Err(Error::WrongAnswer(output.to_string(), expect.to_string()))
        }
    } else {
        Err(Error::MissingExpect(output.to_string()))
    }
}

pub fn run(
    sample_data: bool,
    new_runner: &NewRunner,
    output: bool,
    run_count: usize,
    year: usize,
    day: usize,
    part: usize,
    input_file_cache: &InputFileCache,
) -> Result<Duration, Error> {
    let ydp = helper::YearDayPart::new(year, day, part);

    const USE_FILES_CACHE: bool = true;
    let files = if USE_FILES_CACHE {
        let f = input_file_cache.files(year, day, part, sample_data)?;
        f.iter().map(|f| f.files()).collect()
    } else {
        match find_day_part_files(year, day, part, sample_data) {
            Ok(files) => files,
            Err(e) => {
                if output {
                    println!("{ydp}: Error: {}", format!("{e:?}").bright_red());
                }
                return Err(e);
            }
        }
    };

    let mut total_elapsed = Duration::new(0, 0);
    let mut total_runs = 0;
    for _ in 0..run_count {
        for (input_path, expect_path) in files.iter() {
            if output {
                println!("{ydp}: Using {input_path}");
                helper::output(|output| output.start_run(ydp));
            }

            let start = Instant::now();
            let input = std::fs::read(input_path)?;
            let expect = if let Some(p) = &expect_path {
                Some(std::fs::read(p)?)
            } else {
                None
            };
            let result = run_part(new_runner, part, &input, expect.as_ref());
            let elapsed = start.elapsed();
            total_elapsed += elapsed;
            total_runs += 1;

            if output {
                helper::output(|output| output.ensure_nl());
                if result.is_err() {
                    if let Some(capture) = helper::output(|output| output.get_capture()) {
                        print!("{capture}");
                    }
                }
                match result {
                    Ok(output) => {
                        if !output.contains('\n') {
                            println!("{ydp}:   Answer: {output}", output = output.bright_green());
                        } else {
                            for line in output.split('\n') {
                                println!("{ydp}:   Answer: {output}", output = line.bright_green());
                            }
                        }
                    }
                    Err(Error::WrongAnswer(output, expect)) => {
                        if !output.contains('\n') {
                            println!("{ydp}:   Answer: {output}", output = output.bright_red());
                        } else {
                            for line in output.split('\n') {
                                println!("{ydp}:   Answer: {output}", output = line.bright_red());
                            }
                        }
                        println!("{ydp}: ERROR: Output did not match expected output.");
                        if !expect.contains('\n') {
                            println!("{ydp}: Expected: {expect}", expect = expect.bright_yellow());
                        } else {
                            for line in expect.split('\n') {
                                println!(
                                    "{ydp}: Expected: {output}",
                                    output = line.bright_yellow()
                                );
                            }
                        }
                    }
                    Err(Error::MissingExpect(output)) => {
                        if !output.contains('\n') {
                            println!("{ydp}:   Answer: {output}", output = output.bright_yellow());
                        } else {
                            for line in output.split('\n') {
                                println!(
                                    "{ydp}:   Answer: {output}",
                                    output = line.bright_yellow()
                                );
                            }
                        }
                        println!("{ydp}: No expected output to compare");
                    }
                    Err(Error::Skipped) => {
                        println!("{ydp}: {}", "skipped".bright_yellow());
                    }
                    Err(e) => {
                        println!("{ydp}: Error: {}", format!("{e:?}").bright_red());
                    }
                }
                println!("{ydp}: {elapsed:?}");
                println!();
            } else if let Err(e) = result {
                if !matches!(e, Error::Skipped) {
                    return Err(e);
                }
            }
        }
    }
    Ok(total_elapsed / total_runs)
}
