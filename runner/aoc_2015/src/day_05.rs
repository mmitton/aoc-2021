#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Default)]
pub struct Day05 {
    words: Vec<String>,
}

impl Day05 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day05 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::ALL)?;
        self.words.extend(lines.iter().map(|s| s.into()));
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        fn is_nice(s: &str) -> bool {
            for bad in &["ab", "cd", "pq", "xy"] {
                if s.contains(bad) {
                    return false;
                }
            }

            let chars: Vec<char> = s.chars().collect();
            let mut vowels = 0usize;
            let mut last_c = 0 as char;
            let mut has_double = false;
            for c in &chars {
                if last_c == *c {
                    has_double = true;
                }
                match c {
                    'a' | 'e' | 'i' | 'o' | 'u' => vowels += 1,
                    _ => {}
                }

                last_c = *c;
            }

            vowels >= 3 && has_double
        }

        Ok(self.words.iter().filter(|s| is_nice(s)).count().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        fn is_nice(s: &str) -> bool {
            let c: Vec<char> = s.chars().collect();

            let mut found_part_1 = false;
            'part_1: for i in 0..c.len() - 3 {
                for j in i + 2..c.len() - 1 {
                    if c[i] == c[j] && c[i + 1] == c[j + 1] {
                        found_part_1 = true;
                        break 'part_1;
                    }
                }
            }
            if !found_part_1 {
                return false;
            }

            let mut found_part_2 = false;
            for i in 0..c.len() - 2 {
                if c[i] == c[i + 2] {
                    found_part_2 = true;
                }
            }
            if !found_part_2 {
                return false;
            }

            true
        }

        Ok(self.words.iter().filter(|s| is_nice(s)).count().into())
    }
}
