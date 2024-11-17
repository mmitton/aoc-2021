#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};
use std::collections::BTreeMap;

struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

pub struct Day21 {
    foods: Vec<Food>,
    allergen_map: BTreeMap<String, String>,
    ingredient_map: BTreeMap<String, String>,
}

impl Day21 {
    pub fn new() -> Self {
        Self {
            foods: Vec::new(),
            allergen_map: BTreeMap::default(),
            ingredient_map: BTreeMap::default(),
        }
    }

    fn map(&mut self) {
        let mut allergens = Vec::new();
        for food in &self.foods {
            for allergen in food.allergens.iter() {
                if !allergens.contains(allergen) {
                    allergens.push(allergen.to_string());
                }
            }
        }

        let mut updated_maps = true;
        while updated_maps {
            updated_maps = false;
            for allergen in allergens.iter() {
                if self.allergen_map.contains_key(allergen) {
                    continue;
                }

                let mut common = Vec::new();
                for food in self.foods.iter() {
                    if food.allergens.contains(allergen) {
                        let mut list = Vec::new();
                        for ingredient in food.ingredients.iter() {
                            if !self.ingredient_map.contains_key(ingredient) {
                                list.push(ingredient.to_string());
                            }
                        }
                        assert!(!list.is_empty());
                        common.push(list);
                    }
                }

                assert!(!common.is_empty());
                if common.len() == 1 {
                    if common[0].len() == 1 {
                        self.allergen_map
                            .insert(allergen.to_string(), common[0][0].clone());
                        self.ingredient_map
                            .insert(common[0][0].clone(), allergen.to_string());
                        updated_maps = true;
                    }
                } else {
                    let mut in_all = Vec::new();
                    for i in 0..common[0].len() {
                        let ingredient = common[0][i].clone();
                        let mut found_in_all = true;
                        for common in common.iter().skip(1) {
                            if !common.contains(&ingredient) {
                                found_in_all = false;
                                break;
                            }
                        }

                        if found_in_all {
                            in_all.push(ingredient);
                        }
                    }

                    assert!(!in_all.is_empty());
                    if in_all.len() == 1 {
                        self.allergen_map
                            .insert(allergen.to_string(), in_all[0].clone());
                        self.ingredient_map
                            .insert(in_all[0].clone(), allergen.to_string());
                        updated_maps = true;
                    }
                }
            }
        }
    }
}

impl Runner for Day21 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            let line = line.replace(')', "");
            let parts: Vec<&str> = line.split(" (contains ").collect();

            let mut ingredients = Vec::new();
            let mut allergens = Vec::new();

            for ingredient in parts[0].split(' ') {
                ingredients.push(ingredient.to_string());
            }
            for allergen in parts[1].split(", ") {
                allergens.push(allergen.to_string());
            }

            self.foods.push(Food {
                ingredients,
                allergens,
            });
        }

        self.map();
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

impl Day21 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        let mut answer = 0;
        for food in self.foods.iter() {
            for ingredient in food.ingredients.iter() {
                if !self.ingredient_map.contains_key(ingredient.as_str()) {
                    answer += 1;
                }
            }
        }
        Ok(answer.into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut answer = Vec::new();
        for (_, ingredient) in self.allergen_map.iter() {
            answer.push(ingredient.as_str());
        }
        Ok(answer.join(",").into())
    }
}
