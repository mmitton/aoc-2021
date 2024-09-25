#[allow(unused_imports)]
use helper::{
    modexp, modinverse, print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner,
};

pub struct Day22 {
    inst: Vec<Shuffle>,
}

impl Day22 {
    pub fn new() -> Self {
        Self { inst: Vec::new() }
    }
}

type Int = i128;

enum Shuffle {
    NewStack,
    Increment(Int),
    Cut(Int),
}

impl Day22 {
    fn shuffle(&self, total: Int, card: Int) -> Int {
        self.inst.iter().fold(card, |card, inst| match inst {
            Shuffle::NewStack => total - card - 1,
            Shuffle::Increment(i) => (card * i) % total,
            Shuffle::Cut(c) => {
                let c = if *c < 0 { c.abs() } else { total - c.abs() };
                (card + c) % total
            }
        })
    }

    fn big_shuffle(&self, total: Int, card: Int, times: Int) -> Int {
        // Convert the whole process to a linear equation: ax + b
        let (increment_mul, offset_diff) =
            self.inst
                .iter()
                .fold((1, 0), |(increment_mul, offset_diff), inst| {
                    let (a_new, b_new) = match inst {
                        Shuffle::NewStack => (-increment_mul, offset_diff - increment_mul),
                        Shuffle::Cut(n) => (increment_mul, offset_diff + increment_mul * n),
                        Shuffle::Increment(n) => {
                            let n = modexp(*n, total - 2, total);
                            (increment_mul * n, offset_diff)
                        }
                    };
                    (a_new.rem_euclid(total), b_new.rem_euclid(total))
                });
        let increment = modexp(increment_mul, times, total);
        let offset = (offset_diff * (1_i128 - increment)).rem_euclid(total)
            * modexp(1 - increment_mul, total - 2, total);

        (offset + increment * card).rem_euclid(total)
    }
}

impl Runner for Day22 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            let mut parts: Vec<&str> = line.split_whitespace().collect();
            let num = parts.pop().unwrap();
            match parts.pop().unwrap() {
                "new" => self.inst.push(Shuffle::NewStack),
                "increment" => self.inst.push(Shuffle::Increment(num.parse()?)),
                "cut" => self.inst.push(Shuffle::Cut(num.parse()?)),
                _ => unreachable!(),
            }
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.shuffle(10007, 2019).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self
            .big_shuffle(119315717514047, 2020, 101741582076661)
            .into())
        // Ok(self
        //     .big_shuffle(119315717514047, 2020, 101741582076661)
        //     .into())
    }
}
