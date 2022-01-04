#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
}

fn load_input(filename: &str) -> Result<Vec<Vec<char>>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();

    let mut map = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim().to_string();
        if line == "" || line.starts_with("#") {
            continue;
        }

        map.push(line.chars().collect());
    }

    Ok(map)
}

fn print(map: &Vec<Vec<char>>) {
    let mut occupied = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let c = map[y][x];
            if c == '#' {
                occupied += 1;
            }
            print!("{}", c);
        }
        println!();
    }
    println!("{} seats occupied\n", occupied);
}

fn main() -> Result<(), Error> {
    let mut map = load_input(INPUT_FILE)?;

    let mut flips: Vec<(usize, usize)> = Vec::new();
    let mut changed = true;
    while changed {
        if cfg!(debug_assertions) {
            print(&map);
        }
        flips.clear();

        for y in 0..map.len() as isize {
            for x in 0..map[0].len() as isize {
                let mut occupied = 0;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }

                        let x = x + dx;
                        let y = y + dy;
                        if x < 0 || y < 0 {
                            continue;
                        }

                        let x = x as usize;
                        let y = y as usize;
                        if x == map[0].len() || y == map.len() {
                            continue;
                        }

                        if map[y][x] == '#' {
                            occupied += 1;
                        }
                    }
                }

                let x = x as usize;
                let y = y as usize;
                if map[y][x] == 'L' && occupied == 0 {
                    flips.push((x, y));
                } else if map[y][x] == '#' && occupied >= 4 {
                    flips.push((x, y));
                }
            }
        }

        changed = flips.len() != 0;
        for flip in &flips {
            if map[flip.1][flip.0] == '#' {
                map[flip.1][flip.0] = 'L';
            } else if map[flip.1][flip.0] == 'L' {
                map[flip.1][flip.0] = '#';
            } else {
                panic!();
            }
        }
    }

    print(&map);
    Ok(())
}
