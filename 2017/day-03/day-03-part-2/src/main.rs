use std::collections::BTreeMap;

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

    let lines = BufReader::new(f).lines();

    let mut addresses = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }
        if line.starts_with("#") {
            continue;
        }

        addresses.push(line.parse().map_err(|e| Error::NAN(e))?);
    }

    Ok(addresses)
}

fn map_address(address: usize) -> (isize, isize, isize) {
    if address == 1 {
        return (0, 0, 0);
    }

    let mut ring = 1;
    let mut ring_size = 9;
    let mut ring_start = 2;
    while ring_start + ring_size < address {
        ring += 1;
        ring_start += ring_size;
        ring_size += 8;
    }

    let mut x: isize = ring as isize;
    let mut y: isize = 0;
    let mut left_over = address - ring_start;

    if left_over > 0 {
        // Move Up!
        if left_over <= ring {
            y -= left_over as isize;
            left_over = 0;
        } else {
            y -= ring as isize;
            left_over -= ring;
        }
    }
    if left_over > 0 {
        // Move left!
        if left_over <= ring * 2 {
            x -= left_over as isize;
            left_over = 0;
        } else {
            x -= ring as isize * 2;
            left_over -= ring * 2;
        }
    }
    if left_over > 0 {
        // Move down!
        if left_over <= ring * 2 {
            y += left_over as isize;
            left_over = 0;
        } else {
            y += ring as isize * 2;
            left_over -= ring * 2;
        }
    }
    if left_over > 0 {
        // Move right!
        if left_over <= ring * 2 + 1 {
            x += left_over as isize;
            left_over = 0;
        } else {
            x += ring as isize * 2 + 1;
            left_over -= ring * 2 + 1;
        }
    }
    if left_over > 0 {
        // Move up!
        y -= left_over as isize;
    }

    let dx = if x < 0 { -x } else { x };
    let dy = if y < 0 { -y } else { y };

    (x, y, dx + dy)
}

fn stress(max: usize) -> usize {
    let mut map: BTreeMap<(isize, isize), usize> = BTreeMap::new();
    let mut mem: Vec<usize> = Vec::new();

    map.insert((0, 0), 0);
    mem.push(1);

    let mut addr = 2;
    loop {
        let (x, y, _) = map_address(addr);
        let mut sum = 0;
        for x1 in -1..=1 {
            for y1 in -1..=1 {
                if let Some(idx) = map.get(&(x + x1, y + y1)) {
                    sum += mem[*idx];
                }
            }
        }
        map.insert((x, y), mem.len());
        mem.push(sum);

        if sum > max {
            return sum;
        }

        addr += 1;
    }
}

fn main() -> Result<(), Error> {
    let addresses = load_input(INPUT_FILE)?;

    for address in &addresses {
        let ans = stress(*address);
        println!("{} .. answer {}", *address, ans);
    }

    Ok(())
}
