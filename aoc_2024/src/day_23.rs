#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt};

#[derive(Default)]
pub struct Day23 {
    links: HashMap<String, HashSet<String>>,
}

impl Day23 {
    pub fn new() -> Self {
        Self::default()
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        let mut ans = 0;
        for (a, links) in self.links.iter() {
            for b in links.iter() {
                if b < a {
                    continue;
                }
                for c in links.iter() {
                    if c < b {
                        continue;
                    }
                    if self.links[b].contains(c)
                        && (a.starts_with('t') || b.starts_with('t') || c.starts_with('t'))
                    {
                        ans += 1;
                    }
                }
            }
        }
        Ok(ans.into())
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        let mut best = Vec::new();
        for a in self.links.keys() {
            if self.links[a.as_str()].iter().any(|b| b < a) {
                continue;
            }
            let mut groups: Vec<Vec<&str>> = vec![vec![a.as_str()]];

            for b in self.links[a.as_str()].iter() {
                let mut new_groups: Vec<_> = groups
                    .iter()
                    .filter_map(|g| {
                        for c in g.iter() {
                            if !self.links[b.as_str()].contains(*c) {
                                return None;
                            }
                        }
                        let mut g = g.clone();
                        g.push(b.as_str());
                        if g.len() > best.len() {
                            best.clear();
                            best.extend(g.iter().copied());
                        }
                        Some(g)
                    })
                    .collect();
                groups.append(&mut new_groups);
            }
        }
        best.sort();
        Ok(best.join(",").into())
    }
}

impl helper::Runner for Day23 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            let Some((a, b)) = line.split_once('-') else {
                return Err(Error::InvalidInput(line.into()));
            };
            self.links.entry(a.into()).or_default().insert(b.into());
            self.links.entry(b.into()).or_default().insert(a.into());
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
