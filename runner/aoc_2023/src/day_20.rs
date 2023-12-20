#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};
use std::collections::{BTreeMap, VecDeque};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug)]
enum Module {
    FlipFlop(bool),                      // %
    Conjunction(BTreeMap<usize, Pulse>), // &
    Broadcaster,
    Output,
}

pub struct Day20 {
    modules: Vec<Module>,
    connections: Vec<Vec<usize>>,
    broadcaster: usize,
    rx: usize,
    went_high: BTreeMap<usize, bool>,
}

impl Day20 {
    pub fn new() -> Self {
        Self {
            modules: Vec::new(),
            connections: Vec::new(),
            broadcaster: usize::MAX,
            rx: usize::MAX,
            went_high: BTreeMap::new(),
        }
    }

    fn push_button(&mut self) -> [usize; 2] {
        let mut pulses = VecDeque::new();
        let mut total_pulses = [0; 2];
        pulses.push_front((self.broadcaster, Pulse::Low, 0));
        while let Some((module_idx, pulse, from)) = pulses.pop_front() {
            if pulse == Pulse::High {
                if let Some(v) = self.went_high.get_mut(&from) {
                    *v = true;
                }
            }
            total_pulses[pulse as usize] += 1;
            let module = &mut self.modules[module_idx];
            match module {
                Module::Broadcaster => {
                    for &to_idx in self.connections[module_idx].iter() {
                        pulses.push_back((to_idx, pulse, module_idx));
                    }
                }
                Module::FlipFlop(state) => {
                    if pulse == Pulse::Low {
                        let pulse = if !*state { Pulse::High } else { Pulse::Low };
                        *state = !*state;
                        for &to_idx in self.connections[module_idx].iter() {
                            pulses.push_back((to_idx, pulse, module_idx));
                        }
                    }
                }
                Module::Conjunction(memory) => {
                    memory.insert(from, pulse);
                    let pulse =
                        if pulse == Pulse::High && !memory.values().any(|p| *p == Pulse::Low) {
                            Pulse::Low
                        } else {
                            Pulse::High
                        };
                    for &to_idx in self.connections[module_idx].iter() {
                        pulses.push_back((to_idx, pulse, module_idx));
                    }
                }
                Module::Output => {}
            }
        }

        total_pulses
    }
}

impl Runner for Day20 {
    fn parse(&mut self, path: &str, _part1: bool) -> Result<(), Error> {
        let mut names: Vec<String> = Vec::new();
        let mut connections: Vec<Vec<String>> = Vec::new();
        for line in Lines::from_path(path, LinesOpt::RAW)?.iter() {
            let (first, second) = line.split_once(" -> ").unwrap();
            // println!("{first} .. {second}");
            connections.push(second.split(", ").map(|s| s.to_string()).collect());
            match &line[..1] {
                "%" => {
                    names.push(first[1..].into());
                    self.modules.push(Module::FlipFlop(false));
                }
                "&" => {
                    names.push(first[1..].into());
                    self.modules.push(Module::Conjunction(BTreeMap::new()));
                }
                _ => {
                    assert_eq!(self.broadcaster, usize::MAX);
                    names.push(first.into());
                    self.broadcaster = self.modules.len();
                    self.modules.push(Module::Broadcaster);
                }
            }
        }

        let mut idx = 0;
        while idx < connections.len() {
            let connection_names = connections[idx].clone();
            idx += 1;

            let mut module_connections = Vec::new();
            for name in connection_names.iter() {
                let idx = if let Some(pos) = names.iter().position(|n| n == name) {
                    pos
                } else {
                    self.modules.push(Module::Output);
                    names.push(name.clone());
                    connections.push(Vec::new());
                    names.len() - 1
                };
                module_connections.push(idx);
            }
            self.connections.push(module_connections);
        }

        for (module_idx, module) in self.modules.iter_mut().enumerate() {
            if let Module::Conjunction(memory) = module {
                for (idx, _) in self
                    .connections
                    .iter()
                    .enumerate()
                    .filter(|(_, connections)| connections.contains(&module_idx))
                {
                    memory.insert(idx, Pulse::Low);
                }
            }
        }

        if let Some(idx) = names.iter().position(|n| n == "rx") {
            self.rx = idx;
        }

        assert_ne!(self.broadcaster, usize::MAX);
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        let mut total_pulses = [0; 2];
        for _ in 0..1000 {
            let tp = self.push_button();
            total_pulses[0] += tp[0];
            total_pulses[1] += tp[1];
        }

        println!("total_pulses: {total_pulses:?}");
        Ok((total_pulses[0] * total_pulses[1]).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let pre_rx = if let Some((module_idx, module)) = self
            .connections
            .iter()
            .enumerate()
            .find(|(_, c)| c.contains(&self.rx))
        {
            println!("{module_idx} module: {module:?}");
            module_idx
        } else {
            return Err(Error::Skipped);
        };

        let mut memory = if let Module::Conjunction(memory) = &self.modules[pre_rx] {
            let mut ret = BTreeMap::new();
            for from in memory.keys() {
                ret.insert(*from, 0);
                self.went_high.insert(*from, false);
            }
            ret
        } else {
            unreachable!();
        };

        println!("{memory:?}");
        let mut matched = 0;
        println!("rx: {}  pre_rx: {}", self.rx, pre_rx);
        for button_pushes in 1.. {
            self.push_button();

            for (k, v) in memory.iter_mut().filter(|(_, v)| **v == 0) {
                if self.went_high[k] {
                    println!("{k} {button_pushes}");
                    *v = button_pushes;
                    matched += 1;
                }
            }
            if matched == memory.len() {
                break;
            }
        }

        println!("{memory:?}");
        Ok(memory
            .values()
            .fold(1, |acc, n| helper::lcm(acc, *n))
            .into())
    }
}
