#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample-2.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NoSolution,
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
struct Cart {
    y: usize,
    x: usize,
    dir: u8,
    next_turn: u8,
    crashed: bool,
}

fn load_input(filename: &str) -> Result<(Vec<Vec<char>>, Vec<Cart>), Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();
    let mut carts = Vec::new();
    let mut map = Vec::new();
    let mut y = 0;

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;

        if line.trim() == "" {
            continue;
        }

        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            let c = match c {
                '^' => {
                    carts.push(Cart {
                        y: y,
                        x: x,
                        dir: 0,
                        next_turn: 0,
                        crashed: false,
                    });
                    '|'
                }
                '>' => {
                    carts.push(Cart {
                        y: y,
                        x: x,
                        dir: 1,
                        next_turn: 0,
                        crashed: false,
                    });
                    '-'
                }
                'v' => {
                    carts.push(Cart {
                        y: y,
                        x: x,
                        dir: 2,
                        next_turn: 0,
                        crashed: false,
                    });
                    '|'
                }
                '<' => {
                    carts.push(Cart {
                        y: y,
                        x: x,
                        dir: 3,
                        next_turn: 0,
                        crashed: false,
                    });
                    '-'
                }
                '|' | '-' | '/' | '\\' | ' ' | '+' => c,
                _ => unreachable!("wtf?  '{}'", c),
            };
            row.push(c);
        }

        map.push(row);
        y += 1;
    }

    Ok((map, carts))
}

fn print_map(map: &Vec<Vec<char>>, carts: &Vec<Cart>) {
    fn find_cart(x: usize, y: usize, carts: &Vec<Cart>) -> Option<&Cart> {
        for cart in carts {
            if cart.x == x && cart.y == y && !cart.crashed {
                return Some(cart);
            }
        }
        None
    }

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            print!(
                "{}",
                if let Some(cart) = find_cart(x, y, carts) {
                    match cart.dir {
                        0 => '^',
                        1 => '>',
                        2 => 'v',
                        3 => '<',
                        _ => unreachable!(),
                    }
                } else {
                    map[y][x]
                }
            );
        }
        println!();
    }
}

fn move_carts(map: &Vec<Vec<char>>, carts: &mut Vec<Cart>) -> (bool, usize, usize) {
    carts.sort();
    for i in 0..carts.len() {
        if carts[i].crashed {
            continue;
        }

        match carts[i].dir {
            0 => carts[i].y -= 1,
            1 => carts[i].x += 1,
            2 => carts[i].y += 1,
            3 => carts[i].x -= 1,
            _ => unreachable!(),
        }

        // Check crash
        for j in 0..carts.len() {
            if i == j {
                continue;
            }
            if carts[j].crashed {
                continue;
            }
            if carts[i].x == carts[j].x && carts[i].y == carts[j].y {
                carts[i].crashed = true;
                carts[j].crashed = true;
            }
        }

        // Check for turn
        let c = map[carts[i].y][carts[i].x];
        match c {
            '/' => match carts[i].dir {
                0 => carts[i].dir = 1,
                1 => carts[i].dir = 0,
                2 => carts[i].dir = 3,
                3 => carts[i].dir = 2,
                _ => unreachable!(),
            },
            '\\' => match carts[i].dir {
                0 => carts[i].dir = 3,
                1 => carts[i].dir = 2,
                2 => carts[i].dir = 1,
                3 => carts[i].dir = 0,
                _ => unreachable!(),
            },
            '+' => {
                match carts[i].next_turn {
                    0 => {
                        carts[i].dir = if carts[i].dir == 0 {
                            3
                        } else {
                            carts[i].dir - 1
                        }
                    }
                    1 => {}
                    2 => carts[i].dir = (carts[i].dir + 1) % 4,
                    _ => unreachable!(),
                }
                carts[i].next_turn = (carts[i].next_turn + 1) % 3;
            }
            _ => {}
        }
    }

    let mut still_alive = 0;
    let mut x = 0;
    let mut y = 0;
    for cart in carts {
        if !cart.crashed {
            still_alive += 1;
            x = cart.x;
            y = cart.y;
        }
    }

    if still_alive == 1 {
        return (true, x, y);
    }

    (false, 0, 0)
}

fn main() -> Result<(), Error> {
    let (map, mut carts) = load_input(INPUT_FILE)?;

    for cart in &carts {
        println!("Cart: {:?}", cart);
    }

    if cfg!(debug_assertions) {
        print_map(&map, &carts);
    }

    for i in 1..usize::MAX {
        let (crash, x, y) = move_carts(&map, &mut carts);
        if cfg!(debug_assertions) {
            println!("After {} moves", i);
            print_map(&map, &carts);
        }

        if crash {
            println!("Last cart on move {} at {},{}", i, x, y);
            return Ok(());
        }
    }

    Err(Error::NoSolution)
}
