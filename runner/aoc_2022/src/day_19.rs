#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};
use std::collections::VecDeque;

struct State {
    inventory: [u16; 4],
    bots: [u16; 4],
    elapsed: u16,
}

pub struct Day19 {
    blueprints: Vec<[[u16; 4]; 4]>,
}

impl Day19 {
    pub fn new() -> Self {
        Self {
            blueprints: Vec::new(),
        }
    }

    fn run_blueprint(&self, blueprint: &[[u16; 4]; 4], minutes: u16) -> usize {
        let mut max_robots = [u16::MAX; 4];
        for i in 0..3 {
            max_robots[i] = blueprint.iter().map(|cost| cost[i]).max().unwrap();
        }
        let mut max_geodes = 0;

        let mut q = VecDeque::with_capacity(1 << 12);
        q.push_back(State {
            inventory: [0, 0, 0, 0],
            bots: [1, 0, 0, 0],
            elapsed: 0,
        });

        while let Some(State {
            inventory,
            bots,
            elapsed,
        }) = q.pop_front()
        {
            for i in 0..blueprint.len() {
                if bots[i] == max_robots[i] {
                    continue;
                }

                let costs = &blueprint[i];

                // Find the limiting resource type for the costs.
                let wait_time = (0..3)
                    .map(|idx| match costs[idx] {
                        cost if cost <= inventory[idx] => 0,
                        _ if bots[idx] == 0 => minutes + 1,
                        _ => (costs[idx] - inventory[idx] + bots[idx] - 1) / bots[idx],
                    })
                    .max()
                    .unwrap();

                let new_elapsed = elapsed + wait_time + 1;
                if new_elapsed >= minutes {
                    continue;
                }

                let mut new_inventory = [0; 4];
                for idx in 0..bots.len() {
                    new_inventory[idx] = inventory[idx] + bots[idx] * (wait_time + 1) - costs[idx];
                }

                let mut new_bots = bots;
                new_bots[i] += 1;

                let remaining_time = minutes - new_elapsed;
                if ((remaining_time - 1) * remaining_time) / 2
                    + new_inventory[3]
                    + remaining_time * new_bots[3]
                    < max_geodes
                {
                    continue;
                }

                q.push_back(State {
                    inventory: new_inventory,
                    bots: new_bots,
                    elapsed: new_elapsed,
                })
            }

            let geodes = inventory[3] + bots[3] * (minutes - elapsed);
            max_geodes = geodes.max(max_geodes);
        }

        max_geodes as usize
    }
}

impl Runner for Day19 {
    fn parse(&mut self, path: &str, _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_path(path, LinesOpt::RAW)?;
        for line in lines.iter() {
            let line = line.strip_suffix('.').unwrap();
            let line = line.replace(':', ".");
            let groups: Vec<&str> = line.split(". ").collect();

            let mut blueprint: [[u16; 4]; 4] = [[0; 4]; 4];

            for group in &groups[1..] {
                let group = group.replace(" and ", " ");
                let group: Vec<&str> = group.split(' ').collect();

                let robot = match group[1] {
                    "ore" => 0,
                    "clay" => 1,
                    "obsidian" => 2,
                    "geode" => 3,
                    _ => unreachable!(),
                };

                for req in group[4..].chunks(2) {
                    let material = match req[1] {
                        "ore" => 0,
                        "clay" => 1,
                        "obsidian" => 2,
                        "geode" => 3,
                        _ => unreachable!(),
                    };
                    blueprint[robot][material] = req[0].parse().unwrap();
                }
            }

            self.blueprints.push(blueprint);
        }

        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self
            .blueprints
            .iter()
            .enumerate()
            .map(|(idx, blueprint)| self.run_blueprint(blueprint, 24) * (idx + 1))
            .sum::<usize>()
            .into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self
            .blueprints
            .iter()
            .take(3)
            .map(|blueprint| self.run_blueprint(blueprint, 32))
            .product::<usize>()
            .into())
    }
}
