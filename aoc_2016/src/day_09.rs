#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Default)]
pub struct Day09 {
    input: Vec<char>,
}

impl Day09 {
    pub fn new() -> Self {
        Self::default()
    }

    fn parse_marker(mut input: &[char]) -> (usize, &[char], &[char]) {
        let mut len: usize = 0;
        loop {
            let c = input[0];
            input = &input[1..];
            if c == 'x' {
                break;
            }
            let v = (c as u8 - b'0') as usize;
            len = len * 10 + v;
        }

        let mut repeat: usize = 0;
        loop {
            let c = input[0];
            input = &input[1..];
            if c == ')' {
                break;
            }
            let v = (c as u8 - b'0') as usize;
            repeat = repeat * 10 + v;
        }

        (repeat, &input[..len], &input[len..])
    }

    fn decompress_v1(&self) -> usize {
        fn decompress(input: &[char]) -> usize {
            let mut total = 0;
            for (i, c) in input.iter().copied().enumerate() {
                if c == '(' {
                    let (repeat, inner, rest) = Day09::parse_marker(&input[i + 1..]);
                    total += repeat * inner.len();
                    total += decompress(rest);
                    return total;
                } else {
                    total += 1;
                }
            }
            total
        }

        decompress(&self.input)
    }

    fn decompress_v2(&self) -> usize {
        fn decompress(input: &[char]) -> usize {
            let mut total = 0;
            for (i, c) in input.iter().copied().enumerate() {
                if c == '(' {
                    let (repeat, inner, rest) = Day09::parse_marker(&input[i + 1..]);
                    let inner_len = decompress(inner);
                    total += repeat * inner_len;
                    total += decompress(rest);
                    return total;
                } else {
                    total += 1;
                }
            }
            total
        }

        decompress(&self.input)
    }
}

impl Runner for Day09 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        assert_eq!(lines.len(), 1);
        self.input.extend(lines[0].chars());
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.decompress_v1().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self.decompress_v2().into())
    }
}
