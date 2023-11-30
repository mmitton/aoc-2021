use crate::Error;
use bitflags::bitflags;
use std::fs::{canonicalize, read_dir, File};
use std::io::{BufRead, BufReader, Read};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Lines(Vec<String>);

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct LinesOpt: u8 {
        const RAW = 0;
        const TRIM = 1 << 0;
        const REMOVE_COMMENTS = 1 << 1;
        const REMOVE_EMPTY = 2 << 1;
        const ALL = !0;
    }
}

#[derive(PartialEq, Eq)]
pub enum SearchType {
    File,
    Dir,
}

impl Lines {
    pub fn from_reader(r: impl Read, options: LinesOpt) -> Result<Self, Error> {
        let mut lines = Vec::new();
        for line in BufReader::new(r).lines() {
            let line = line?;
            let mut l = line.as_str();
            if options.contains(LinesOpt::TRIM) {
                l = l.trim();
            }
            if options.contains(LinesOpt::REMOVE_COMMENTS) && l.starts_with('#') {
                continue;
            }
            if options.contains(LinesOpt::REMOVE_EMPTY) && l.is_empty() {
                continue;
            }
            lines.push(String::from(l));
        }

        Ok(Self(lines))
    }

    pub fn from_path(path: impl AsRef<Path>, options: LinesOpt) -> Result<Self, Error> {
        Self::from_reader(File::open(path)?, options)
    }

    pub fn iter(&self) -> LinesIter {
        LinesIter(self.0.iter())
    }
}

pub struct LinesIter<'a>(std::slice::Iter<'a, String>);

impl<'a> Iterator for LinesIter<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        match self.0.next() {
            Some(s) => Some(s.as_str()),
            None => None,
        }
    }
}

pub fn search_up(file: &str, file_type: SearchType) -> Result<PathBuf, Error> {
    let mut root = canonicalize(".")?;
    let mut path;
    loop {
        path = root.clone();
        path.push(file);
        match file_type {
            SearchType::Dir => {
                if path.is_dir() {
                    return Ok(path);
                }
            }
            SearchType::File => {
                if path.is_file() {
                    return Ok(path);
                }
            }
        }

        root = if let Some(root) = root.parent() {
            root.into()
        } else {
            return Err(Error::SearchUpFailed(file.into()));
        };
    }
}

#[allow(clippy::type_complexity)]
fn get_files(
    year: usize,
    day: usize,
) -> Result<(Vec<String>, Vec<String>, Vec<String>, Vec<String>), Error> {
    let input_files = search_up("input_files", SearchType::Dir)?;

    let mut sample_1 = Vec::new();
    let mut sample_2 = Vec::new();
    let mut real_1 = Vec::new();
    let mut real_2 = Vec::new();
    for path in read_dir(&input_files)? {
        let path = path?;
        let file_name = path.file_name();
        let file_name: String = if let Some(file_name) = file_name.to_str() {
            file_name.into()
        } else {
            return Err(Error::InvalidInputFile(file_name));
        };
        let mut path = input_files.clone();
        path.push(&file_name);
        let path: String = if let Some(path) = path.to_str() {
            path.into()
        } else {
            return Err(Error::InvalidInputFile(path.into_os_string()));
        };

        if let Some(base) = file_name.as_str().strip_suffix(".txt") {
            let parts: Vec<&str> = base.split('-').collect();
            if parts.len() < 3 {
                continue;
            }
            if parts[0] != "input" {
                continue;
            }
            if let Ok(file_year) = parts[1].parse() {
                if year != file_year {
                    continue;
                }
            }
            if let Ok(file_day) = parts[2].parse() {
                if day != file_day {
                    continue;
                }
            }

            let mut rest = &parts[3..];
            if rest.is_empty() {
                real_1.push(path);
                continue;
            }
            let sample = if rest[0] == "sample" {
                rest = &rest[1..];
                true
            } else {
                false
            };

            let part = if rest.is_empty() {
                1
            } else if let Ok(part) = rest[0].parse() {
                part
            } else {
                1
            };

            match (sample, part) {
                (false, 1) => real_1.push(path),
                (false, 2) => real_2.push(path),
                (true, 1) => sample_1.push(path),
                (true, 2) => sample_2.push(path),
                _ => {}
            }
        }
    }
    Ok((sample_1, sample_2, real_1, real_2))
}

