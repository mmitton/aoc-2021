const INPUT_FILE: &str = if cfg!(debug_assertions) {
    "../input-sample.txt"
} else {
    "../input.txt"
};

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn print(
    dir: i8,
    pos: (usize, usize),
    map: &[Vec<char>],
    warps: &BTreeMap<(usize, usize), Vec<(usize, usize)>>,
) {
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
                if let Some(v) = warps.get(&(col, row)) {
                    assert!(map[row][col] == ' ');
                    print!("{}", v.len());
                } else {
                    print!("{}", map[row][col]);
                }
            }
        }
        println!(" {row}");
    }
}

/*
fn do_move(
    pos: (usize, usize),
    dir: i8,
    movement: u32,
    map: &mut Vec<Vec<char>>,
    sides: &Vec<Vec<Surface>>,
) -> (usize, usize) {
    println!("do_move({pos:?}, {dir}, {movement}, ..)");
    let mut pos = pos;
    for _ in 0..movement {
        map[pos.1][pos.0] = match dir {
            0 => '>',
            1 => 'v',
            2 => '<',
            3 => '^',
            _ => unreachable!(),
        };
        // print(dir, pos, map);
        let mut next_pos = match dir {
            0 => (pos.0 + 1, pos.1),
            1 => (pos.0, pos.1 + 1),
            2 => (pos.0 - 1, pos.1),
            3 => (pos.0, pos.1 - 1),
            _ => unreachable!(),
        };

        if map[next_pos.1][next_pos.0] == ' ' {
            let mut vector = sides[pos.1][pos.0];
            // rotate the vector
            match dir {
                0 => vector.rotate(0, -1, 0),
                1 => vector.rotate(-1, 0, 0),
                2 => vector.rotate(0, 1, 0),
                3 => vector.rotate(1, 0, 0),
                _ => unreachable!(),
            }

            println!(
                "Looking for partner of {pos:?} {vector:?} {:?}",
                sides[pos.1][pos.0]
            );

            let mut new_pos: Option<(usize, usize)> = None;
            // Find the other tile with the same normal vector
            'search_loop: for (y, sides) in sides.iter().enumerate() {
                for (x, side) in sides.iter().enumerate() {
                    if x == pos.0 && y == pos.1 {
                        continue;
                    }
                    if side.pos == vector.pos && side.normal == vector.normal {
                        new_pos = Some((x, y));
                        break 'search_loop;
                    }
                }
            }
            panic!("{:?}", new_pos);
        }

        if map[next_pos.1][next_pos.0] == '#' {
            break;
        }

        pos = next_pos;
        map[pos.1][pos.0] = match dir {
            0 => '>',
            1 => 'v',
            2 => '<',
            3 => '^',
            _ => unreachable!(),
        };
    }

    print(dir, pos, map);
    pos
}
*/

