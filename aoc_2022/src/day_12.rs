use std::collections::VecDeque;

#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Debug)]
pub enum RunnerError {}

impl From<RunnerError> for Error {
    fn from(e: RunnerError) -> Self {
        Self::Runner(format!("{e:?}"))
    }
}

pub struct Day12 {
    graph: Vec<(i8, Vec<usize>)>,
    start: usize,
    end: usize,
}

impl Day12 {
    pub fn new() -> Self {
        Self {
            graph: Vec::new(),
            start: usize::MAX,
            end: usize::MAX,
        }
    }

    fn walk<F>(&self, f: F) -> usize
    where
        F: Fn(usize) -> bool,
    {
        let mut best: Vec<Option<u32>> = self.graph.iter().map(|_| None).collect();
        best[self.end] = Some(0);
        let mut work: VecDeque<usize> = VecDeque::new();
        work.push_front(self.end);

        while let Some(pos) = work.pop_front() {
            let steps = best[pos].unwrap() + 1;
            for npos in self.graph[pos].1.iter() {
                if best[*npos].is_none() {
                    // Check if at level 0 and at exit condition
                    if self.graph[*npos].0 == 0 && f(*npos) {
                        return steps as usize;
                    }

                    best[*npos] = Some(steps);
                    work.push_back(*npos);
                }
            }
        }
        0
    }
}

impl Runner for Day12 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        let lines: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
        let height = lines.len();
        let width = lines[0].len();
        self.graph
            .extend((0..height * width).map(|_| (i8::MAX, Vec::new())));
        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                let pos = (y * width) + x;
                macro_rules! height_of {
                    ($c:expr) => {
                        match $c {
                            'S' => 0,
                            'E' => 25,
                            c => (c as u8 - b'a') as i8,
                        }
                    };
                }
                let hc = height_of!(*c);
                match c {
                    'S' => self.start = pos,
                    'E' => self.end = pos,
                    _ => {}
                }
                macro_rules! can_walk {
                    ($x:expr, $y:expr) => {{
                        let hn = height_of!(lines[$y][$x]);
                        if hn >= hc - 1 {
                            Some((($y) * width) + ($x))
                        } else {
                            None
                        }
                    }};
                }
                let mut edges = Vec::new();
                if y > 0 {
                    // Look up
                    if let Some(posn) = can_walk!(x, y - 1) {
                        edges.push(posn);
                    }
                }
                if y < height - 1 {
                    // Look down
                    if let Some(posn) = can_walk!(x, y + 1) {
                        edges.push(posn);
                    }
                }
                if x > 0 {
                    // Look left
                    if let Some(posn) = can_walk!(x - 1, y) {
                        edges.push(posn);
                    }
                }
                if x < width - 1 {
                    // Look right
                    if let Some(posn) = can_walk!(x + 1, y) {
                        edges.push(posn);
                    }
                }
                self.graph[pos] = (hc, edges);
            }
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.walk(|pos| pos == self.start).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self.walk(|_| true).into())
    }
}
