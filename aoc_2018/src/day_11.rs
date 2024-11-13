#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Default)]
pub struct Day11 {
    serial: usize,
    summed_area_table: HashMap<(isize, isize), isize>,
}

impl Day11 {
    pub fn new() -> Self {
        Self::default()
    }

    fn get_summed_area(&self, x: isize, y: isize) -> isize {
        self.summed_area_table.get(&(x, y)).copied().unwrap_or(0)
    }

    fn generate_summed_area_table(&mut self) {
        for y in 1..=300 {
            for x in 1..=300 {
                // compute the value of this cell using the specified formula
                let r = x + 10;
                let p = (((r * y + self.serial as isize) * r) / 100) % 10 - 5;
                // store the result in summed-area form
                let summed_area =
                    p + self.get_summed_area(x, y - 1) + self.get_summed_area(x - 1, y)
                        - self.get_summed_area(x - 1, y - 1);
                self.summed_area_table.insert((x, y), summed_area);
            }
        }
    }

    fn region_sum(&self, size: isize, x: isize, y: isize) -> isize {
        let (x0, y0, x1, y1) = (x - 1, y - 1, x + size - 1, y + size - 1);
        self.get_summed_area(x0, y0) + self.get_summed_area(x1, y1)
            - self.get_summed_area(x1, y0)
            - self.get_summed_area(x0, y1)
    }

    fn best(&self, size: isize) -> ((isize, isize), isize) {
        let mut max_power = isize::MIN;
        let mut max = (0, 0);
        for y in 0..300 - size {
            for x in 0..300 - size {
                let r = self.region_sum(size, x, y);
                if r > max_power {
                    max_power = r;
                    max = (x, y);
                }
            }
        }

        (max, max_power)
    }
}

impl Runner for Day11 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        assert_eq!(lines.len(), 1);
        self.serial = lines[0].parse()?;
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        self.generate_summed_area_table();
        let ((x, y), _) = self.best(3);
        Ok(format!("{x},{y}").into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        self.generate_summed_area_table();
        let (((x, y), _), size) =
            (1..=300)
                .map(|size| (self.best(size), size))
                .fold((((0, 0), 0), 0), |best, cur| {
                    if cur.0 .1 > best.0 .1 {
                        cur
                    } else {
                        best
                    }
                });
        Ok(format!("{x},{y},{size}").into())
    }
}
