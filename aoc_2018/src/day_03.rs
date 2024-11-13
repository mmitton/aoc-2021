#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};
use helper::{Point, Tile, TileSet};
use std::str::FromStr;

struct Claim {
    num: usize,
    tile: Tile<u16>,
}

impl FromStr for Claim {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(s) = s.strip_prefix('#') {
            let (num, rest) = s.split_once(" @ ").unwrap();
            let (xy, wh) = rest.split_once(": ").unwrap();
            let (x, y) = xy.split_once(',').unwrap();
            let (w, h) = wh.split_once('x').unwrap();
            let num = num.parse()?;
            let x: u16 = x.parse()?;
            let y: u16 = y.parse()?;
            let w: u16 = w.parse()?;
            let h: u16 = h.parse()?;
            Ok(Self {
                num,
                tile: Tile::new(Point::new(x, y), Point::new(x + w - 1, y + h - 1)),
            })
        } else {
            Err(Error::InvalidInput(s.into()))
        }
    }
}

#[derive(Default)]
pub struct Day03 {
    claims: Vec<Claim>,
}

impl Day03 {
    pub fn new() -> Self {
        Self::default()
    }

    fn overlapping(&self) -> TileSet<u16> {
        let mut overlapping = TileSet::default();
        for (i, claim_0) in self.claims.iter().enumerate() {
            for claim_1 in self.claims.iter().skip(i + 1) {
                if let Some(overlap) = claim_0.tile.overlaps(&claim_1.tile) {
                    overlapping.add_tile(overlap);
                }
            }
        }
        overlapping
    }

    fn non_overlapping_claim(&self) -> usize {
        let mut non_overlapping: HashSet<usize> = HashSet::default();
        for claim in self.claims.iter() {
            non_overlapping.insert(claim.num);
        }

        for (i, claim_0) in self.claims.iter().enumerate() {
            for claim_1 in self.claims.iter().skip(i + 1) {
                if claim_0.tile.overlaps(&claim_1.tile).is_some() {
                    non_overlapping.remove(&claim_0.num);
                    non_overlapping.remove(&claim_1.num);
                }
            }
        }

        assert_eq!(non_overlapping.len(), 1);
        *non_overlapping.iter().next().unwrap()
    }
}

impl Runner for Day03 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            self.claims.push(line.parse()?);
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self
            .overlapping()
            .iter()
            .map(|t| t.area() as usize)
            .sum::<usize>()
            .into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self.non_overlapping_claim().into())
    }
}
