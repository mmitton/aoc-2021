#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Default)]
struct Node {
    children: Vec<Node>,
    meta: Vec<usize>,
}

impl Node {
    fn parse<'a>(&mut self, mut data: &'a [usize]) -> &'a [usize] {
        let num_children = data[0];
        let num_meta = data[1];
        data = &data[2..];
        for _ in 0..num_children {
            let mut child = Node::default();
            data = child.parse(data);
            self.children.push(child);
        }
        self.meta.extend(data.iter().take(num_meta));

        &data[num_meta..]
    }

    fn meta_sum(&self) -> usize {
        let children_sum = self.children.iter().map(|c| c.meta_sum()).sum::<usize>();
        children_sum + self.meta.iter().sum::<usize>()
    }

    fn meta_sum2(&self) -> usize {
        if self.children.is_empty() {
            self.meta.iter().sum::<usize>()
        } else {
            self.meta
                .iter()
                .filter_map(|i| self.children.get(*i - 1).map(|child| child.meta_sum2()))
                .sum::<usize>()
        }
    }
}

#[derive(Default)]
pub struct Day08 {
    root: Node,
}

impl Day08 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day08 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        let data = lines[0]
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect::<Vec<usize>>();
        assert!(self.root.parse(&data).is_empty());
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.root.meta_sum().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self.root.meta_sum2().into())
    }
}
