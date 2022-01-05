#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

use std::collections::BTreeSet;

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::IO(e)
    }
}

struct State {
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
    min_z: isize,
    max_z: isize,
    min_w: isize,
    max_w: isize,
    on: BTreeSet<(isize, isize, isize, isize)>,
}

impl State {
    fn print(&self) {
        for w in self.min_w..=self.max_w {
            for z in self.min_z..=self.max_z {
                println!("\nz={} w={}", z, w);
                for y in self.min_y..=self.max_y {
                    for x in self.min_x..=self.max_x {
                        if self.on.contains(&(x, y, z, w)) {
                            print!("#");
                        } else {
                            print!(".");
                        }
                    }
                    println!();
                }
            }
        }
    }

    fn next_cycle(&self) -> Self {
        let mut next = State {
            min_x: isize::MAX,
            max_x: isize::MIN,
            min_y: isize::MAX,
            max_y: isize::MIN,
            min_z: isize::MAX,
            max_z: isize::MIN,
            min_w: isize::MAX,
            max_w: isize::MIN,
            on: BTreeSet::new(),
        };

        macro_rules! insert {
            ($x:expr, $y:expr, $z:expr, $w:expr) => {
                if $x < next.min_x {
                    next.min_x = $x
                }
                if $x > next.max_x {
                    next.max_x = $x
                }
                if $y < next.min_y {
                    next.min_y = $y
                }
                if $y > next.max_y {
                    next.max_y = $y
                }
                if $z < next.min_z {
                    next.min_z = $z
                }
                if $z > next.max_z {
                    next.max_z = $z
                }
                if $w < next.min_w {
                    next.min_w = $w
                }
                if $w > next.max_w {
                    next.max_w = $w
                }

                next.on.insert(($x, $y, $z, $w));
            };
        }

        macro_rules! neighbors_on {
            ($x:expr, $y:expr, $z:expr, $w:expr) => {{
                let mut count = 0;
                let mut considered = 0;
                for nx in $x - 1..=$x + 1 {
                    for ny in $y - 1..=$y + 1 {
                        for nz in $z - 1..=$z + 1 {
                            for nw in $w - 1..=$w + 1 {
                                if nx == $x && ny == $y && nz == $z && nw == $w {
                                    continue;
                                }
                                considered += 1;
                                if self.on.contains(&(nx, ny, nz, nw)) {
                                    count += 1;
                                }
                            }
                        }
                    }
                }

                assert!(considered == (3 * 3 * 3 * 3) - 1);

                count
            }};
        }
        for w in self.min_w - 1..=self.max_w + 1 {
            for z in self.min_z - 1..=self.max_z + 1 {
                for y in self.min_y - 1..=self.max_y + 1 {
                    for x in self.min_x - 1..=self.max_x + 1 {
                        let ncount = neighbors_on!(x, y, z, w);
                        if self.on.contains(&(x, y, z, w)) {
                            if ncount == 2 || ncount == 3 {
                                insert!(x, y, z, w);
                            }
                        } else {
                            if ncount == 3 {
                                insert!(x, y, z, w);
                            }
                        }
                    }
                }
            }
        }

        next
    }
}

fn load_input(filename: &str) -> Result<State, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let reader = BufReader::new(f);
    let lines = reader.lines();
    let mut state = State {
        min_x: 0,
        max_x: 0,
        min_y: 0,
        max_y: 0,
        min_z: 0,
        max_z: 0,
        min_w: 0,
        max_w: 0,
        on: BTreeSet::new(),
    };

    let mut y = 0;
    for line in lines {
        let line = line.unwrap();
        let line = line.trim();

        if line == "" {
            continue;
        }

        let chars = line.chars().collect::<Vec<char>>();
        for (x, c) in chars.iter().enumerate() {
            let x = x as isize;
            let c = *c;
            if c == '#' {
                state.on.insert((x, y, 0, 0));
                if x > state.max_x {
                    state.max_x = x;
                }
            }
        }

        state.max_y = y;
        y += 1;
    }

    Ok(state)
}

fn main() -> Result<(), Error> {
    let mut state = load_input(INPUT_FILE)?;

    state.print();
    for c in 1..=6 {
        state = state.next_cycle();
        if cfg!(debug_assertions) {
            println!("\nAfter cycle {}", c);
            state.print();
        }
    }

    println!("Total On: {}", state.on.len());

    Ok(())
}
