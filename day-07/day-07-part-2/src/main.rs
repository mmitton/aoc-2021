#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NAN(std::num::ParseIntError),
}

fn load_input(filename: &str) -> Result<Vec<usize>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let mut input = Vec::new();
    let lines = BufReader::new(f).lines();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }

        for num in line.split(",") {
            input.push(num.parse::<usize>().map_err(|e| Error::NAN(e))?);
        }
    }

    Ok(input)
}

fn calc_cost(input: &Vec<usize>, costs: &Vec<usize>, pos: usize) -> usize {
    let mut cost = 0usize;
    let pos = pos as isize;
    for i in 0..input.len() {
        cost += costs[(input[i] as isize - pos).abs() as usize];
    }

    cost
}

fn main() -> Result<(), Error> {
    let input = load_input(INPUT_FILE)?;

    if cfg!(debug_assertions) {
        println!("Initial state: {:?}", input);
    }

    let mut sum = 0usize;
    let mut max = 0usize;
    for num in &input {
        sum += num;
        if *num > max {
            max = *num;
        }
    }
    println!("sum:{}  avg:{}  max:{}", sum, sum / input.len(), max);

    let mut costs: Vec<usize> = vec![0; max + 1];
    for i in 1..costs.len() {
        costs[i] = costs[i - 1] + i;
    }

    let mut cost: Vec<usize> = vec![0; max + 1];
    let mut best_pos = sum / input.len();
    loop {
        for i in best_pos - 1..=best_pos + 1 {
            if cost[i] == 0 {
                cost[i] = calc_cost(&input, &costs, i);
            }
        }

        if cost[best_pos - 1] < cost[best_pos] {
            best_pos -= 1;
        } else if cost[best_pos + 1] < cost[best_pos] {
            best_pos += 1;
        } else {
            break;
        }
    }

    println!("best pos:{}  {}", best_pos, cost[best_pos]);

    Ok(())
}
