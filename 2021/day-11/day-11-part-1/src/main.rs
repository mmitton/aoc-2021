#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
}

fn tick(map: &mut Vec<Vec<usize>>) -> usize {
    fn inc(map: &mut Vec<Vec<usize>>, x: usize, y: usize) -> usize {
        let mut flashes = 0usize;
        if map[y][x] != !0usize {
            map[y][x] += 1;
            if map[y][x] >= 10 {
                flashes += 1;

                map[y][x] = !0usize;
                let x1 = x as isize;
                let y1 = y as isize;
                for x2 in -1..=1 {
                    for y2 in -1..=1 {
                        if x2 == 0 && y2 == 0 {
                            continue;
                        }
                        let x = x1 + x2;
                        let y = y1 + y2;

                        if x < 0 || x as usize >= map[0].len() || y < 0 || y as usize >= map.len() {
                            continue;
                        }

                        flashes += inc(map, x as usize, y as usize);
                    }
                }
            }
        }

        flashes
    }

    let mut flashes = 0usize;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            flashes += inc(map, x, y);
        }
    }

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == !0usize {
                map[y][x] = 0;
            }
        }
    }

    flashes
}

fn load_input(filename: &str) -> Result<Vec<Vec<usize>>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();

    let mut ret = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            break;
        }

        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c as usize - '0' as usize);
        }
        ret.push(row);
    }

    Ok(ret)
}

fn main() -> Result<(), Error> {
    let mut map = load_input(INPUT_FILE)?;

    let mut flashes = 0usize;
    for _ in 0..100 {
        flashes += tick(&mut map);
    }

    println!("Total Flashes: {}", flashes);

    Ok(())
}