pub fn find_day_part_files(
    year: usize,
    day: usize,
    part: usize,
    sample_data: bool,
) -> Result<Vec<(String, Option<String>)>, Error> {
    download_input(year, day)?;

    let (sample_1, sample_2, real_1, real_2) = get_files(year, day)?;

    let (part1, mut part2) = if sample_data {
        (sample_1, sample_2)
    } else {
        (real_1, real_2)
    };

    if part == 2 && part2.is_empty() {
        part2.extend(part1.iter().cloned());
    }

    let files = match part {
        1 => part1,
        2 => part2,
        _ => unreachable!(),
    };

    if files.is_empty() {
        Err(Error::MissingInput)
    } else {
        let mut ret = Vec::new();
        for f in files {
            let output = if let Some(output) = f.strip_suffix(".txt") {
                let output = format!("{output}.expect{part}");
                if PathBuf::from(&output).is_file() {
                    Some(output)
                } else {
                    None
                }
            } else {
                None
            };

            ret.push((f, output));
        }
        Ok(ret)
    }
}

fn download_input(year: usize, day: usize) -> Result<(), Error> {
    let mut local = search_up("input_files", SearchType::Dir)?;
    local.push(format!("input-{year}-{day:02}.txt"));
    if local.is_file() {
        return Ok(());
    }
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    let cookies_path = search_up("cookies.txt", SearchType::File)?;
    let cookies = std::fs::read_to_string(cookies_path)?;
    let response = minreq::get(url).with_header("Cookie", cookies).send()?;

    std::fs::write(local, response.as_str()?)?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Cursor;

    const INPUT: &str = " # Hello

World
  
# Todo 
Yup  ";

    fn reader() -> Cursor<Vec<u8>> {
        Cursor::new(INPUT.into())
    }

    #[test]
    fn test_raw() -> Result<(), Error> {
        assert_eq!(
            Lines::from_reader(reader(), LinesOpt::RAW)?
                .iter()
                .collect::<Vec<&str>>(),
            [" # Hello", "", "World", "  ", "# Todo ", "Yup  "]
        );

        Ok(())
    }

    #[test]
    fn test_trim() -> Result<(), Error> {
        assert_eq!(
            Lines::from_reader(reader(), LinesOpt::TRIM)?
                .iter()
                .collect::<Vec<&str>>(),
            ["# Hello", "", "World", "", "# Todo", "Yup"]
        );

        Ok(())
    }

    #[test]
    fn test_remove_comments() -> Result<(), Error> {
        assert_eq!(
            Lines::from_reader(reader(), LinesOpt::REMOVE_COMMENTS)?
                .iter()
                .collect::<Vec<&str>>(),
            [" # Hello", "", "World", "  ", "Yup  "]
        );

        Ok(())
    }

    #[test]
    fn test_remove_empty() -> Result<(), Error> {
        assert_eq!(
            Lines::from_reader(reader(), LinesOpt::REMOVE_EMPTY)?
                .iter()
                .collect::<Vec<&str>>(),
            [" # Hello", "World", "  ", "# Todo ", "Yup  "]
        );

        Ok(())
    }

    #[test]
    fn test_trim_remove_comments() -> Result<(), Error> {
        assert_eq!(
            Lines::from_reader(reader(), LinesOpt::TRIM | LinesOpt::REMOVE_COMMENTS)?
                .iter()
                .collect::<Vec<&str>>(),
            ["", "World", "", "Yup"]
        );

        Ok(())
    }

    #[test]
    fn test_trim_remove_empty() -> Result<(), Error> {
        assert_eq!(
            Lines::from_reader(reader(), LinesOpt::TRIM | LinesOpt::REMOVE_EMPTY)?
                .iter()
                .collect::<Vec<&str>>(),
            ["# Hello", "World", "# Todo", "Yup"]
        );

        Ok(())
    }

    #[test]
    fn test_remove_empty_and_comments() -> Result<(), Error> {
        assert_eq!(
            Lines::from_reader(reader(), LinesOpt::REMOVE_COMMENTS | LinesOpt::REMOVE_EMPTY)?
                .iter()
                .collect::<Vec<&str>>(),
            [" # Hello", "World", "  ", "Yup  "]
        );

        Ok(())
    }

    #[test]
    fn test_all() -> Result<(), Error> {
        assert_eq!(
            Lines::from_reader(reader(), LinesOpt::ALL)?
                .iter()
                .collect::<Vec<&str>>(),
            ["World", "Yup"]
        );

        Ok(())
    }
}
