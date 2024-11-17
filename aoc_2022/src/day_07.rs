#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesIter, LinesOpt, Output, RunOutput, Runner};
use std::collections::BTreeMap;
use std::iter::Peekable;

#[derive(Default, Debug)]
struct Folder {
    file_size: usize,
    folders: BTreeMap<String, Folder>,
    total_size: Option<usize>,
}

impl Folder {
    fn calc_size(&mut self) -> usize {
        if let Some(total_size) = self.total_size {
            return total_size;
        }
        let mut total_size = self.file_size;
        for folder in self.folders.values_mut() {
            if let Some(folder_size) = folder.total_size {
                total_size += folder_size;
            } else {
                total_size += folder.calc_size();
            }
        }

        self.total_size = Some(total_size);
        total_size
    }

    fn filter<F>(&mut self, matches: &mut Vec<usize>, f: F)
    where
        F: Fn(usize) -> bool + Copy + Clone,
    {
        let total_size = self.calc_size();
        if f(total_size) {
            matches.push(total_size);
        }
        for folder in self.folders.values_mut() {
            folder.filter(matches, f);
        }
    }

    // returns true to return to root.  otherwise it will return on "cd .." or end of input
    fn parse(&mut self, is_root: bool, iter: &mut Peekable<LinesIter>) -> Result<bool, Error> {
        while let Some(line) = iter.next() {
            match line {
                "$ cd /" => {
                    if !is_root {
                        return Ok(true);
                    }
                }
                "$ cd .." => {
                    if !is_root {
                        return Ok(false);
                    }
                }
                "$ ls" => {
                    // List files and folders
                    while let Some(line) = iter.next_if(|s| !s.starts_with('$')) {
                        let (size, _) = line.split_once(' ').unwrap();
                        if size != "dir" {
                            self.file_size += size.parse::<usize>()?;
                        }
                    }
                }
                _ if line.starts_with("$ cd ") => {
                    let name = line.split_whitespace().last().unwrap();
                    if self
                        .folders
                        .entry(name.into())
                        .or_default()
                        .parse(false, iter)?
                        && !is_root
                    {
                        return Ok(true);
                    }
                }
                _ => unreachable!(),
            }
        }

        Ok(false)
    }
}

pub struct Day07 {
    root: Folder,
}

impl Day07 {
    pub fn new() -> Self {
        Self {
            root: Folder {
                file_size: 0,
                folders: BTreeMap::new(),
                total_size: None,
            },
        }
    }
}

impl Runner for Day07 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        let mut lines = lines.iter().peekable();
        self.root.parse(true, &mut lines)?;
        Ok(())
    }

    fn run_part(&mut self, part: u8) -> Result<RunOutput, Error> {
        match part {
            1 => self.part1(),
            2 => self.part2(),
            _ => Err(Error::Skipped),
        }
    }
}

impl Day07 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        self.root.calc_size();
        let mut matches = Vec::new();
        self.root.filter(&mut matches, |size| size <= 100000);
        Ok(matches.iter().sum::<usize>().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let free_space = 70000000 - self.root.calc_size();
        let needed_space = 30000000 - free_space;

        let mut matches = Vec::new();
        self.root.filter(&mut matches, |size| size >= needed_space);
        Ok(matches
            .iter()
            .fold(usize::MAX, |acc, size| acc.min(*size))
            .into())
    }
}
