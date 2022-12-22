const INPUT_FILE: &str = if cfg!(debug_assertions) {
    "../input-sample.txt"
} else {
    "../input.txt"
};

use std::fs::File;
use std::io::{BufRead, BufReader};

fn print(dir: i8, pos: (usize, usize), map: &Vec<Vec<char>>) {
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if pos == (col, row) {
                match dir {
                    0 => print!(">"),
                    1 => print!("v"),
                    2 => print!("<"),
                    3 => print!("^"),
                    _ => unreachable!(),
                }
            } else {
                print!("{}", map[row][col]);
            }
        }
        println!(" {row}");
    }
}

fn do_move(
    pos: (usize, usize),
    dir: i8,
    movement: u32,
    map: &mut Vec<Vec<char>>,
) -> (usize, usize) {
    let delta = match dir {
        0 => (1, 0),
        1 => (0, 1),
        2 => (-1, 0),
        3 => (0, -1),
        _ => unreachable!(),
    };

    // print(dir, (pos.0 as usize, pos.1 as usize), map);
    println!("do_move({pos:?}, {dir}, {movement}, ..)");
    let mut pos = (pos.0 as isize, pos.1 as isize);
    for _ in 0..movement {
        map[pos.1 as usize][pos.0 as usize] = match dir {
            0 => '>',
            1 => 'v',
            2 => '<',
            3 => '^',
            _ => unreachable!(),
        };
        // print(dir, (pos.0 as usize, pos.1 as usize), map);

        let mut next_pos = (pos.0 + delta.0, pos.1 + delta.1);
        if map[next_pos.1 as usize][next_pos.0 as usize] == ' ' {
            match dir {
                0 => {
                    // Wrap to left
                    for (idx, c) in map[next_pos.1 as usize].iter().enumerate() {
                        if *c != ' ' {
                            next_pos.0 = idx as isize;
                            break;
                        }
                    }
                }
                1 => {
                    // Wrap to top
                    for (idx, row) in map.iter().enumerate() {
                        let c = row[next_pos.0 as usize];
                        if c != ' ' {
                            next_pos.1 = idx as isize;
                            break;
                        }
                    }
                }
                2 => {
                    // Wrap to right
                    for (idx, c) in map[next_pos.1 as usize].iter().enumerate().rev() {
                        if *c != ' ' {
                            next_pos.0 = idx as isize;
                            break;
                        }
                    }
                }
                3 => {
                    // Wrap to bottom
                    for (idx, row) in map.iter().enumerate().rev() {
                        let c = row[next_pos.0 as usize];
                        if c != ' ' {
                            next_pos.1 = idx as isize;
                            break;
                        }
                    }
                }
                _ => unreachable!(),
            }
        }

        if map[next_pos.1 as usize][next_pos.0 as usize] == '#' {
            break;
        }

        pos = next_pos;
        map[pos.1 as usize][pos.0 as usize] = match dir {
            0 => '>',
            1 => 'v',
            2 => '<',
            3 => '^',
            _ => unreachable!(),
        };
    }

    print(dir, (pos.0 as usize, pos.1 as usize), map);
    (pos.0 as usize, pos.1 as usize)
}

fn main() {
    let lines: Vec<String> = {
        let file = File::open(INPUT_FILE).expect("Cannot open input file");
        BufReader::new(file).lines().flatten().collect()
    };

    let mut lines = lines.as_slice();
    let mut pos = (0, 0);
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut max_line = 0;
    map.push(Vec::new());
    loop {
        let line = &lines[0];
        lines = &lines[1..];

        if line.is_empty() {
            break;
        }

        // println!("{line}");
        let mut row = Vec::new();
        row.push(' ');
        for (i, c) in line.chars().enumerate() {
            if pos == (0, 0) && c == '.' {
                pos = (i + 1, 1);
            }
            row.push(c);
        }
        row.push(' ');
        if row.len() > max_line {
            max_line = row.len();
        }
        map.push(row);
    }
    map.push(Vec::new());

    for row in map.iter_mut() {
        while row.len() != max_line {
            row.push(' ');
        }
    }

    let inst = &lines[0];
    let mut dir = 0i8;

    let mut movement = 0u32;
    for c in inst.chars() {
        println!("c:{c}");
        match c {
            'L' => {
                pos = do_move(pos, dir, movement, &mut map);
                movement = 0;
                dir = (dir - 1) & 0b11;
                println!("turn left: dir:{dir}");
                map[pos.1][pos.0] = match dir {
                    0 => '>',
                    1 => 'v',
                    2 => '<',
                    3 => '^',
                    _ => unreachable!(),
                };
            }
            'R' => {
                pos = do_move(pos, dir, movement, &mut map);
                movement = 0;
                dir = (dir + 1) & 0b11;
                println!("turn right: dir:{dir}");
                map[pos.1][pos.0] = match dir {
                    0 => '>',
                    1 => 'v',
                    2 => '<',
                    3 => '^',
                    _ => unreachable!(),
                };
            }
            '0'..='9' => movement = (movement * 10) + (c as u32 - '0' as u32),
            _ => unreachable!(),
        }
    }
    pos = do_move(pos, dir, movement, &mut map);

    let ans = (pos.1 * 1000) + (pos.0 * 4) + dir as usize;
    println!("pos:{pos:?}  dir:{dir}");
    println!("ans: {ans}");
    // 123106 too high
}
