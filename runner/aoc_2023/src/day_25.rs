#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};
use std::collections::{HashMap, VecDeque};

pub struct Day25 {
    names: Vec<String>,
    connections: Vec<Vec<usize>>,
    connection_use: HashMap<(usize, usize), usize>,
}

impl Day25 {
    pub fn new() -> Self {
        Self {
            names: Vec::new(),
            connections: Vec::new(),
            connection_use: HashMap::new(),
        }
    }

    fn get_paths(&mut self, from: usize) -> Vec<Vec<usize>> {
        let mut paths = Vec::new();
        let mut seen = vec![false; self.names.len()];
        seen[from] = true;
        let mut work = VecDeque::new();
        work.push_front(vec![from]);
        while let Some(path) = work.pop_front() {
            paths.push(path.clone());
            let from = path[path.len() - 1];
            for &connection in self.connections[from].iter() {
                if !seen[connection] {
                    seen[connection] = true;
                    let mut new_path = path.clone();
                    new_path.push(connection);
                    work.push_back(new_path);
                }
            }
        }

        paths
    }
}

#[derive(PartialOrd, Ord, PartialEq, Eq)]
struct ConnectionUse {
    cnt: usize,
    from: usize,
    to: usize,
}

impl Runner for Day25 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let mut names: HashMap<String, usize> = HashMap::new();
        for line in Lines::from_bufread(file, LinesOpt::RAW)?.iter() {
            let (name, connections) = line.split_once(": ").unwrap();
            if !names.contains_key(name) {
                names.insert(name.to_string(), names.len());
                self.names.push(name.to_string());
            }
            for connection in connections.split_whitespace() {
                if !names.contains_key(connection) {
                    names.insert(connection.to_string(), names.len());
                    self.names.push(connection.to_string());
                }
            }
        }
        self.connections = vec![Vec::new(); names.len()];
        for line in Lines::from_bufread(file, LinesOpt::RAW)?.iter() {
            let (name, connections) = line.split_once(": ").unwrap();
            let name_idx = *names.get(name).unwrap();
            for connection in connections.split_whitespace() {
                let connection_idx = *names.get(connection).unwrap();
                self.connections[name_idx].push(connection_idx);
                self.connections[connection_idx].push(name_idx);
            }
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        let step_by = if self.names.len() == 13 {
            1
        } else {
            self.names.len() / 15
        };
        for i in (0..self.names.len()).step_by(step_by) {
            for path in self.get_paths(i) {
                for i in 0..path.len() - 1 {
                    let a = path[i].min(path[i + 1]);
                    let b = path[i].max(path[i + 1]);
                    *self.connection_use.entry((a, b)).or_default() += 1;
                }
            }
        }
        let mut uses: Vec<_> = self
            .connection_use
            .iter()
            .map(|(k, v)| ConnectionUse {
                cnt: *v,
                from: k.0,
                to: k.1,
            })
            .collect();
        uses.sort();
        let remove = &uses[uses.len() - 3..];
        for u in remove.iter() {
            self.connections[u.from].retain(|&to| to != u.to);
            self.connections[u.to].retain(|&from| from != u.from);
        }
        let paths = self.get_paths(0);
        let a = paths.len();
        let b = self.names.len() - a;

        Ok((a * b).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Err(Error::Skipped)
    }
}
