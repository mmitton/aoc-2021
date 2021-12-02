use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let lines = {
        let file = File::open("../input.txt").expect("Cannot open input file");
        BufReader::new(file).lines()
    };

    let mut last_depth = !0usize;
    let mut increases = 0usize;
    for line in lines {
        match line {
            Ok(line) => {
                let depth = line.parse::<usize>().expect("Cannot parse integer");
                if depth > last_depth {
                    increases += 1;
                }
                last_depth = depth;
            }
            Err(e) => panic!("{}", e),
        }
    }

    println!("# of increases: {}", increases);
}
