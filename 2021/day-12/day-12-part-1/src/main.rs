#![feature(drain_filter)]
#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
}

#[derive(Debug)]
enum CaveType {
    Start,
    End,
    Big,
    Little,
}

#[derive(Debug)]
struct Node {
    name: String,
    cave_type: CaveType,
    edges: Vec<usize>,
}

#[derive(Debug)]
struct Graph {
    nodes: Vec<Node>,
    start: usize,
}

impl Graph {
    fn paths(&self) -> Vec<Vec<usize>> {
        let mut paths = Vec::new();
        paths.push(vec![self.start]);

        let mut cur = 0usize;
        while cur < paths.len() {
            let idx = paths[cur][paths[cur].len() - 1];
            // println!(
            //     "path_walker: path:{:?} last:{:?}",
            //     paths[cur], self.nodes[idx]
            // );

            if let CaveType::End = self.nodes[idx].cave_type {
                cur += 1;
                continue;
            }

            for edge in &self.nodes[idx].edges {
                let ok = match self.nodes[*edge].cave_type {
                    CaveType::Big => true,
                    CaveType::Little => !paths[cur].contains(edge),
                    CaveType::End => true,
                    _ => false,
                };
                // println!(
                //     "  checking to see if we can go to {:?}  ok:{}",
                //     self.nodes[*edge], ok
                // );

                if ok {
                    let mut new_path = paths[cur].clone();
                    new_path.push(*edge);
                    paths.push(new_path);
                }
            }
            cur += 1;
        }
        paths.drain_filter(|p| {
            if let CaveType::End = self.nodes[p[p.len() - 1]].cave_type {
                false
            } else {
                true
            }
        });

        paths
    }
}

fn load_input(filename: &str) -> Result<Graph, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();

    let mut graph = Graph {
        nodes: Vec::new(),
        start: !0,
    };

    fn find_node(graph: &mut Graph, name: &str) -> usize {
        for i in 0..graph.nodes.len() {
            if &graph.nodes[i].name == name {
                return i;
            }
        }

        let cave_type = match name {
            "start" => CaveType::Start,
            "end" => CaveType::End,
            _ => {
                if name.chars().nth(0).unwrap().is_uppercase() {
                    CaveType::Big
                } else {
                    CaveType::Little
                }
            }
        };

        if let CaveType::Start = cave_type {
            graph.start = graph.nodes.len();
        }

        graph.nodes.push(Node {
            name: name.to_string(),
            cave_type: cave_type,
            edges: Vec::new(),
        });
        return graph.nodes.len() - 1;
    }

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }
        if line.chars().nth(0).unwrap() == '#' {
            continue;
        }

        let parts = line.split("-").collect::<Vec<&str>>();

        let a = find_node(&mut graph, parts[0]);
        let b = find_node(&mut graph, parts[1]);

        graph.nodes[a].edges.push(b);
        graph.nodes[b].edges.push(a);
    }

    Ok(graph)
}

fn main() -> Result<(), Error> {
    let graph = load_input(INPUT_FILE)?;

    let paths = graph.paths();
    println!("Number of paths: {}", paths.len());

    Ok(())
}
