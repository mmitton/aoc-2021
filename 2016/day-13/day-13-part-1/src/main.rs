#[cfg(debug_assertions)]
const INPUT: isize = 10;
#[cfg(not(debug_assertions))]
const INPUT: isize = 1358;

#[cfg(debug_assertions)]
const DEST: (isize, isize) = (7, 4);
#[cfg(not(debug_assertions))]
const DEST: (isize, isize) = (31, 39);

fn print(path: &Vec<(isize, isize)>) {
    for y in 0..DEST.1 + 10 {
        for x in 0..DEST.0 + 10 {
            let in_path = path.contains(&(x, y));
            if is_open(x, y) {
                if in_path {
                    print!("O");
                } else {
                    print!(".");
                }
            } else {
                assert!(in_path == false);
                print!("#");
            }
        }
        println!();
    }
}

fn is_open(x: isize, y: isize) -> bool {
    let num = (x * x) + (3 * x) + (2 * x * y) + y + (y * y);
    let num = num + INPUT;

    let bits = num.count_ones();
    bits % 2 == 0
}

fn main() {
    let mut seen = std::collections::BTreeSet::new();
    let mut paths = Vec::new();

    let initial = (1isize, 1isize);
    paths.push(vec![initial]);
    seen.insert(initial);

    let mut i = 0;
    while i < paths.len() {
        let last = paths[i][paths[i].len() - 1];
        let x = last.0;
        let y = last.1;

        for dy in -1..=1 {
            for dx in -1..=1 {
                if (dy == 0 && dx == 0) || (dy != 0 && dx != 0) {
                    continue;
                }
                let x = x + dx;
                let y = y + dy;
                if x < 0 || y < 0 {
                    continue;
                }

                if !is_open(x, y) {
                    continue;
                }

                let next = (x, y);
                if seen.contains(&next) {
                    continue;
                }

                let mut next_path = paths[i].clone();
                next_path.push(next);

                if x == DEST.0 && y == DEST.1 {
                    println!("Found a path with {} steps", next_path.len() - 1);
                    print(&next_path);
                    return;
                }

                paths.push(next_path);
                seen.insert(next);
            }
        }

        i += 1;
    }

    println!("No Path");
}
