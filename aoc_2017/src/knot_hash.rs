use helper::Error;
use std::str::FromStr;

pub(crate) struct KnotHash {
    pub(crate) numbers: Vec<u8>,
    pub(crate) lengths: Vec<usize>,
    pos: usize,
    skip: usize,
}

impl Default for KnotHash {
    fn default() -> Self {
        let mut knot_hash = Self {
            numbers: Vec::new(),
            lengths: Vec::new(),
            pos: 0,
            skip: 0,
        };
        knot_hash.set_numbers_len(255);
        knot_hash
    }
}

impl FromStr for KnotHash {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut knot_hash = KnotHash::default();
        knot_hash.extent_from_str(s);
        Ok(knot_hash)
    }
}

impl KnotHash {
    pub(crate) fn extent_from_str(&mut self, s: &str) {
        for c in s.chars() {
            self.lengths.push(c as u8 as usize);
        }
    }

    pub(crate) fn set_numbers_len(&mut self, len: u8) {
        self.numbers.clear();
        self.numbers.extend(0..=len);
    }

    pub(crate) fn reorder(&mut self, rounds: usize) {
        if rounds == 64 {
            self.lengths.extend_from_slice(&[17, 31, 73, 47, 23]);
        }
        for _ in 0..rounds {
            for len in self.lengths.iter().copied() {
                for i in 0..len / 2 {
                    let idx1 = (self.pos + i) % self.numbers.len();
                    let idx2 = (self.pos + (len - i - 1)) % self.numbers.len();
                    let (a, b) = (self.numbers[idx1], self.numbers[idx2]);
                    self.numbers[idx2] = a;
                    self.numbers[idx1] = b;
                }
                self.pos = (self.pos + len + self.skip) % self.numbers.len();
                self.skip += 1;
            }
        }
    }

    pub(crate) fn hash(&mut self) -> [u8; 16] {
        self.reorder(64);
        std::array::from_fn(|i| {
            let start = i * 16;
            let block = &self.numbers[start..start + 16];
            block.iter().fold(0, |h, v| h ^ v)
        })
    }
}
