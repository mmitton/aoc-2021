use std::ops::Range;

#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Debug)]
pub enum RunnerError {}

impl From<RunnerError> for Error {
    fn from(e: RunnerError) -> Self {
        Self::Runner(format!("{e:?}"))
    }
}

#[derive(Debug)]
struct Map {
    dest: Range<usize>,
    src: Range<usize>,
}

#[derive(Default)]
pub struct Day05 {
    seeds: Vec<usize>,
    maps: Vec<Vec<Map>>,
}

impl Day05 {
    pub fn new() -> Self {
        Self::default()
    }

    fn find_best_location(&self, range: Range<usize>, conversions: &[Vec<Map>]) -> usize {
        if conversions.is_empty() {
            // No more conversions, must be at Location.  Return the lowest in the range
            return range.start;
        }
        let mut best_location = usize::MAX;

        // Map the range using the current conversion (conversions[0]), removing the covered ranges as you
        // go and then identity map the remaining at the end.  When you find a mapped range (or identity
        // map a range), recurse for the remaining conversions.

        let mut remaining = vec![range];
        macro_rules! new_work {
            ($typ:expr, $dest:expr) => {{
                let location = self.find_best_location($dest, &conversions[1..]);
                if location < best_location {
                    best_location = location;
                }
            }};
        }
        for map in conversions[0].iter() {
            let mut i = 0;
            while i < remaining.len() {
                let cur_remaining = &mut remaining[i];
                let start = map.src.start.max(cur_remaining.start);
                let end = map.src.end.min(cur_remaining.end);

                // If end > start then we found an overlap!
                if end > start {
                    let offset = start - map.src.start;
                    let dest = map.dest.start + offset..map.dest.start + offset + end - start;
                    new_work!(*to, dest.clone());

                    // Remove overlapping range from remaining
                    if *cur_remaining == (start..end) {
                        // Whole cur_remaining is consumed
                        remaining.remove(i);
                    } else if cur_remaining.start < start && cur_remaining.end > end {
                        // Remove gap in the middle of cur_remaining
                        let extra_remaining = end..cur_remaining.end;
                        cur_remaining.end = start;
                        remaining.push(extra_remaining);
                    } else if cur_remaining.start < start {
                        // Remove the end of cur_remaining
                        cur_remaining.end = start;
                    } else if cur_remaining.end > end {
                        // Remove the beginning of cur_remaining
                        cur_remaining.start = end;
                    } else {
                        // Should not ever get here
                        unreachable!();
                    }

                    // Continue to reprocess the cur_remaining (in case there is another overlap
                    // somewhere)
                    continue;
                }
                i += 1;
            }
        }

        // Identity map any remaining ranges
        for remaining in remaining {
            new_work!(*to, remaining);
        }

        best_location
    }
}

impl Runner for Day05 {
    fn parse(&mut self, path: &str) -> Result<(), Error> {
        let lines = Lines::from_path(path, LinesOpt::TRIM | LinesOpt::REMOVE_EMPTY)?;
        let lines: Vec<&str> = lines.iter().collect();
        self.seeds.extend(
            lines[0]
                .split_whitespace()
                .skip(1)
                .map(|s| s.parse::<usize>().unwrap()),
        );

        let mut map = Vec::new();
        for line in lines.iter().skip(1) {
            if line.ends_with(" map:") {
                if !map.is_empty() {
                    self.maps.push(map);
                    map = Vec::new();
                }
            } else {
                let parts = line.split_whitespace().collect::<Vec<&str>>();
                assert_eq!(parts.len(), 3);
                let dest: usize = parts[0].parse().unwrap();
                let src: usize = parts[1].parse().unwrap();
                let len: usize = parts[2].parse().unwrap();
                let dest = dest..dest + len;
                let src = src..src + len;
                map.push(Map { dest, src });
            }
        }
        if !map.is_empty() {
            self.maps.push(map);
        }

        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        let mut best_location = usize::MAX;
        for seed in self.seeds.iter() {
            let location = self.find_best_location(*seed..*seed + 1, &self.maps);
            println!("{seed} => Location {location}");
            if location < best_location {
                best_location = location;
            }
        }
        Ok(best_location.into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut best_location = usize::MAX;
        for seed in self.seeds.chunks(2) {
            let location = self.find_best_location(seed[0]..seed[0] + seed[1], &self.maps);
            println!("{seed:?} => Location {location}");
            if location < best_location {
                best_location = location;
            }
        }
        Ok(best_location.into())
    }
}
