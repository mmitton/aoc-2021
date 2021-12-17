use std::collections::BTreeMap;

#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NAN(std::num::ParseIntError),
}

#[derive(Debug)]
struct Graph {
    cities: Vec<String>,
    edges: BTreeMap<(String, String), usize>,
}

#[derive(Debug, Clone)]
struct Path {
    distance: usize,
    cities: Vec<String>,
}

fn load_input(filename: &str) -> Result<Graph, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();

    let mut graph = Graph {
        cities: Vec::new(),
        edges: BTreeMap::new(),
    };

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }

        let line = line.replace(" to ", " ");
        let line = line.replace(" = ", " ");
        let parts = line.split(" ").collect::<Vec<&str>>();
        assert!(parts.len() == 3);

        let a = parts[0].to_string();
        let b = parts[1].to_string();
        let distance: usize = parts[2].parse().map_err(|e| Error::NAN(e))?;

        if !graph.cities.contains(&a) {
            graph.cities.push(a.clone());
        }
        if !graph.cities.contains(&b) {
            graph.cities.push(b.clone());
        }
        graph.edges.insert((a.clone(), b.clone()), distance);
        graph.edges.insert((b.clone(), a.clone()), distance);
    }

    Ok(graph)
}

fn find_paths(graph: &Graph, start: usize, end: usize) -> Vec<Path> {
    let mut complete_paths: Vec<Path> = Vec::new();
    let mut paths: Vec<Path> = Vec::new();

    paths.push(Path {
        distance: 0,
        cities: vec![graph.cities[start].clone()],
    });

    let mut i = 0;
    loop {
        if i == paths.len() {
            break;
        }

        for city in &graph.cities {
            if paths[i].cities.contains(city) {
                continue;
            }

            if let Some(distance) = graph.edges.get(&(
                paths[i].cities[paths[i].cities.len() - 1].clone(),
                city.clone(),
            )) {
                let mut new_path = paths[i].clone();
                new_path.cities.push(city.clone());
                new_path.distance += distance;
                if *city == graph.cities[end] {
                    if new_path.cities.len() == graph.cities.len() {
                        complete_paths.push(new_path);
                    }
                } else {
                    paths.push(new_path);
                }
            }
        }

        i += 1
    }

    complete_paths
}

fn main() -> Result<(), Error> {
    let graph = load_input(INPUT_FILE)?;

    let mut longest = 0;
    println!("Paths");
    for a in 0..graph.cities.len() - 1 {
        for b in a + 1..graph.cities.len() {
            let paths = find_paths(&graph, a, b);
            for path in &paths {
                if path.distance > longest {
                    longest = path.distance;
                }
                println!("  {:?}", path);
            }
        }
    }

    println!("Longest: {}", longest);
    Ok(())
}
