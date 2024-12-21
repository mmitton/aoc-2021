#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Point2D};

struct KeyPad {
    num_levels: usize,
    cache: HashMap<(Point2D<i8>, Point2D<i8>, usize), usize>,
}

impl KeyPad {
    const NUMS: [(Point2D<i8>, char); 11] = [
        (Point2D::new(0, 0), '7'),
        (Point2D::new(1, 0), '8'),
        (Point2D::new(2, 0), '9'),
        (Point2D::new(0, 1), '4'),
        (Point2D::new(1, 1), '5'),
        (Point2D::new(2, 1), '6'),
        (Point2D::new(0, 2), '1'),
        (Point2D::new(1, 2), '2'),
        (Point2D::new(2, 2), '3'),
        (Point2D::new(1, 3), '0'),
        (Point2D::new(2, 3), 'A'),
    ];
    const DIRS: [(Point2D<i8>, char); 5] = [
        (Point2D::new(1, 0), '^'),
        (Point2D::new(2, 0), 'A'),
        (Point2D::new(0, 1), '<'),
        (Point2D::new(1, 1), 'v'),
        (Point2D::new(2, 1), '>'),
    ];

    fn new(num_levels: usize) -> Self {
        Self {
            num_levels,
            cache: HashMap::default(),
        }
    }

    fn cheapest_from_to(
        &mut self,
        from: Point2D<i8>,
        to: Point2D<i8>,
        skip: Point2D<i8>,
        level: usize,
    ) -> usize {
        if let Some(len) = self.cache.get(&(from, to, level)) {
            return *len;
        }

        let mut cheapest = usize::MAX;
        let mut queue = Vec::new();
        queue.push((from, String::new()));
        while let Some((at, path)) = queue.pop() {
            if at == to {
                cheapest = cheapest.min(self.cheapest(&Self::DIRS, &format!("{path}A"), level - 1));
                continue;
            }
            if at == skip {
                continue;
            }

            use std::cmp::Ordering;
            match at.y.cmp(&to.y) {
                Ordering::Less => queue.push((Point2D::new(at.x, at.y + 1), format!("{path}v"))),
                Ordering::Greater => queue.push((Point2D::new(at.x, at.y - 1), format!("{path}^"))),
                Ordering::Equal => {}
            }
            match at.x.cmp(&to.x) {
                Ordering::Less => queue.push((Point2D::new(at.x + 1, at.y), format!("{path}>"))),
                Ordering::Greater => queue.push((Point2D::new(at.x - 1, at.y), format!("{path}<"))),
                Ordering::Equal => {}
            }
        }

        self.cache.insert((from, to, level), cheapest);
        cheapest
    }

    fn cheapest(&mut self, keys: &[(Point2D<i8>, char)], s: &str, level: usize) -> usize {
        if level == 0 {
            return s.len();
        }

        let mut cheapest = 0;

        let mut at = keys
            .iter()
            .find_map(|(k, c)| if *c == 'A' { Some(*k) } else { None })
            .unwrap();
        let skip = Point2D::new(0, at.y);
        for c in s.chars() {
            for k in keys {
                if k.1 == c {
                    cheapest += self.cheapest_from_to(at, k.0, skip, level);
                    at = k.0;
                }
            }
        }

        cheapest
    }

    fn solve(&mut self, s: &str) -> usize {
        self.cheapest(&Self::NUMS, s, self.num_levels)
    }
}

#[derive(Default)]
pub struct Day21 {
    codes: Vec<String>,
}

impl Day21 {
    pub fn new() -> Self {
        Self::default()
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        let mut keypads = KeyPad::new(3);
        Ok(self
            .codes
            .iter()
            .map(|c| keypads.solve(c) * c.strip_suffix('A').unwrap().parse::<usize>().unwrap())
            .sum::<usize>()
            .into())
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        let mut keypads = KeyPad::new(26);
        Ok(self
            .codes
            .iter()
            .map(|c| keypads.solve(c) * c.strip_suffix('A').unwrap().parse::<usize>().unwrap())
            .sum::<usize>()
            .into())
    }
}

impl helper::Runner for Day21 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        self.codes.extend(lines.iter().map(|s| s.into()));
        Ok(())
    }

    fn run_part(&mut self, part: u8) -> Result<helper::RunOutput, Error> {
        match part {
            1 => self.part1(),
            2 => self.part2(),
            _ => Err(Error::Skipped),
        }
    }
}
