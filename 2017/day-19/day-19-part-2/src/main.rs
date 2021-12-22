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

    let mut map: Vec<Vec<char>> = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        if line == "" || line.starts_with("#") {
            continue;
        }

        map.push(line.chars().collect());
    }

    Ok(map)
}

#[derive(Debug)]
enum Dir {
    North,
    South,
    East,
    West,
}

fn main() -> Result<(), Error> {
    let map = load_input(INPUT_FILE)?;
    let max_x = map[0].len() as isize - 1;
    let max_y = map.len() as isize - 1;

    let mut letters: Vec<char> = Vec::new();

    let mut x: isize = isize::MAX;
    let mut y: isize = 0;
    let mut dir = Dir::South;

    for i in 0..map[0].len() {
        if map[0][i] == '|' {
            x = i as isize;
            break;
        }
    }
    assert!(x != isize::MAX);

    let mut steps = 0;
    'walk_loop: loop {
        steps += 1;
        let c = map[y as usize][x as usize];
        if cfg!(debug_assertions) {
            println!(
                "pos: {},{}  c: {}  dir: {:?}  steps:{}",
                x, y, c, dir, steps
            );
        }
        if c.is_alphabetic() {
            letters.push(c);
        }

        let (next_x, next_y) = match dir {
            Dir::North => (x, y - 1),
            Dir::South => (x, y + 1),
            Dir::East => (x - 1, y),
            Dir::West => (x + 1, y),
        };

        if next_x < 0
            || next_x > max_x
            || next_y < 0
            || next_y > max_y
            || map[next_y as usize][next_x as usize] == ' '
        {
            // Change direction
            match dir {
                Dir::North | Dir::South => {
                    if x != 0 && map[y as usize][x as usize - 1] != ' ' {
                        x = x - 1;
                        dir = Dir::East;
                    } else if x != max_x && map[y as usize][x as usize + 1] != ' ' {
                        x = x + 1;
                        dir = Dir::West;
                    } else {
                        break 'walk_loop;
                    }
                }
                Dir::West | Dir::East => {
                    if y != 0 && map[y as usize - 1][x as usize] != ' ' {
                        y = y - 1;
                        dir = Dir::North;
                    } else if y != max_y && map[y as usize + 1][x as usize] != ' ' {
                        y = y + 1;
                        dir = Dir::South;
                    } else {
                        break 'walk_loop;
                    }
                }
            }
        } else {
            x = next_x;
            y = next_y;
        }
    }

    println!("Letters: {}", letters.iter().collect::<String>());
    println!("Steps: {}", steps);

    Ok(())
}
