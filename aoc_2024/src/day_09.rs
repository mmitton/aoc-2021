#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt};
use std::collections::VecDeque;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
struct File {
    block: usize,
    len: usize,
    id: usize,
}

impl File {
    fn checksum(&self) -> usize {
        (self.block..self.block + self.len).sum::<usize>() * self.id
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
struct Free {
    block: usize,
    len: usize,
}

#[derive(Default)]
pub struct Day09 {
    files: Vec<File>,
    free: VecDeque<Free>,
}

impl Day09 {
    pub fn new() -> Self {
        Self::default()
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        for i in (0..self.files.len()).rev() {
            'move_file: loop {
                while let Some(free) = self.free.front_mut() {
                    if free.block > self.files[i].block {
                        break 'move_file;
                    }
                    if free.len >= self.files[i].len {
                        // Move the remainder of the file to the free span
                        self.files[i].block = free.block;
                        free.len -= self.files[i].len;
                        free.block += self.files[i].len;
                        if free.len == 0 {
                            self.free.pop_front();
                        }
                        break 'move_file;
                    } else {
                        // Split the file
                        self.files.push(File {
                            block: free.block,
                            len: free.len,
                            id: self.files[i].id,
                        });
                        self.files[i].len -= free.len;
                        self.free.pop_front();
                    }
                }
            }
        }
        Ok(self.files.iter().map(File::checksum).sum::<usize>().into())
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        // Make a set of free lists, one for each length of the free spans
        let mut free_lists: [VecDeque<Free>; 10] = std::array::from_fn(|_| VecDeque::new());
        for free in self.free.iter() {
            free_lists[free.len].push_back(*free);
        }

        for i in (0..self.files.len()).rev() {
            let len = self.files[i].len;
            let block = self.files[i].block;
            // Find the lowest free span that is at least as big as the file and before the file
            if let Some(free) = free_lists
                .iter()
                .skip(len)
                .filter_map(|free_list| {
                    let free = free_list.front()?;
                    if free.block < block {
                        Some(free)
                    } else {
                        None
                    }
                })
                .min()
            {
                let mut free = free_lists[free.len].pop_front().unwrap();
                self.files[i].block = free.block;
                if free.len > len {
                    // Add remaining free span into free list for new free length
                    free.len -= len;
                    free.block += len;
                    match free_lists[free.len].binary_search(&free) {
                        Ok(_) => unreachable!(),
                        Err(idx) => free_lists[free.len].insert(idx, free),
                    }
                }
            }
        }
        Ok(self.files.iter().map(File::checksum).sum::<usize>().into())
    }
}

impl helper::Runner for Day09 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let mut block = 0;
        for (i, c) in Lines::from_bufread(file, LinesOpt::RAW)?
            .single_line()?
            .chars()
            .enumerate()
        {
            let len = (c as u8 - b'0') as usize;
            if len > 0 {
                if i % 2 == 0 {
                    self.files.push(File {
                        block,
                        len,
                        id: i / 2,
                    });
                } else {
                    self.free.push_back(Free { block, len })
                }
            }
            block += len;
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
