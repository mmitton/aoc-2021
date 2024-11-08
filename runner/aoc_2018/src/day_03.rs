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

#[derive(Default)]
struct Map {
    overlapping: TileSet<u16>,
    non_overlapped: Vec<usize>,
}

impl Day03 {
    pub fn new() -> Self {
        Self::default()
    }

    fn map_claims(&self) -> Map {
        let mut mapped = Map::default();
        let mut non_overlapped: HashSet<usize> = HashSet::default();
        for claim in self.claims.iter() {
            non_overlapped.insert(claim.num);
        }

        for (i, claim_0) in self.claims.iter().enumerate() {
            for claim_1 in self.claims.iter().skip(i + 1) {
                if let Some(overlap) = claim_0.tile.overlaps(&claim_1.tile) {
                    mapped.overlapping.add_tile(overlap);
                    non_overlapped.remove(&claim_0.num);
                    non_overlapped.remove(&claim_1.num);
                }
            }
        }

        mapped.non_overlapped.extend(non_overlapped.drain());
        mapped
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
        let mapped_claims = self.map_claims();
        Ok(mapped_claims
            .overlapping
            .into_iter()
            .map(|t| t.area() as usize)
            .sum::<usize>()
            .into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mapped_claims = self.map_claims();
        if mapped_claims.non_overlapped.len() == 1 {
            Ok(mapped_claims.non_overlapped[0].into())
        } else {
            Err(Error::Unsolved)
        }
    }
}
