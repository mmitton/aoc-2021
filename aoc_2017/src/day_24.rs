#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Copy, Clone)]
struct Component(u8, u8);

#[derive(Copy, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Bridge {
    strength: usize,
    components: u64,
    end: u8,
}

#[derive(Default)]
pub struct Day24 {
    components: Vec<Component>,
}

impl Day24 {
    pub fn new() -> Self {
        Self::default()
    }

    fn build_bridges(&self) -> Vec<Bridge> {
        let mut bridges = HashSet::default();
        bridges.insert(Bridge::default());

        let mut work = vec![Bridge::default()];
        while let Some(bridge) = work.pop() {
            for (idx, component) in self.components.iter().enumerate() {
                let mask = 1u64 << idx;
                macro_rules! attach {
                    ($a:expr, $b:expr) => {{
                        if bridge.end == $a && bridge.components & mask == 0 {
                            let mut bridge = bridge;
                            bridge.components |= mask;
                            bridge.strength += component.0 as usize + component.1 as usize;
                            bridge.end = $b;
                            if bridges.insert(bridge) {
                                work.push(bridge);
                            }
                        }
                    }};
                }
                attach!(component.0, component.1);
                attach!(component.1, component.0);
            }
        }

        bridges.drain().collect()
    }
}

impl Runner for Day24 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            let parts: Vec<&str> = line.split('/').collect();
            let n1: u8 = parts[0].parse()?;
            let n2: u8 = parts[1].parse()?;
            self.components.push(Component(n1, n2));
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        let mut bridges = self.build_bridges();
        bridges.sort();
        Ok(bridges.last().unwrap().strength.into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut bridges = self.build_bridges();
        bridges.sort_by(|a, b| {
            let a_len = a.components.count_ones();
            let b_len = b.components.count_ones();
            if a_len == b_len {
                a.strength.cmp(&b.strength)
            } else {
                a_len.cmp(&b_len)
            }
        });
        Ok(bridges.last().unwrap().strength.into())
    }
}
