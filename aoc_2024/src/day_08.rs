use helper::Point2D;
#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt};

#[derive(Default)]
pub struct Day08 {
    points: HashMap<char, Vec<Point2D<i16>>>,
    bounds: Point2D<i16>,
}

impl Day08 {
    pub fn new() -> Self {
        Self::default()
    }

    fn _print(&self) {
        for y in 0..=self.bounds.y {
            for x in 0..=self.bounds.x {
                let point = Point2D::new(x, y);
                let at: Vec<char> = self
                    .points
                    .iter()
                    .filter_map(|(c, points)| {
                        if points.contains(&point) {
                            Some(*c)
                        } else {
                            None
                        }
                    })
                    .collect();
                match at.len() {
                    0 => print!("."),
                    1 => {
                        assert_eq!(at[0], '#');
                        print!("{}", at[0]);
                    }
                    2 => print!("{}", at.iter().find(|c| **c != '#').unwrap()),
                    _ => unreachable!(),
                }
            }
            println!();
        }
    }

    fn generate_antinodes(&mut self, harmonics: bool) {
        let mut antinodes = HashSet::default();
        macro_rules! make_antinode {
            ($a:ident, $b:ident) => {{
                let dx = $a.x - $b.x;
                let dy = $a.y - $b.y;
                let mut antinode: Point2D<i16> = Point2D::new($a.x + dx, $a.y + dy);
                if harmonics {
                    antinodes.insert(*$a);
                }
                while (0..=self.bounds.x).contains(&antinode.x)
                    && (0..=self.bounds.y).contains(&antinode.y)
                {
                    antinodes.insert(antinode);
                    if !harmonics {
                        break;
                    }
                    antinode.x += dx;
                    antinode.y += dy;
                }
            }};
        }
        for antennas in self
            .points
            .iter()
            .filter_map(|(c, antennas)| if *c != '#' { Some(antennas) } else { None })
        {
            for (i, a) in antennas.iter().enumerate() {
                for b in antennas.iter().skip(i + 1) {
                    make_antinode!(a, b);
                    make_antinode!(b, a);
                }
            }
        }
        self.points.insert('#', antinodes.iter().copied().collect());
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        self.generate_antinodes(false);
        // self._print();
        Ok(self.points.get(&'#').unwrap().len().into())
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        self.generate_antinodes(true);
        // self._print();
        Ok(self.points.get(&'#').unwrap().len().into())
    }
}

impl helper::Runner for Day08 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for (y, line) in lines.iter().enumerate() {
            let y = y as i16;
            self.bounds.y = self.bounds.y.max(y);
            for (x, c) in line.chars().enumerate() {
                let x = x as i16;
                self.bounds.x = self.bounds.x.max(x);
                if c != '.' {
                    self.points.entry(c).or_default().push(Point2D::new(x, y));
                }
            }
        }
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
