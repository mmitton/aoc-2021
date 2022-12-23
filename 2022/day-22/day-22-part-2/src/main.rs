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

fn do_move(
    pos: (usize, usize),
    dir: i8,
    movement: u32,
    map: &mut Vec<Vec<char>>,
    warps: &BTreeMap<(usize, usize), Vec<(usize, usize)>>,
) -> ((usize, usize), i8) {
    // print(dir, (pos.0 as usize, pos.1 as usize), map);
    println!("do_move({pos:?}, {dir}, {movement}, ..)");
    let mut pos = pos;
    let mut dir = dir;
    for _ in 0..movement {
        let mut next_pos = match dir {
            0 => (pos.0 + 1, pos.1 + 0),
            1 => (pos.0 + 0, pos.1 + 1),
            2 => (pos.0 - 1, pos.1 + 0),
            3 => (pos.0 + 0, pos.1 - 1),
            _ => unreachable!(),
        };
        let mut next_dir = dir;

        map[pos.1][pos.0] = match dir {
            0 => '>',
            1 => 'v',
            2 => '<',
            3 => '^',
            _ => unreachable!(),
        };
        // print(dir, (pos.0 as usize, pos.1 as usize), map);

        if let Some(local_warps) = warps.get(&next_pos) {
            println!("moving into {next_pos:?} is a warp to {local_warps:?}");
            next_pos = if local_warps.len() == 1 {
                local_warps[0]
            } else if local_warps.len() == 2 {
                if local_warps[0] == pos {
                    local_warps[1]
                } else {
                    local_warps[0]
                }
            } else {
                unreachable!();
            };

            next_dir = !0;
            macro_rules! find_dir {
                ($x:expr, $y:expr, $dir:expr) => {{
                    if let Some(values) = warps.get(&($x, $y)) {
                        if values.contains(&pos) {
                            next_dir = $dir;
                        }
                    }
                }};
            }

            find_dir!(next_pos.0, next_pos.1 - 1, 1);
            find_dir!(next_pos.0, next_pos.1 + 1, 3);
            find_dir!(next_pos.0 - 1, next_pos.1, 0);
            find_dir!(next_pos.0 + 1, next_pos.1, 2);
            assert!(next_dir != !0);
        }

        if map[next_pos.1][next_pos.0] == '#' {
            break;
        }

        pos = next_pos;
        dir = next_dir;
        map[pos.1][pos.0] = match dir {
            0 => '>',
            1 => 'v',
            2 => '<',
            3 => '^',
            _ => unreachable!(),
        };
    }

    // print(dir, pos, map, warps);
    (pos, dir)
}

#[derive(Copy, Clone, Debug)]
struct Tail {
    p: (usize, usize),
    d: (isize, isize),
    e: (usize, usize),
}

fn zip(
    warps: &mut BTreeMap<(usize, usize), Vec<(usize, usize)>>,
    tails: &mut BTreeMap<(usize, usize), Tail>,
    map: &[Vec<char>],
    mut p1: (usize, usize),
    mut d1: (isize, isize),
    mut e1: (usize, usize),
    mut p2: (usize, usize),
    mut d2: (isize, isize),
    mut e2: (usize, usize),
) {
    let mut t1 = 0;
    let mut t2 = 0;

    'zip_loop: while t1 == 0 || t2 == 0 {
        println!("{p1:?} {d1:?} {e1:?} {t1:?}  {p2:?} {d2:?} {e2:?} {t2:?}");

        if let Some(values) = warps.get_mut(&e1) {
            if values.contains(&p2) {
                break 'zip_loop;
            }
            values.push(p2);
        } else {
            warps.insert(e1, vec![p2]);
        }
        if let Some(values) = warps.get_mut(&e2) {
            if values.contains(&p1) {
                break 'zip_loop;
            }
            values.push(p1);
        } else {
            warps.insert(e2, vec![p1]);
        }

        // print(0, p1, map, warps);

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

    if let std::collections::btree_map::Entry::Vacant(e) = tails.entry(p1) {
        e.insert(Tail {
            p: p1,
            d: d1,
            e: e1,
        });
    } else {
        tails.remove(&p1);
    }
    if let std::collections::btree_map::Entry::Vacant(e) = tails.entry(p2) {
        e.insert(Tail {
            p: p2,
            d: d2,
            e: e2,
        });
    } else {
        tails.remove(&p2);
    }
}

