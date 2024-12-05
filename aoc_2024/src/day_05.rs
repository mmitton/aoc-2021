#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt};

#[derive(Default)]
pub struct Day05 {
    rules: Vec<(usize, usize)>,
    updates: Vec<Update>,
}

struct Update {
    rules: Vec<(usize, usize)>,
    map: HashMap<usize, usize>,
}

impl Update {
    fn new(rules: Vec<(usize, usize)>, pages: Vec<usize>) -> Self {
        let mut map = HashMap::default();
        for (i, n) in pages.iter().enumerate() {
            map.insert(*n, i);
        }

        Self { rules, map }
    }

    fn mid_page(&self) -> usize {
        let mid = self.map.len() / 2;
        *self.map.iter().find(|(_, pos)| **pos == mid).unwrap().0
    }

    fn process(&mut self, fix: bool) -> bool {
        let mut good = true;
        loop {
            let mut swapped = false;

            for rule in self.rules.iter() {
                if let (Some(b), Some(a)) = (self.map.get(&rule.0), self.map.get(&rule.1)) {
                    if b > a {
                        if !fix {
                            return false;
                        }
                        swapped = true;
                        let a = *a;
                        let b = *b;
                        self.map.insert(rule.0, a);
                        self.map.insert(rule.1, b);
                    }
                }
            }
            if !swapped {
                break;
            }
            good = false;
        }

        good
    }
}

impl Day05 {
    pub fn new() -> Self {
        Self::default()
    }

    fn process_updates(&mut self, fix: bool) -> Vec<&Update> {
        let mut ret = Vec::new();

        for update in self.updates.iter_mut() {
            if matches!((fix, update.process(fix)), (false, true) | (true, false)) {
                ret.push(update as &_);
            }
        }
        ret
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self
            .process_updates(false)
            .iter()
            .fold(0, |sum, update| sum + update.mid_page())
            .into())
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self
            .process_updates(true)
            .iter()
            .fold(0, |sum, update| sum + update.mid_page())
            .into())
    }
}

impl helper::Runner for Day05 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        let mut lines = lines.iter();
        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }
            let Some((b, a)) = line.split_once('|') else {
                return Err(Error::InvalidInput(line.into()));
            };
            self.rules.push((b.parse()?, a.parse()?));
        }
        for line in lines {
            let mut filter = [false; 100];
            let mut pages = Vec::with_capacity(100);
            for page in line.split(',') {
                let page: usize = page.parse()?;
                filter[page] = true;
                pages.push(page);
            }
            let mut rules: Vec<(usize, usize)> = Vec::with_capacity(pages.len());
            for rule in self.rules.iter() {
                if filter[rule.0] && filter[rule.1] {
                    rules.push(*rule);
                }
            }
            self.updates.push(Update::new(rules, pages));
        }

        Ok(())
    }

    fn run_part(&mut self, part: u8) -> Result<helper::RunOutput, Error> {
        match part {
            1 => self.part1(),
            2 => self.part2(),
            _ => Err(Error::Skipped),
        }
    }
}
