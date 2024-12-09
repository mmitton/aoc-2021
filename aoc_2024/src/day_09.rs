#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt};
use std::collections::VecDeque;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Block {
    File(u16),
    Free,
}

#[derive(Default)]
struct FS {
    blocks: Vec<Block>,
}

impl FS {
    fn compact_blocks(&mut self) {
        let free_blocks: Vec<usize> = self
            .blocks
            .iter()
            .enumerate()
            .filter_map(|(i, b)| {
                if matches!(b, Block::Free) {
                    Some(i)
                } else {
                    None
                }
            })
            .collect();
        let mut free_blocks = free_blocks.iter();
        let mut next_free = *free_blocks.next().unwrap();
        for i in (0..self.blocks.len()).rev() {
            if next_free >= i {
                break;
            }
            if matches!(self.blocks[i], Block::File(_)) {
                self.blocks.swap(i, next_free);
                if let Some(next) = free_blocks.next() {
                    next_free = *next;
                } else {
                    break;
                }
            }
        }
    }

    fn compact_files(&mut self) {
        fn find_limits(idx: usize, block: &Block, blocks: &[Block]) -> (usize, usize) {
            let (mut start, mut end) = (idx, idx);
            loop {
                match blocks.get(start.wrapping_sub(1)) {
                    Some(b) if b == block => start -= 1,
                    _ => break,
                }
            }
            loop {
                match blocks.get(end + 1) {
                    Some(b) if b == block => end += 1,
                    _ => break,
                }
            }

            (start, end)
        }

        let mut free_list: [VecDeque<usize>; 10] = std::array::from_fn(|_| VecDeque::new());
        let mut i = 0;
        while i < self.blocks.len() {
            let b = &self.blocks[i];
            let (start, end) = find_limits(i, b, &self.blocks);
            let len = end - start + 1;
            if matches!(b, Block::Free) {
                free_list[len].push_back(start);
            }
            i = end + 1;
        }

        let mut i = self.blocks.len() - 1;
        let mut looking_for = self
            .blocks
            .iter()
            .filter_map(|b| {
                if let Block::File(id) = b {
                    Some(*id)
                } else {
                    None
                }
            })
            .max()
            .unwrap();
        while i > 0 {
            if let b @ Block::File(id) = &self.blocks[i] {
                let (b_start, b_end) = find_limits(i, b, &self.blocks);
                if b_start == 0 {
                    break;
                }
                if *id == looking_for {
                    let len = b_end - b_start + 1;
                    looking_for -= 1;

                    if let Some((free_start, free_len)) = free_list
                        .iter()
                        .enumerate()
                        .skip(len)
                        .filter_map(|(len, list)| {
                            let start = *list.front()?;
                            if start < b_start {
                                Some((start, len))
                            } else {
                                None
                            }
                        })
                        .min()
                    {
                        free_list[free_len].pop_front();
                        if free_len != len {
                            match free_list[free_len - len].binary_search(&(free_start + len)) {
                                Ok(_) => unreachable!(),
                                Err(idx) => free_list[free_len - len].insert(idx, free_start + len),
                            }
                        }
                        for idx in 0..len {
                            self.blocks.swap(b_start + idx, free_start + idx);
                        }
                    }
                }
                i = b_start;
            }
            i -= 1;
        }
    }

    fn checksum(&self) -> usize {
        self.blocks.iter().enumerate().fold(0, |checksum, (i, b)| {
            if let Block::File(id) = b {
                checksum + *id as usize * i
            } else {
                checksum
            }
        })
    }
}

impl std::fmt::Display for FS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for b in self.blocks.iter() {
            match b {
                Block::Free => write!(f, ".")?,
                Block::File(id) => write!(f, "{id}")?,
            }
        }
        Ok(())
    }
}

#[derive(Default)]
pub struct Day09 {
    fs: FS,
}

impl Day09 {
    pub fn new() -> Self {
        Self::default()
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        self.fs.compact_blocks();
        Ok(self.fs.checksum().into())
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        self.fs.compact_files();
        Ok(self.fs.checksum().into())
    }
}

impl helper::Runner for Day09 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        for (i, c) in Lines::from_bufread(file, LinesOpt::RAW)?
            .single_line()?
            .chars()
            .enumerate()
        {
            let block = if i % 2 == 0 {
                let id = i / 2;
                assert_ne!(c, '0');
                Block::File(id as u16)
            } else {
                Block::Free
            };
            let len = c as u8 - b'0';
            for _ in 0..len {
                self.fs.blocks.push(block);
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
