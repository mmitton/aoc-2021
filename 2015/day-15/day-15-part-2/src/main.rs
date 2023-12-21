#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NAN(std::num::ParseIntError),
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

fn load_input(filename: &str) -> Result<Vec<Ingredient>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();

    let mut ingredients = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }

        let line = line.replace(": capacity ", " ");
        let line = line.replace(", durability ", " ");
        let line = line.replace(", flavor ", " ");
        let line = line.replace(", texture ", " ");
        let line = line.replace(", calories ", " ");
        let parts: Vec<&str> = line.split(" ").collect();

        let ingredient = Ingredient {
            _name: parts[0].to_string(),
            capacity: parts[1].parse().map_err(|e| Error::NAN(e))?,
            durability: parts[2].parse().map_err(|e| Error::NAN(e))?,
            flavor: parts[3].parse().map_err(|e| Error::NAN(e))?,
            texture: parts[4].parse().map_err(|e| Error::NAN(e))?,
            calories: parts[5].parse().map_err(|e| Error::NAN(e))?,
        };

        ingredients.push(ingredient);
    }

    Ok(ingredients)
}

fn main() -> Result<(), Error> {
    let ingredients = load_input(INPUT_FILE)?;
    const TARGET: isize = 100;

    let mut quant = vec![1isize; ingredients.len()];
    let mut best = 0;
    'out_loop: loop {
        for i in 0..ingredients.len() {
            if quant[i] != TARGET {
                quant[i] += 1;
                break;
            }

            if i == ingredients.len() - 1 {
                break 'out_loop;
            }

            quant[i] = 1;
        }
        let mut sum = 0;
        let mut capacity: isize = 0;
        let mut durability: isize = 0;
        let mut flavor: isize = 0;
        let mut texture: isize = 0;
        let mut calories: isize = 0;

        for i in 0..ingredients.len() {
            sum += quant[i];

            capacity += quant[i] * ingredients[i].capacity;
            durability += quant[i] * ingredients[i].durability;
            flavor += quant[i] * ingredients[i].flavor;
            texture += quant[i] * ingredients[i].texture;
            calories += quant[i] * ingredients[i].calories;
        }

        if sum != TARGET {
            continue;
        }

        if capacity < 0 {
            capacity = 0;
        }
        if durability < 0 {
            durability = 0;
        }
        if flavor < 0 {
            flavor = 0;
        }
        if texture < 0 {
            texture = 0;
        }

        if calories != 500 {
            continue;
        }

        let score = capacity * durability * flavor * texture;
        if score > best {
            best = score;
            println!(
                "new best: {}   {:?} {} {} {} {}",
                best, quant, capacity, durability, flavor, texture
            );
        }
    }

    Ok(())
}