fn zip_tails(
    warps: &mut BTreeMap<(usize, usize), Vec<(usize, usize)>>,
    tails: &mut BTreeMap<(usize, usize), Tail>,
    map: &[Vec<char>],
) {
    fn find_neighbor(
        warps: &BTreeMap<(usize, usize), Vec<(usize, usize)>>,
        p: (usize, usize),
    ) -> Option<(usize, usize)> {
        macro_rules! check {
            ($x:expr, $y:expr) => {
                if let Some(w) = warps.get(&($x, $y)) {
                    assert!(w.len() == 1);
                    return Some(w[0]);
                }
            };
        }

        check!(p.0 - 1, p.1);
        check!(p.0 + 1, p.1);
        check!(p.0, p.1 - 1);
        check!(p.0, p.1 + 1);
        None
    }

    let mut values: Vec<Tail> = Vec::new();
    for tail in tails.values() {
        values.push(*tail);
    }
    for i in 0..values.len() {
        let i_neighbor = find_neighbor(warps, values[i].p);
        if let Some(i_neighbor) = i_neighbor {
            for j in i + 1..values.len() {
                // Check to see if values[i] is next to values[j]
                let j_neighbor = find_neighbor(warps, values[j].p);
                if let Some(j_neighbor) = j_neighbor {
                    if i_neighbor == j_neighbor {
                        println!(
                            "{:?} => {i_neighbor:?}   {:?} => {j_neighbor:?}",
                            values[i].p, values[j].p
                        );

                        let p1 = values[i].p;
                        let d1 = values[i].d;
                        let e1 = values[i].e;
                        let p2 = values[j].p;
                        let d2 = values[j].d;
                        let e2 = values[j].e;
                        zip(warps, tails, map, p1, d1, e1, p2, d2, e2);
                    }
                }
            }
        }
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

    let mut warps: BTreeMap<(usize, usize), Vec<(usize, usize)>> = BTreeMap::new();
    let mut tails: BTreeMap<(usize, usize), Tail> = BTreeMap::new();
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

                    zip(&mut warps, &mut tails, &map, p1, d1, e, p2, d2, e);
                }
                if map[y + 1][x] != ' ' && map[y][x - 1] != ' ' {
                    let p1 = (x, y + 1);
                    let p2 = (x - 1, y);
                    let d1 = (1, 0);
                    let d2 = (0, -1);
                    let e = (x, y);

                    zip(&mut warps, &mut tails, &map, p1, d1, e, p2, d2, e);
                }
                if map[y - 1][x] != ' ' && map[y][x + 1] != ' ' {
                    let p1 = (x, y - 1);
                    let p2 = (x + 1, y);
                    let d1 = (-1, 0);
                    let d2 = (0, 1);
                    let e = (x, y);

                    zip(&mut warps, &mut tails, &map, p1, d1, e, p2, d2, e);
                }
                if map[y - 1][x] != ' ' && map[y][x - 1] != ' ' {
                    let p1 = (x, y - 1);
                    let p2 = (x - 1, y);
                    let d1 = (1, 0);
                    let d2 = (0, 1);
                    let e = (x, y);

                    zip(&mut warps, &mut tails, &map, p1, d1, e, p2, d2, e);
                }
            }
        }
    }

    print(0, pos, &map, &warps);
    for (k, v) in warps.iter() {
        println!("{k:?} {v:?}");
    }
    for tail in tails.values() {
        println!("tail: {tail:?}");
    }

    zip_tails(&mut warps, &mut tails, &map);
    print(0, pos, &map, &warps);

    let inst = &lines[0];
    let mut dir = 0i8;

    let mut movement = 0u32;
    for c in inst.chars() {
        println!("c:{c}");
        match c {
            'L' => {
                (pos, dir) = do_move(pos, dir, movement, &mut map, &warps);
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
                (pos, dir) = do_move(pos, dir, movement, &mut map, &warps);
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
    (pos, dir) = do_move(pos, dir, movement, &mut map, &warps);
    print(dir, pos, &map, &BTreeMap::new());

    let ans = (pos.1 * 1000) + (pos.0 * 4) + dir as usize;
    println!("pos:{pos:?}  dir:{dir}");
    println!("ans: {ans}");
    // 122007 too low
    // 129193 too low
}
