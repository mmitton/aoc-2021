#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

use std::collections::BTreeMap;

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
}

fn load_input(filename: &str) -> Result<Vec<(Vec<String>, Vec<String>)>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();
    let mut foods = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim().to_string();
        if line == "" {
            continue;
        }

        let line = line.replace(")", "");
        let parts: Vec<&str> = line.split(" (contains ").collect();

        let mut ingredients = Vec::new();
        let mut allergens = Vec::new();

        for ingredient in parts[0].split(" ") {
            ingredients.push(ingredient.to_string());
        }
        for allergen in parts[1].split(", ") {
            allergens.push(allergen.to_string());
        }

        foods.push((ingredients, allergens));
    }

    Ok(foods)
}

fn main() -> Result<(), Error> {
    let foods = load_input(INPUT_FILE)?;

    let mut allergens = Vec::new();
    for food in &foods {
        for allergen in &food.1 {
            if !allergens.contains(&allergen) {
                allergens.push(allergen);
            }
        }
    }

    let mut allergen_map = BTreeMap::new();
    let mut ingredient_map = BTreeMap::new();
    let mut updated_maps = true;
    while updated_maps {
        updated_maps = false;
        for allergen in &allergens {
            if allergen_map.contains_key(&allergen) {
                continue;
            }

            let mut common = Vec::new();
            for food in &foods {
                if food.1.contains(&allergen) {
                    let mut list = Vec::new();
                    for ingredient in &food.0 {
                        if !ingredient_map.contains_key(&ingredient) {
                            list.push(ingredient);
                        }
                    }
                    assert!(list.len() > 0);
                    common.push(list);
                }
            }

            assert!(common.len() > 0);
            if common.len() == 1 {
                if common[0].len() == 1 {
                    allergen_map.insert(allergen, common[0][0]);
                    ingredient_map.insert(common[0][0], allergen);
                    updated_maps = true;
                }
            } else {
                let mut in_all = Vec::new();
                for i in 0..common[0].len() {
                    let ingredient = common[0][i];
                    let mut found_in_all = true;
                    for j in 1..common.len() {
                        if !common[j].contains(&ingredient) {
                            found_in_all = false;
                            break;
                        }
                    }

                    if found_in_all {
                        in_all.push(ingredient);
                    }
                }

                assert!(in_all.len() > 0);
                if in_all.len() == 1 {
                    allergen_map.insert(allergen, in_all[0]);
                    ingredient_map.insert(in_all[0], allergen);
                    updated_maps = true;
                }
            }
        }
    }

    println!("Allergens: {:?}", allergens);
    println!("Allergen Map: {:?}", allergen_map);
    println!("Ingredient Map: {:?}", ingredient_map);

    assert!(allergens.len() == allergen_map.len());
    assert!(allergens.len() == ingredient_map.len());

    let mut answer = Vec::new();
    for (_, ingredient) in &allergen_map {
        answer.push(ingredient.as_str());
    }

    println!("Answer: {}", answer.join(","));

    Ok(())
}
