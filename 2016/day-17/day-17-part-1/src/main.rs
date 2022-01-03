struct Grid {
    initial: String,
}

impl Grid {
    fn new(s: &str) -> Self {
        Self {
            initial: s.to_string(),
        }
    }

    fn find_shortest_path(&self) -> Option<String> {
        let mut input = Vec::new();
        let mut queue = std::collections::VecDeque::new();
        queue.push_front((0, 0, "".to_string()));

        macro_rules! push {
            ($x:expr, $y: expr, $path:expr, $dir:expr) => {{
                let mut new_path = $path.clone();
                new_path.push($dir);
                if $x == 3 && $y == 3 {
                    return Some(new_path);
                }
                queue.push_back(($x, $y, new_path));
            }};
        }

        while queue.len() > 0 {
            let cur = queue.pop_front().unwrap();

            input.clear();
            input.extend_from_slice(&self.initial.as_bytes());
            input.extend_from_slice(cur.2.as_bytes());

            let md5 = md5::compute(&input);

            // println!("{}{} => {:?}", self.initial, cur.2, md5);

            if md5[0] >> 4 > 10 && cur.1 != 0 {
                push!(cur.0, cur.1 - 1, cur.2, 'U');
            }
            if md5[0] & 0xF > 10 && cur.1 != 3 {
                push!(cur.0, cur.1 + 1, cur.2, 'D');
            }
            if md5[1] >> 4 > 10 && cur.0 != 0 {
                push!(cur.0 - 1, cur.1, cur.2, 'L');
            }
            if md5[1] & 0xF > 10 && cur.0 != 3 {
                push!(cur.0 + 1, cur.1, cur.2, 'R');
            }
        }

        None
    }
}

fn main() {
    let hashes = if cfg!(debug_assertions) {
        vec!["hijkl", "ihgpwlah", "kglvqrro", "ulqzkmiv"]
    } else {
        vec!["vwbaicqe"]
    };

    for hash in hashes {
        let grid = Grid::new(hash);
        let shortest_path = grid.find_shortest_path();

        match shortest_path {
            Some(shortest_path) => println!("{} => {}", hash, shortest_path),
            None => println!("{} => No Solution", hash),
        }
    }
}
