#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

pub struct Day12 {
    caves: Vec<Cave>,
    paths: Vec<Vec<usize>>,
}

#[derive(Debug)]
enum Cave {
    Start,
    End,
    Big,
    Little,
}

impl Day12 {
    pub fn new() -> Self {
        Self {
            caves: Vec::new(),
            paths: Vec::new(),
        }
    }

    fn find_paths(&mut self, allow_double: bool) -> usize {
        fn walk(
            at: usize,
            allow_double: bool,
            found_double: bool,
            caves: &mut Vec<Cave>,
            paths: &[Vec<usize>],
            visited: &mut Vec<usize>,
        ) -> usize {
            let mut found_paths = 0;
            for to in paths[at].iter().copied() {
                match &mut caves[to] {
                    Cave::Start => {}
                    Cave::End => found_paths += 1,
                    Cave::Big => {
                        found_paths += walk(to, allow_double, found_double, caves, paths, visited)
                    }
                    Cave::Little => {
                        visited[to] += 1;
                        if visited[to] == 1 {
                            found_paths +=
                                walk(to, allow_double, found_double, caves, paths, visited);
                        } else if visited[to] == 2 && allow_double && !found_double {
                            found_paths += walk(to, allow_double, true, caves, paths, visited);
                        }
                        visited[to] -= 1;
                    }
                }
            }
            found_paths
        }
        let mut visited = vec![0; self.caves.len()];
        walk(
            0,
            allow_double,
            false,
            &mut self.caves,
            &self.paths,
            &mut visited,
        )
    }
}

impl Runner for Day12 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        fn get_name(
            caves: &mut Vec<Cave>,
            paths: &mut Vec<Vec<usize>>,
            names: &mut HashMap<String, usize>,
            name: String,
        ) -> usize {
            let next = names.len();
            let cave = match name.as_str() {
                "start" => Cave::Start,
                "end" => Cave::End,
                _ => {
                    if name.chars().next().expect("Bad Name").is_lowercase() {
                        Cave::Little
                    } else {
                        Cave::Big
                    }
                }
            };
            use std::collections::hash_map::Entry;
            match names.entry(name) {
                Entry::Occupied(o) => *o.get(),
                Entry::Vacant(v) => {
                    v.insert(next);
                    caves.push(cave);
                    paths.push(Vec::new());
                    next
                }
            }
        }

        let mut names: HashMap<String, usize> = HashMap::default();
        get_name(&mut self.caves, &mut self.paths, &mut names, "start".into());
        get_name(&mut self.caves, &mut self.paths, &mut names, "end".into());
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            let (from, to) = line.split_once('-').expect("Bad line");

            let from = get_name(&mut self.caves, &mut self.paths, &mut names, from.into());
            let to = get_name(&mut self.caves, &mut self.paths, &mut names, to.into());

            self.paths[from].push(to);
            self.paths[to].push(from);
        }
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

impl Day12 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.find_paths(false).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self.find_paths(true).into())
    }
}
