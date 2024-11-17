#[allow(unused_imports)]
use helper::{
    print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, Permutations, RunOutput,
    Runner,
};

#[derive(Default, Debug)]
struct Rules {
    people: HashMap<String, usize>,
    rules: HashMap<(usize, usize), isize>,
}

impl Rules {
    fn add_person(&mut self, name: &str) -> usize {
        if let Some(idx) = self.people.get(name) {
            *idx
        } else {
            let next = self.people.len();
            self.people.insert(name.into(), next);
            next
        }
    }

    fn happiness(&self, arrangement: &[usize]) -> isize {
        let mut happiness = 0isize;

        for i in 0..arrangement.len() {
            let next_i = if i == arrangement.len() - 1 { 0 } else { i + 1 };
            if let Some(delta) = self.rules.get(&(arrangement[i], arrangement[next_i])) {
                happiness += delta;
            }
            if let Some(delta) = self.rules.get(&(arrangement[next_i], arrangement[i])) {
                happiness += delta;
            }
        }

        happiness
    }
}

#[derive(Default)]
pub struct Day13 {
    rules: Rules,
}

impl Day13 {
    pub fn new() -> Self {
        Self::default()
    }

    fn seat(&self, extra: usize) -> isize {
        let mut arrangement: Vec<usize> = (0..extra + self.rules.people.len()).collect();
        let mut max = isize::MIN;
        Permutations::iter_skip_last(&mut arrangement, |a| max = max.max(self.rules.happiness(a)));

        max
    }
}

impl Runner for Day13 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            let line = line.replace(".", "");
            let line = line.replace(" would ", " ");
            let line = line.replace(" happiness units by sitting next to ", " ");

            let parts: Vec<&str> = line.split(" ").collect();
            let mut gain: isize = parts[2].parse()?;
            if parts[1] == "lose" {
                gain = -gain;
            }
            let a = self.rules.add_person(parts[0]);
            let b = self.rules.add_person(parts[3]);

            self.rules.rules.insert((a, b), gain);
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

impl Day13 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.seat(0).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self.seat(1).into())
    }
}