fn zip(
    warps: &mut BTreeMap<(usize, usize), Vec<(usize, usize)>>,
    map: &[Vec<char>],
    mut p1: (usize, usize),
    mut d1: (isize, isize),
    mut p2: (usize, usize),
    mut d2: (isize, isize),
    e: (usize, usize),
) {
    let mut t1 = 0;
    let mut t2 = 0;
    let mut e1 = e;
    let mut e2 = e;

    while t1 == 0 || t2 == 0 {
        println!("{p1:?} {d1:?} {e1:?} {t1:?}  {p2:?} {d2:?} {e2:?} {t2:?}");
        warps
            .entry(e1)
            .and_modify(|v| v.push(p2))
            .or_insert(vec![p2]);
        warps
            .entry(e2)
            .and_modify(|v| v.push(p1))
            .or_insert(vec![p1]);

        macro_rules! next {
            ($p:expr, $d:expr) => {
                (
                    ($p.0 as isize + $d.0 as isize) as usize,
                    ($p.1 as isize + $d.1 as isize) as usize,
                )
            };
        }
        macro_rules! walk {
            ($p:expr, $e:expr, $d:expr, $t:expr) => {{
                let new_p = next!($p, $d);
                if map[new_p.1][new_p.0] == ' ' {
                    // Turn!
                    if $d.0 != 0 {
                        if $e.1 == $p.1 - 1 {
                            $d = (0, 1);
                        } else {
                            $d = (0, -1);
                        }
                    } else {
                        if $e.0 == $p.0 - 1 {
                            $d = (1, 0);
                        } else {
                            $d = (-1, 0);
                        }
                    }

                    $t += 1;
                    $e = new_p;
                } else {
                    $e = next!($e, $d);
                    $p = new_p;
                }
            }};
        }

        // Move p1
        walk!(p1, e1, d1, t1);

        // Move p2
        walk!(p2, e2, d2, t2);
    }
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

    // Calculate the length of a side
    let mut side_lens = [0; 4];
    for i in 0..max_line {
        if map[1][i] != ' ' {
            side_lens[0] += 1;
        }
        if map[map.len() - 2][i] != ' ' {
            side_lens[1] += 1;
        }
    }
    for i in 0..map.len() {
        if map[i][1] != ' ' {
            side_lens[2] += 1;
        }
        if map[i][max_line - 2] != ' ' {
            side_lens[3] += 1;
        }
    }
    side_lens.sort();
    let side_len = side_lens[0];

    println!("{side_lens:?}  {side_len}");
    let mut warps: BTreeMap<(usize, usize), Vec<(usize, usize)>> = BTreeMap::new();
    print(0, pos, &map, &warps);

    // Find the inside corners
    for y in 1..map.len() - 1 {
        for x in 1..max_line - 1 {
            if map[y][x] == ' ' {
                if map[y + 1][x] != ' ' && map[y][x + 1] != ' ' {
                    let p1 = (x, y + 1);
                    let p2 = (x + 1, y);
                    let d1 = (-1, 0);
                    let d2 = (0, -1);
                    let e = (x, y);

                    zip(&mut warps, &map, p1, d1, p2, d2, e);
                }
                if map[y + 1][x] != ' ' && map[y][x - 1] != ' ' {
                    let p1 = (x, y + 1);
                    let p2 = (x - 1, y);
                    let d1 = (1, 0);
                    let d2 = (0, -1);
                    let e = (x, y);

                    zip(&mut warps, &map, p1, d1, p2, d2, e);
                }
                if map[y - 1][x] != ' ' && map[y][x + 1] != ' ' {
                    let p1 = (x, y - 1);
                    let p2 = (x + 1, y);
                    let d1 = (-1, 0);
                    let d2 = (0, 1);
                    let e = (x, y);

                    zip(&mut warps, &map, p1, d1, p2, d2, e);
                }
                if map[y - 1][x] != ' ' && map[y][x - 1] != ' ' {
                    let p1 = (x, y - 1);
                    let p2 = (x - 1, y);
                    let d1 = (1, 0);
                    let d2 = (0, 1);
                    let e = (x, y);

                    zip(&mut warps, &map, p1, d1, p2, d2, e);
                }
            }
        }
    }

    for (k, v) in warps.iter() {
        println!("{k:?} {v:?}");
    }
    print(0, pos, &map, &warps);
    /*
    return;

    for (y, side) in sides.iter().enumerate() {
        for (x, side) in side.iter().enumerate() {
            if x >= 7 && x <= 10 && y >= 5 && y <= 8 {
                println!("{x},{y}  {side:?}");
            }
        }
    }

    let inst = &lines[0];
    let mut dir = 0i8;

    let mut movement = 0u32;
    for c in inst.chars() {
        println!("c:{c}");
        match c {
            'L' => {
                pos = do_move(pos, dir, movement, &mut map, &sides);
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
                pos = do_move(pos, dir, movement, &mut map, &sides);
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
    pos = do_move(pos, dir, movement, &mut map, &sides);

    let ans = (pos.1 * 1000) + (pos.0 * 4) + dir as usize;
    println!("pos:{pos:?}  dir:{dir}");
    println!("ans: {ans}");
    */
}
