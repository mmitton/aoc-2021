#[cfg(debug_assertions)]
const INPUT: isize = 10;
#[cfg(not(debug_assertions))]
const INPUT: isize = 1358;

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

                seen.insert(next);
                if next_path.len() < 51 {
                    paths.push(next_path);
                }
            }
        }

        i += 1;
    }

    println!("Answer: {}", seen.len());
}
