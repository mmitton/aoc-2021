const INPUT_FILE: &str = if cfg!(debug_assertions) {
    "../input-sample.txt"
} else {
    "../input.txt"
};

use std::fmt::Write;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Num(isize);

impl Num {
    fn decode(s: &str) -> Self {
        let mut n: isize = 0;
        for c in s.chars() {
            n *= 5;
            match c {
                '0'..='2' => n += (c as u32 - '0' as u32) as isize,
                '-' => n -= 1,
                '=' => n -= 2,
                _ => unreachable!(),
            }
        }

        Self(n)
    }

    fn new(n: isize) -> Self {
        Self(n)
    }

    fn to_snafu(&self) -> String {
        let mut units: Vec<i8> = Vec::new();
        let mut n = self.0;
        while n != 0 {
            let rem = n % 5;
            n /= 5;

            units.push(rem as i8);
        }

        for idx in 0..units.len() {
            while units[idx] > 4 {
                units[idx] -= 5;
                if idx != units.len() - 1 {
                    units[idx + 1] += 1;
                } else {
                    units.push(1);
                }
            }
            if units[idx] > 2 {
                let d = 5 - units[idx];
                units[idx] = 0 - d;
                if idx != units.len() - 1 {
                    units[idx + 1] += 1;
                } else {
                    units.push(1);
                }
            }
        }

        let mut s = String::new();
        for u in units.iter().rev() {
            match u {
                0..=2 => s.write_fmt(format_args!("{u}")).unwrap(),
                -1 => s.write_char('-').unwrap(),
                -2 => s.write_char('=').unwrap(),
                _ => unreachable!(),
            }
        }

        assert!(Self::decode(&s).0 == self.0);
        s
    }
}

fn main() {
    let lines: Vec<String> = {
        let file = File::open(INPUT_FILE).expect("Cannot open input file");
        BufReader::new(file).lines().flatten().collect()
    };

    let mut ans: isize = 0;
    for line in lines {
        let n = Num::decode(line.as_str());
        ans += n.0;
        println!(
            "{line} => {n:?} => {:?} => {:?}",
            n.to_snafu(),
            Num::decode(n.to_snafu().as_str())
        );
    }

    let ans = Num::new(ans);
    println!("ans: {ans:?}");
    println!("ans: {}", ans.to_snafu());
}
