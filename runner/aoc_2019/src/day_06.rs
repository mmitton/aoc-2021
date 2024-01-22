use std::collections::HashMap;

#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};

pub struct Day06 {
    orbits: Vec<Vec<usize>>,
    you: usize,
    san: usize,
}

impl Day06 {
    pub fn new() -> Self {
        Self {
            orbits: Vec::new(),
            you: usize::MAX,
            san: usize::MAX,
        }
    }
}

impl Runner for Day06 {
    fn parse(&mut self, path: &str, _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_path(path, LinesOpt::RAW)?;
        let mut names: HashMap<&str, usize> = HashMap::with_capacity(lines.len() + 3);
        fn map_name<'a>(names: &mut HashMap<&'a str, usize>, name: &'a str) -> usize {
            if let Some(idx) = names.get(name) {
                *idx
            } else {
                let idx = names.len();
                names.insert(name, idx);
                idx
            }
        }

        let mut around = vec![usize::MAX; lines.len() + 3];
        for line in lines.iter() {
            let (key, value) = line.split_once(')').unwrap();
            let key = map_name(&mut names, key);
            let value = map_name(&mut names, value);
            around[value] = key;
        }

        self.you = map_name(&mut names, "YOU");
        self.san = map_name(&mut names, "SAN");
        let com_idx = *names.get("COM").unwrap();

        for key in 0..names.len() {
            let mut orbits = Vec::new();
            let mut entity = key;
            while entity != com_idx {
                entity = around[entity];
                if entity == usize::MAX {
                    break;
                }
                orbits.push(entity);
            }
            self.orbits.push(orbits);
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        let mut ans = 0;
        for orbits in self.orbits.iter() {
            ans += orbits.len();
        }
        Ok(ans.into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let you = &self.orbits[self.you];
        let san = &self.orbits[self.san];
        for (you_jumps, you) in you.iter().enumerate() {
            if let Some(san_jumps) = san.iter().position(|e| e == you) {
                return Ok((you_jumps + san_jumps).into());
            }
        }
        Err(Error::Unsolved)
    }
}
