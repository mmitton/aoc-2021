#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Default, Debug)]
struct Bag {
    contains: Vec<(usize, usize)>,
    inner_bags: Option<usize>,
    is_in: Vec<usize>,
}

pub struct Day07 {
    bags: Vec<Bag>,
}

impl Day07 {
    pub fn new() -> Self {
        Self { bags: Vec::new() }
    }

    fn inner_bags(&mut self, bag: usize) -> usize {
        if let Some(inner_bags) = self.bags[bag].inner_bags {
            inner_bags
        } else {
            let mut inner_bags = 0;
            for i in 0..self.bags[bag].contains.len() {
                inner_bags += (1 + self.inner_bags(self.bags[bag].contains[i].0))
                    * self.bags[bag].contains[i].1;
            }
            self.bags[bag].inner_bags = Some(inner_bags);
            inner_bags
        }
    }
}

impl Runner for Day07 {
    fn parse(&mut self, path: &str, _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_path(path, LinesOpt::RAW)?;
        let mut names: HashMap<&str, usize> = HashMap::default();

        macro_rules! name_index {
            ($name:expr) => {{
                if let Some(idx) = names.get($name) {
                    *idx
                } else {
                    names.insert($name, self.bags.len());
                    self.bags.push(Bag::default());
                    self.bags.len() - 1
                }
            }};
        }

        name_index!("shiny gold");

        for line in lines.iter() {
            let (name, contains) = line.split_once(" bags contain ").unwrap();
            let idx = name_index!(name);

            for contains in contains.split(", ") {
                if contains == "no other bags." {
                    break;
                }

                let first_space = contains
                    .chars()
                    .position(|c| c.is_ascii_whitespace())
                    .unwrap();
                let last_space = contains.len()
                    - 1
                    - contains
                        .chars()
                        .rev()
                        .position(|c| c.is_ascii_whitespace())
                        .unwrap();

                let idx2 = name_index!(&contains[first_space + 1..last_space]);
                self.bags[idx]
                    .contains
                    .push((idx2, contains[..first_space].parse()?));
                self.bags[idx2].is_in.push(idx);
            }
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        let mut work = vec![0];
        let mut seen = HashSet::default();
        seen.insert(0);

        while let Some(idx) = work.pop() {
            for idx in self.bags[idx].is_in.iter() {
                if !seen.contains(idx) {
                    work.push(*idx);
                    seen.insert(*idx);
                }
            }
        }
        Ok((seen.len() - 1).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self.inner_bags(0).into())
    }
}
