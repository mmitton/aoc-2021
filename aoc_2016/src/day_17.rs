#[allow(unused_imports)]
use helper::{
    print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner, MD5,
};

#[derive(Default)]
pub struct Day17 {
    initial: String,
}

impl Day17 {
    pub fn new() -> Self {
        Self::default()
    }

    fn find_path(&self, shortest: bool) -> Result<String, Error> {
        let mut queue = std::collections::VecDeque::new();
        queue.push_front((0, 0, "".to_string()));
        let mut longest = String::new();
        let mut md5 = self.initial.clone();

        macro_rules! push {
            ($x:expr, $y: expr, $path:expr, $dir:expr) => {{
                let mut new_path = $path.clone();
                new_path.push($dir);
                if $x == 3 && $y == 3 {
                    longest.clear();
                    longest.push_str(new_path.as_str());
                    if shortest {
                        break;
                    }
                } else {
                    queue.push_back(($x, $y, new_path));
                }
            }};
        }

        while let Some(cur) = queue.pop_front() {
            md5.truncate(self.initial.len());
            md5.push_str(cur.2.as_str());

            let digest = MD5::digest(md5.as_bytes());

            // println!("{}{} => {:?}", self.initial, cur.2, md5);

            if digest[0] >> 4 > 10 && cur.1 != 0 {
                push!(cur.0, cur.1 - 1, cur.2, 'U');
            }
            if digest[0] & 0xF > 10 && cur.1 != 3 {
                push!(cur.0, cur.1 + 1, cur.2, 'D');
            }
            if digest[1] >> 4 > 10 && cur.0 != 0 {
                push!(cur.0 - 1, cur.1, cur.2, 'L');
            }
            if digest[1] & 0xF > 10 && cur.0 != 3 {
                push!(cur.0 + 1, cur.1, cur.2, 'R');
            }
        }

        Ok(longest)
    }
}

impl Runner for Day17 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        assert_eq!(lines.len(), 1);
        self.initial.push_str(lines[0].as_str());
        Ok(())
    }

    fn run_part(&mut self, part: u8) -> Result<RunOutput, Error> {
        match part {
            1 => self.part1(),
            2 => self.part2(),
            _ => Err(Error::Skipped),
        }
    }
}

impl Day17 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.find_path(true)?.into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self.find_path(false)?.len().into())
    }
}
