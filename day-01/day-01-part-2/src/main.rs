use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let lines = {
        let file = File::open("../input.txt").expect("Cannot open input file");
        BufReader::new(file).lines()
    };

    let mut depths = Vec::new();
    for line in lines {
        match line {
            Ok(line) => {
                let depth = line.parse::<usize>().expect("Cannot parse integer");
                depths.push(depth);
            }
            Err(e) => panic!("{}", e),
        }
    }

    let mut cur_sum = depths[0] + depths[1];
    let mut last_sum = !0usize;
    let mut increases = 0usize;
    for i in 2..depths.len() {
        if i != 2 {
            cur_sum -= depths[i - 3];
        }
        cur_sum += depths[i];
        if cur_sum > last_sum {
            increases += 1;
        }
        last_sum = cur_sum;
    }

    println!("# of increases: {}", increases);
}
