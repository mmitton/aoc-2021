#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NAN(std::num::ParseIntError),
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Error {
        Error::NAN(e)
    }
}

fn load_input(filename: &str) -> Result<BTreeMap<usize, (usize, usize, usize, usize)>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();
    let mut claims = BTreeMap::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim().to_string();
        if line == "" {
            continue;
        }

        let line = line.replace("#", "");
        let line = line.replace(": ", ",");
        let line = line.replace("x", ",");

        let parts: Vec<&str> = line.split(" @ ").collect();
        let claim_num: usize = parts[0].parse()?;

        let parts: Vec<&str> = parts[1].split(",").collect();
        let x: usize = parts[0].parse()?;
        let y: usize = parts[1].parse()?;
        let w: usize = parts[2].parse()?;
        let h: usize = parts[3].parse()?;

        claims.insert(claim_num, (x, y, w, h));
    }

    Ok(claims)
}

fn main() -> Result<(), Error> {
    let claims = load_input(INPUT_FILE)?;

    const WIDTH: usize = 1000;
    const HEIGHT: usize = 1000;

    let mut material = vec![0; WIDTH * HEIGHT];
    let mut ok = BTreeSet::new();

    for (claim_num, claim) in &claims {
        let mut is_ok = true;
        for x in claim.0..claim.0 + claim.2 {
            for y in claim.1..claim.1 + claim.3 {
                let idx = (y * WIDTH) + x;

                if material[idx] != 0 {
                    ok.remove(&material[idx]);
                    material[idx] = usize::MAX;
                    is_ok = false;
                } else {
                    material[idx] = *claim_num;
                }
            }
        }
        if is_ok {
            ok.insert(*claim_num);
        }
    }

    println!("Ok Claims: {:?}", ok);

    Ok(())
}
