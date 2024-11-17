use std::str::FromStr;

#[allow(unused_imports)]
use helper::{
    print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, Permutations, RunOutput,
    Runner,
};

struct Quant {
    sum: isize,
    len: usize,
    quant: Vec<isize>,
}

impl Quant {
    fn new(len: usize, sum: isize) -> Self {
        Self {
            sum,
            len,
            quant: Vec::new(),
        }
    }

    fn set(&mut self, idx: usize, mut num: isize) -> bool {
        if idx != 0 {
            num = num.min(self.quant[idx - 1]);
        }
        self.quant[idx] = num;
        let remaining = self.sum - self.quant[..=idx].iter().sum::<isize>();
        let entries = self.quant.len() - 1 - idx;
        if entries == 0 {
            remaining == 0
        } else {
            self.set(idx + 1, remaining - entries as isize + 1)
        }
    }

    fn next(&mut self) -> Option<Vec<isize>> {
        if self.quant.is_empty() {
            self.quant.extend(0..self.len as isize);
            self.set(0, self.sum - self.len as isize + 1);
            Some(self.quant.clone())
        } else {
            for i in (2..self.len).rev() {
                if self.quant[i - 1] > self.quant[i] + 1 {
                    let prev = self.quant[i - 1];
                    if self.set(i - 1, prev - 1) {
                        // Good!
                        return Some(self.quant.clone());
                    } else {
                        self.quant[i - 1] = prev;
                    }
                }
            }
            if self.set(0, self.quant[0] - 1) {
                Some(self.quant.clone())
            } else {
                None
            }
        }
    }
}

#[derive(Debug)]
struct Ingredient {
    _name: String,
    capacity: isize,
    durability: isize,
    flavor: isize,
    texture: isize,
    calories: isize,
}

impl FromStr for Ingredient {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.replace(": capacity ", " ");
        let s = s.replace(", durability ", " ");
        let s = s.replace(", flavor ", " ");
        let s = s.replace(", texture ", " ");
        let s = s.replace(", calories ", " ");
        let parts: Vec<&str> = s.split(" ").collect();

        Ok(Ingredient {
            _name: parts[0].to_string(),
            capacity: parts[1].parse()?,
            durability: parts[2].parse()?,
            flavor: parts[3].parse()?,
            texture: parts[4].parse()?,
            calories: parts[5].parse()?,
        })
    }
}

#[derive(Default)]
pub struct Day15 {
    ingredients: Vec<Ingredient>,
}

impl Day15 {
    pub fn new() -> Self {
        Self::default()
    }

    fn best_cookie(&self, part2: bool) -> isize {
        const TARGET: isize = 100;

        let mut quant = Quant::new(self.ingredients.len(), TARGET);
        let mut best = 0;
        while let Some(mut quant) = quant.next() {
            Permutations::iter(&mut quant, |quant| {
                let mut sum = 0;
                let mut capacity: isize = 0;
                let mut durability: isize = 0;
                let mut flavor: isize = 0;
                let mut texture: isize = 0;
                let mut calories: isize = 0;

                for (ingredient, quant) in self.ingredients.iter().zip(quant.iter()) {
                    sum += quant;

                    capacity += quant * ingredient.capacity;
                    durability += quant * ingredient.durability;
                    flavor += quant * ingredient.flavor;
                    texture += quant * ingredient.texture;
                    calories += quant * ingredient.calories;
                }

                assert_eq!(sum, TARGET);

                if sum != TARGET || (part2 && calories != 500) {
                    return;
                }

                capacity = capacity.max(0);
                durability = durability.max(0);
                flavor = flavor.max(0);
                texture = texture.max(0);

                let score = capacity * durability * flavor * texture;
                best = best.max(score);
            });
        }

        best
    }
}

impl Runner for Day15 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            self.ingredients.push(line.parse()?);
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

impl Day15 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.best_cookie(false).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self.best_cookie(true).into())
    }
}
