use std::collections::BTreeSet;

#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};

pub struct Day17 {
    map: Vec<Vec<(usize, usize)>>,
}

#[derive(Debug, Copy, Clone)]
struct Edge {
    to: usize,
    heat_loss: usize,
    vert: bool,
}

#[derive(Debug)]
struct Node {
    min_cost_vert: usize,
    min_cost_horz: usize,
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
                let mut node = Node {
                    min_cost_vert: usize::MAX,
                    min_cost_horz: usize::MAX,
                    outbound: Vec::new(),
                };

                macro_rules! build_outbound {
                    ($dx:literal, $dy:literal) => {{
                        let mut heat_loss = 0;
                        let vert = $dx == 0;
                        let mut x = nx as isize;
                        let mut y = ny as isize;
                        for i in 1..=max {
                            x += $dx;
                            y += $dy;
                            if x >= 0 && x <= fx as isize && y >= 0 && y <= fy as isize {
                                let x = x as usize;
                                let y = y as usize;
                                let to = y * self.map[0].len() + x;
                                heat_loss += self.map[y][x].0;
                                if i >= min {
                                    node.outbound.push(Edge {
                                        to,
                                        heat_loss,
                                        vert,
                                    });
                                }
                            }
                        }
                    }};
                }
                build_outbound!(1, 0);
                build_outbound!(-1, 0);
                build_outbound!(0, 1);
                build_outbound!(0, -1);

                nodes.push(node);
            }
        }

        nodes[0].min_cost_vert = 0;
        nodes[0].min_cost_horz = 0;
        let mut costs = BTreeSet::new();
        for i in 0..nodes[0].outbound.len() {
            let edge = nodes[0].outbound[i];
            if edge.vert {
                nodes[edge.to].min_cost_vert = edge.heat_loss;
            } else {
                nodes[edge.to].min_cost_horz = edge.heat_loss;
            }
            costs.insert((edge.heat_loss, edge.vert));
        }

        let fto = nodes.len() - 1;

        'search_loop: while let Some((cost, vert)) = costs.pop_first() {
            for i in 0..nodes.len() {
                if vert {
                    // Next is horz
                    if nodes[i].min_cost_vert == cost {
                        if i == fto {
                            break 'search_loop;
                        }
                        for j in 0..nodes[i].outbound.len() {
                            if !nodes[i].outbound[j].vert {
                                let edge = nodes[i].outbound[j];
                                let new_cost = cost + edge.heat_loss;
                                if nodes[edge.to].min_cost_horz > new_cost {
                                    nodes[edge.to].min_cost_horz = new_cost;
                                    costs.insert((new_cost, false));
                                }
                            }
                        }
                    }
                } else {
                    // Next is vert
                    if nodes[i].min_cost_horz == cost {
                        if i == fto {
                            break 'search_loop;
                        }
                        for j in 0..nodes[i].outbound.len() {
                            if nodes[i].outbound[j].vert {
                                let edge = nodes[i].outbound[j];
                                let new_cost = cost + edge.heat_loss;
                                if nodes[edge.to].min_cost_vert > new_cost {
                                    nodes[edge.to].min_cost_vert = new_cost;
                                    costs.insert((new_cost, true));
                                }
                            }
                        }
                    }
                }
            }
        }

        let min_cost_vert = nodes[nodes.len() - 1].min_cost_vert;
        let min_cost_horz = nodes[nodes.len() - 1].min_cost_horz;
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
