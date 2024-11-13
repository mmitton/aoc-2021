#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};
use std::str::FromStr;

#[derive(Clone, Debug)]
struct Sue {
    num: usize,
    known: HashMap<String, usize>,
}

impl Sue {
    fn matches(&self, part1: bool, results: &HashMap<String, usize>) -> bool {
        if part1 {
            for (rkey, rval) in results.iter() {
                if let Some(val) = self.known.get(rkey) {
                    if val != rval {
                        return false;
                    }
                }
            }
        } else {
            for (rkey, rval) in results.iter() {
                if let Some(val) = self.known.get(rkey) {
                    let matches = match rkey.as_str() {
                        "children" | "samoyeds" | "akitas" | "vizslas" | "cars" | "perfumes" => {
                            rval == val
                        }

                        "pomeranians" | "goldfish" => rval > val,

                        "cats" | "trees" => rval < val,
                        _ => unreachable!(),
                    };
                    if !matches {
                        return false;
                    }
                }
            }
        }

        true
    }
}

impl FromStr for Sue {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.find(": ").unwrap();
        let num: usize = (s[0..split].split(" ").collect::<Vec<&str>>())[1].parse()?;

        let mut known = HashMap::default();
        for part in s[split + 2..].split(", ") {
            let parts: Vec<&str> = part.split(": ").collect();
            known.insert(parts[0].to_string(), parts[1].parse()?);
        }

        Ok(Sue { num, known })
    }
}
#[derive(Default)]
pub struct Day16 {
    sues: Vec<Sue>,
}

impl Day16 {
    pub fn new() -> Self {
        Self::default()
    }

    fn base_results(&self) -> HashMap<String, usize> {
        let mut results = HashMap::default();
        results.insert("children".to_string(), 3);
        results.insert("cats".to_string(), 7);
        results.insert("samoyeds".to_string(), 2);
        results.insert("pomeranians".to_string(), 3);
        results.insert("akitas".to_string(), 0);
        results.insert("vizslas".to_string(), 0);
        results.insert("goldfish".to_string(), 5);
        results.insert("trees".to_string(), 3);
        results.insert("cars".to_string(), 2);
        results.insert("perfumes".to_string(), 1);

        results
    }
}

impl Runner for Day16 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            self.sues.push(line.parse()?);
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        let results = self.base_results();

        for sue in &self.sues {
            if sue.matches(true, &results) {
                return Ok(sue.num.into());
            }
        }
        Err(Error::Unsolved)
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let results = self.base_results();

        for sue in &self.sues {
            if sue.matches(false, &results) {
                return Ok(sue.num.into());
            }
        }
        Err(Error::Unsolved)
    }
}
