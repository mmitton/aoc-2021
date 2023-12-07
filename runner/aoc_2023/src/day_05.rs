#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Debug)]
pub enum RunnerError {}

impl From<RunnerError> for Error {
    fn from(e: RunnerError) -> Self {
        Self::Runner(format!("{e:?}"))
    }
}

#[derive(Clone, PartialEq, Eq)]
struct MyRange(std::ops::Range<isize>);

impl PartialOrd for MyRange {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MyRange {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.start.cmp(&other.start) {
            std::cmp::Ordering::Equal => self.end.cmp(&other.end),
            x => x,
        }
    }
}

impl std::fmt::Debug for MyRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl MyRange {
    fn add(&mut self, delta: isize) {
        self.start += delta;
        self.end += delta;
    }

    fn overlaps_with(&self, other: &Self) -> Option<Self> {
        let start = self.start.max(other.start);
        let end = self.end.min(other.end);

        // If end > start then we found an overlap!
        if end > start {
            Some(MyRange(start..end))
        } else {
            None
        }
    }

    fn remove_overlap(&mut self, overlap: &Self) -> Option<Self> {
        assert!(self.start <= overlap.start);
        assert!(self.end >= overlap.end);

        let mut tail = None;
        if self.0 == (overlap.start..overlap.end) {
            // Whole cur_remaining is consumed
            self.end = self.start
        } else if self.start < overlap.start && self.end > overlap.end {
            // Remove gap in the middle of cur_remaining
            tail = Some(Self(overlap.end..self.end));
            self.end = overlap.start;
        } else if self.start < overlap.start {
            // Remove the end of cur_remaining
            self.end = overlap.start;
        } else if self.end > overlap.end {
            // Remove the beginning of cur_remaining
            self.start = overlap.end;
        } else {
            // Should not ever get here
            unreachable!();
        }

        tail
    }
}

impl std::ops::Deref for MyRange {
    type Target = std::ops::Range<isize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for MyRange {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Map {
    src: MyRange,
    delta: isize,
}

impl std::ops::Deref for Map {
    type Target = MyRange;

    fn deref(&self) -> &Self::Target {
        &self.src
    }
}

impl std::ops::DerefMut for Map {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.src
    }
}

#[derive(Default)]
pub struct Day05 {
    seeds: Vec<isize>,
    ranges: Vec<Map>,
}

impl Day05 {
    pub fn new() -> Self {
        Self::default()
    }

    fn find_best_location(&self, range: MyRange) -> isize {
        let mut best_location = isize::MAX;
        let mut search = vec![range];

        while let Some(mut search_range) = search.pop() {
            for range in self.ranges.iter() {
                if let Some(overlap) = range.overlaps_with(&search_range) {
                    best_location = best_location.min(overlap.start + range.delta);
                    if let Some(tail) = search_range.remove_overlap(&overlap) {
                        search.push(tail);
                    }
                }
            }

            if search_range.start != search_range.end {
                best_location = best_location.min(search_range.start);
            }
        }

        best_location
    }
}

impl Runner for Day05 {
    fn parse(&mut self, path: &str, _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_path(path, LinesOpt::TRIM | LinesOpt::REMOVE_EMPTY)?;
        let lines: Vec<&str> = lines.iter().collect();
        self.seeds.extend(
            lines[0]
                .split_whitespace()
                .skip(1)
                .map(|s| s.parse::<isize>().unwrap()),
        );

        // While building the ranges, all of the ranges in self.ranges are in destination ranges.
        // This lets you match a current rule with the existing merged ranges.
        //
        // ie.  If you map 50..52 to 30.32 in the first set of rules, you would then be looking for
        // something in the 30..32 range for the second set of rules.  After all of the rule sets
        // are merged in to one set of rules (with all of the accumulated deltas summed up),
        // convert the ranges back in to source ranges.
        let mut next_ranges = Vec::new();
        for line in lines.iter().skip(1) {
            if line.ends_with(" map:") {
                self.ranges.append(&mut next_ranges);
            } else {
                let parts = line.split_whitespace().collect::<Vec<&str>>();
                assert_eq!(parts.len(), 3);
                let dest: isize = parts[0].parse().unwrap();
                let src: isize = parts[1].parse().unwrap();
                let len: isize = parts[2].parse().unwrap();
                let delta = dest - src;
                let src = MyRange(src..src + len);
                let mut cur_ranges = vec![Map { src, delta }];
                let mut ii = 0;
                'search_loop: while ii < cur_ranges.len() {
                    let mut jj = 0;
                    while jj < self.ranges.len() {
                        // Does cur_ranges[ii] overlap with self.ranges[jj] ?
                        if let Some(mut overlap) = cur_ranges[ii].overlaps_with(&self.ranges[jj]) {
                            if let Some(tail) = cur_ranges[ii].remove_overlap(&overlap) {
                                cur_ranges.push(Map { src: tail, delta });
                            }
                            if let Some(tail) = self.ranges[jj].remove_overlap(&overlap) {
                                self.ranges.push(Map {
                                    src: tail,
                                    delta: self.ranges[jj].delta,
                                });
                            }

                            overlap.add(delta);
                            let overlap = Map {
                                src: overlap,
                                delta: delta + self.ranges[jj].delta,
                            };
                            next_ranges.push(overlap);
                            continue 'search_loop;
                        }
                        jj += 1;
                    }
                    ii += 1;
                }
                cur_ranges.retain(|m| m.start != m.end);
                cur_ranges.iter_mut().for_each(|map| map.src.add(map.delta));
                next_ranges.extend(cur_ranges);
            }
        }
        self.ranges.append(&mut next_ranges);
        self.ranges.retain(|m| m.start != m.end);

        // Convert all ranges from dest to source
        self.ranges
            .iter_mut()
            .for_each(|map| map.src.add(-map.delta));
        self.ranges.sort();

        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        let mut best_location = isize::MAX;
        for seed in self.seeds.iter().copied() {
            let location = self.find_best_location(MyRange(seed..seed + 1));
            if location < best_location {
                best_location = location;
            }
            println!("{seed} => {location}");
        }
        Ok(best_location.into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut best_location = isize::MAX;
        for seed in self.seeds.chunks(2) {
            let location = self.find_best_location(MyRange(seed[0]..seed[0] + seed[1]));
            if location < best_location {
                best_location = location;
            }
            println!("{seed:?} => {location}");
        }
        Ok(best_location.into())
    }
}
