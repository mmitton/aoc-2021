#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
}

#[derive(Debug)]
struct Present {
    l: usize,
    w: usize,
    h: usize,
}

impl Present {
    fn wrapping_paper(&self) -> usize {
        let mut min = self.l * self.w;
        if self.w * self.h < min {
            min = self.w * self.h;
        }
        if self.h * self.l < min {
            min = self.h * self.l;
        }

        (2 * self.l * self.w) + (2 * self.w * self.h) + (2 * self.h * self.l) + min
    }
}

fn load_input(filename: &str) -> Result<Vec<Present>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();

    let mut presents = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }

        let parts: Vec<&str> = line.split("x").collect();
        let l: usize = parts[0].parse().unwrap();
        let w: usize = parts[1].parse().unwrap();
        let h: usize = parts[2].parse().unwrap();

        presents.push(Present { l: l, w: w, h: h });
    }

    Ok(presents)
}

fn main() -> Result<(), Error> {
    let presents = load_input(INPUT_FILE)?;

    let mut total = 0;
    for present in &presents {
        if cfg!(debug_assertions) {
            println!("presents: {:?}  {}", present, present.wrapping_paper());
        }
        total += present.wrapping_paper();
    }

    println!("total: {}", total);

    Ok(())
}
