const INPUT_FILE: &str = if cfg!(debug_assertions) {
    "../input-sample.txt"
} else {
    "../input.txt"
};

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Entry {
    num: isize,
    prev: usize,
    next: usize,
}

fn get_num(nums: &[Entry], start_at: usize, len: usize) -> (isize, usize) {
    let mut idx = start_at;
    let mut len = len;
    while len != 0 {
        len -= 1;
        idx = nums[idx].next;
    }

    (nums[idx].num, idx)
}

fn main() {
    let lines: Vec<String> = {
        let file = File::open(INPUT_FILE).expect("Cannot open input file");
        BufReader::new(file).lines().flatten().collect()
    };

    let mut nums = Vec::new();
    let mut zero_at = 0;
    for line in lines {
        if line.is_empty() {
            continue;
        }

        let num: isize = line.parse().unwrap();
        let num = num * 811589153;
        let prev = if nums.is_empty() { 0 } else { nums.len() - 1 };
        let next = nums.len() + 1;

        if num == 0 {
            zero_at = nums.len();
        }

        nums.push(Entry { num, prev, next });
    }

    let first = 0;
    let last = nums.len() - 1;
    nums[first].prev = last;
    nums[last].next = first;

    for _ in 0..10 {
        for idx in 0..nums.len() {
            // Move nums[idx] around
            let mut to_move = nums[idx].num % (nums.len() - 1) as isize;
            if to_move != 0 {
                let prev = nums[idx].prev;
                let next = nums[idx].next;
                nums[prev].next = next;
                nums[next].prev = prev;

                let mut insert_after = prev;
                while to_move != 0 {
                    if to_move < 0 {
                        insert_after = nums[insert_after].prev;
                        to_move += 1;
                    } else {
                        insert_after = nums[insert_after].next;
                        to_move -= 1;
                    }
                }

                nums[idx].prev = insert_after;
                nums[idx].next = nums[insert_after].next;
                let next = nums[idx].next;
                nums[insert_after].next = idx;
                nums[next].prev = idx;
            }
        }
    }

    let (num1, start_at) = get_num(&nums, zero_at, 1000);
    let (num2, start_at) = get_num(&nums, start_at, 1000);
    let (num3, _) = get_num(&nums, start_at, 1000);

    println!("{num1} {num2} {num3} : {}", num1 + num2 + num3);
}
