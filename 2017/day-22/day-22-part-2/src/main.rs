#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
}

use std::collections::BTreeMap;

fn print_map(infected: &BTreeMap<(isize, isize), State>) {
    if infected.len() == 0 {
        println!("No Infections.");
        return;
    }

    let mut min_x = isize::MAX;
    let mut min_y = isize::MAX;
    let mut max_x = isize::MIN;
    let mut max_y = isize::MIN;
    for (x, y) in infected.keys() {
        if *x < min_x {
            min_x = *x
        }
        if *x > max_x {
            max_x = *x
        }
        if *y < min_y {
            min_y = *y
        }
        if *y > max_y {
            max_y = *y
        }
    }

    println!("Infections: {}", infected.len());
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let c = match infected.get(&(x, y)) {
                None => ".",
                Some(State::Weakened) => "W",
                Some(State::Flagged) => "F",
                Some(State::Infected) => "#",
            };
            print!("{}", c);
        }
        println!();
    }
}

enum State {
    Weakened,
    Flagged,
    Infected,
}

#[derive(Debug)]
struct Carrier {
    x: isize,
    y: isize,
    dir: u8,
}

impl Carrier {
    fn burst(&mut self, infected: &mut BTreeMap<(isize, isize), State>) -> bool {
        let mut set_infected = false;
        let state = infected.get(&(self.x, self.y));
        match state {
            None => {
                if self.dir == 0 {
                    self.dir = 3;
                } else {
                    self.dir -= 1;
                }
                infected.insert((self.x, self.y), State::Weakened);
            }
            Some(State::Weakened) => {
                infected.insert((self.x, self.y), State::Infected);
                set_infected = true;
            }
            Some(State::Infected) => {
                if self.dir == 3 {
                    self.dir = 0;
                } else {
                    self.dir += 1;
                }
                infected.insert((self.x, self.y), State::Flagged);
            }
            Some(State::Flagged) => {
                match self.dir {
                    0 => self.dir = 2,
                    1 => self.dir = 3,
                    2 => self.dir = 0,
                    3 => self.dir = 1,
                    _ => unreachable!(),
                }
                infected.remove(&(self.x, self.y));
            }
        }

        match self.dir {
            0 => self.y -= 1,
            1 => self.x += 1,
            2 => self.y += 1,
            3 => self.x -= 1,
            _ => unreachable!(),
        }

        set_infected
    }
}

fn load_input(filename: &str) -> Result<(Carrier, BTreeMap<(isize, isize), State>), Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;
    let lines = BufReader::new(f).lines();

    let mut infected: BTreeMap<(isize, isize), State> = BTreeMap::new();
    let mut carrier = Carrier { x: 0, y: 0, dir: 0 };

    let mut y = 0isize;
    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }

        carrier.x = line.len() as isize / 2;
        carrier.y = y / 2;

        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                infected.insert((x as isize, y), State::Infected);
            }
        }
        y += 1;
    }

    Ok((carrier, infected))
}

fn main() -> Result<(), Error> {
    let (mut carrier, mut infected) = load_input(INPUT_FILE)?;

    let mut answer = 0usize;
    for _ in 0..10000000 {
        if carrier.burst(&mut infected) {
            answer += 1;
        }
        // print_map(&infected);
    }

    println!("Answer: {}", answer);

    Ok(())
}
