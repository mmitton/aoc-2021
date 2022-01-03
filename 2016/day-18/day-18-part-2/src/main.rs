fn main() {
    let rooms = if cfg!(debug_assertions) {
        vec![(3, "..^^."), (10, ".^^.^.^^^^")]
    } else {
        vec![(400000, "^.^^^.^..^....^^....^^^^.^^.^...^^.^.^^.^^.^^..^.^...^.^..^.^^.^..^.....^^^.^.^^^..^^...^^^...^...^.")]
    };

    for room in rooms {
        let mut last_row: Vec<char> = room.1.chars().collect();
        let mut safe = room.1.replace("^", "").len();

        for _ in 1..room.0 {
            let mut row = Vec::new();
            for x in 0..last_row.len() {
                let left = if x != 0 {
                    last_row[x - 1] == '^'
                } else {
                    false
                };
                let center = last_row[x] == '^';
                let right = if x != last_row.len() - 1 {
                    last_row[x + 1] == '^'
                } else {
                    false
                };

                match (left, center, right) {
                    (true, true, false)
                    | (false, true, true)
                    | (true, false, false)
                    | (false, false, true) => row.push('^'),
                    _ => {
                        safe += 1;
                        row.push('.');
                    }
                }
            }

            last_row = row;
        }

        println!("Safe Tiles: {}", safe);
    }
}
