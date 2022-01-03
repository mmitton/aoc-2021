fn main() {
    let rooms = if cfg!(debug_assertions) {
        vec![(3, "..^^."), (10, ".^^.^.^^^^")]
    } else {
        vec![(40, "^.^^^.^..^....^^....^^^^.^^.^...^^.^.^^.^^.^^..^.^...^.^..^.^^.^..^.....^^^.^.^^^..^^...^^^...^...^.")]
    };

    for room in rooms {
        let mut rows: Vec<Vec<char>> = Vec::new();
        rows.push(room.1.chars().collect());
        while rows.len() < room.0 {
            let prev = rows.len() - 1;
            let mut row = Vec::new();
            for x in 0..rows[0].len() {
                let left = if x != 0 {
                    rows[prev][x - 1] == '^'
                } else {
                    false
                };
                let center = rows[prev][x] == '^';
                let right = if x != rows[0].len() - 1 {
                    rows[prev][x + 1] == '^'
                } else {
                    false
                };

                match (left, center, right) {
                    (true, true, false)
                    | (false, true, true)
                    | (true, false, false)
                    | (false, false, true) => row.push('^'),
                    _ => row.push('.'),
                }
            }
            rows.push(row);
        }

        let mut safe = 0;
        for row in &rows {
            for tile in row {
                if *tile != '^' {
                    safe += 1;
                }
                print!("{}", tile);
            }
            println!();
        }

        println!("Safe Tiles: {}\n", safe);
    }
}
