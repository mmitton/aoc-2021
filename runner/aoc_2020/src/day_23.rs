#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

pub struct Day23 {
    initial: Vec<usize>,
    cups: Vec<usize>,
}

impl Day23 {
    pub fn new() -> Self {
        Self {
            initial: Vec::new(),
            cups: Vec::new(),
        }
    }

    fn _print(&self, cur: usize) {
        let mut c = cur;
        loop {
            print!("{c}");
            c = self.cups[c];
            if c == cur {
                break;
            }
        }
        println!();
    }

    fn cups_order(&self) -> Vec<usize> {
        let mut order = vec![1];
        let mut n = 1;
        for _ in 2..self.cups.len() {
            n = self.cups[n];
            order.push(n)
        }
        order
    }

    fn first_n(&self, count: usize) -> Vec<usize> {
        let mut order = vec![1];
        let mut n = 1;
        for _ in 2..=count {
            n = self.cups[n];
            order.push(n)
        }
        order
    }

    fn play_game(&mut self, iters: usize, len: usize) {
        self.cups.extend(1..=len + 1);
        *self.cups.last_mut().unwrap() = 1;

        let mut last_idx = 1;
        for num in self.initial.iter().copied() {
            self.cups[last_idx] = num;
            last_idx = num;
        }
        if len == self.initial.len() {
            self.cups[last_idx] = self.initial[0];
        } else {
            self.cups[last_idx] = self.initial.len() + 1;
            self.cups[len] = self.initial[0];
        }

        let mut cur = self.initial[0];

        // let mut cur = self.cups[0];
        let mut holding: [usize; 3] = [0; 3];

        for _ in 1..=iters {
            // "copy" 3 holding
            holding[0] = self.cups[cur];
            holding[1] = self.cups[holding[0]];
            holding[2] = self.cups[holding[1]];

            // find destination
            let mut dest = if cur == 1 { len } else { cur - 1 };
            while holding.contains(&dest) {
                if dest <= 1 {
                    dest = len;
                } else {
                    dest -= 1;
                }
            }

            // update
            let tmp = self.cups[dest];
            self.cups[dest] = holding[0];
            self.cups[cur] = self.cups[holding[2]];
            self.cups[holding[2]] = tmp;

            cur = self.cups[cur];
        }
    }
}

impl Runner for Day23 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        assert_eq!(lines.len(), 1);
        self.initial
            .extend(lines[0].chars().map(|c| (c as u8 - b'0') as usize));
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        self.play_game(100, 9);
        // println!("{:?}", self.cups);

        Ok(self
            .cups_order()
            .iter()
            .skip(1)
            .map(|n| (*n as u8 + b'0') as char)
            .collect::<String>()
            .into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        self.play_game(10000000, 1000000);

        Ok(self.first_n(3).iter().skip(1).product::<usize>().into())
    }
}
