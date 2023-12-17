use std::collections::{BTreeMap, BTreeSet};

#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};

pub struct Day17 {
    map: Vec<Vec<(usize, usize)>>,
}

#[derive(Debug, Copy, Clone)]
struct Edge {
    to: usize,
    heat_loss: usize,
}

#[derive(Debug)]
struct Node {
    cost: usize,
    outbound: Vec<Edge>,
}

impl Day17 {
    pub fn new() -> Self {
        Self { map: Vec::new() }
    }

    fn find_path(&mut self, min: usize, max: usize) -> usize {
        let fx = self.map[0].len() - 1;
        let fy = self.map.len() - 1;

        let mut nodes: Vec<Node> = Vec::with_capacity(self.map[0].len() * self.map.len());
        for (ny, row) in self.map.iter().enumerate() {
            for (nx, _) in row.iter().enumerate() {
                let mut node = vec![
                    Node {
                        cost: usize::MAX,
                        outbound: Vec::new(),
                    },
                    Node {
                        cost: usize::MAX,
                        outbound: Vec::new(),
                    },
                ];

                macro_rules! build_outbound {
                    ($dx:literal, $dy:literal) => {{
                        let mut heat_loss = 0;
                        let vert = if $dx == 0 { 0 } else { 1 };
                        let mut x = nx as isize;
                        let mut y = ny as isize;
                        for i in 1..=max {
                            x += $dx;
                            y += $dy;
                            if x >= 0 && x <= fx as isize && y >= 0 && y <= fy as isize {
                                let x = x as usize;
                                let y = y as usize;
                                let to = (y * self.map[0].len() + x) * 2 + 1 - vert;
                                heat_loss += self.map[y][x].0;
                                if i >= min {
                                    node[vert].outbound.push(Edge { to, heat_loss });
                                }
                            }
                        }
                    }};
                }
                if fx != nx || fy != ny {
                    // Don't build outbound edges for the final tile
                    build_outbound!(1, 0);
                    build_outbound!(-1, 0);
                    build_outbound!(0, 1);
                    build_outbound!(0, -1);
                }

                nodes.append(&mut node);
            }
        }

        nodes[0].cost = 0;
        nodes[1].cost = 0;
        let mut costs: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
        for i in 0..2 {
            for j in 0..nodes[i].outbound.len() {
                let edge = nodes[i].outbound[j];
                nodes[edge.to].cost = edge.heat_loss;
                costs.entry(edge.heat_loss).or_default().insert(edge.to);
            }
        }

        while let Some((cost, canidates)) = costs.pop_first() {
            for canidate in canidates {
                if nodes[canidate].cost == cost {
                    for j in 0..nodes[canidate].outbound.len() {
                        let edge = nodes[canidate].outbound[j];
                        let new_cost = cost + edge.heat_loss;
                        if nodes[edge.to].cost > new_cost {
                            nodes[edge.to].cost = new_cost;
                            costs.entry(new_cost).or_default().insert(edge.to);
                        }
                    }
                }
            }
        }

        let min_cost_vert = nodes[nodes.len() - 2].cost;
        let min_cost_horz = nodes[nodes.len() - 1].cost;
        min_cost_horz.min(min_cost_vert)
    }
}

impl Runner for Day17 {
    fn parse(&mut self, path: &str, _part1: bool) -> Result<(), Error> {
        for line in Lines::from_path(path, LinesOpt::RAW)?.iter() {
            self.map.push(
                line.chars()
                    .map(|c| (c as usize - '0' as usize, usize::MAX))
                    .collect(),
            );
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        let heat_loss = self.find_path(1, 3);
        Ok(heat_loss.into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let heat_loss = self.find_path(4, 10);
        Ok(heat_loss.into())
    }
}
