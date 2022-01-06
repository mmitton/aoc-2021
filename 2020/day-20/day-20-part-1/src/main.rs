#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

use std::collections::BTreeMap;

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NAN(std::num::ParseIntError),
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Error {
        Error::NAN(e)
    }
}

struct Tile {
    grid: Vec<Vec<char>>,
    edge_connections: [Option<(usize, usize)>; 4],
}

impl Tile {
    fn edges(&self) -> Vec<Vec<char>> {
        let mut edges = Vec::new();

        edges.push(self.grid[0].clone());
        edges.push(self.grid[self.grid.len() - 1].clone());

        let mut left = Vec::new();
        let mut right = Vec::new();

        for y in 0..self.grid.len() {
            left.push(self.grid[y][0]);
            right.push(self.grid[y][self.grid[y].len() - 1]);
        }

        edges.push(left);
        edges.push(right);

        edges
    }

    fn set_edge(&mut self, edge: usize, connect_to: (usize, usize)) {
        assert!(
            self.edge_connections[edge].is_none()
                || self.edge_connections[edge] == Some(connect_to)
        );

        self.edge_connections[edge] = Some(connect_to);
    }
}

fn load_input(filename: &str) -> Result<BTreeMap<usize, Tile>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();
    let mut tiles = BTreeMap::new();

    let mut tile = Tile {
        grid: Vec::new(),
        edge_connections: [None; 4],
    };
    let mut tile_num: usize = 0;

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim().to_string();
        if line == "" {
            continue;
        }

        if line.starts_with("Tile ") {
            if tile.grid.len() > 0 {
                tiles.insert(tile_num, tile);
                tile = Tile {
                    grid: Vec::new(),
                    edge_connections: [None; 4],
                };
            }
            tile_num = line[5..line.len() - 1].parse()?;
        } else {
            tile.grid.push(line.chars().collect());
        }
    }
    if tile.grid.len() > 0 {
        tiles.insert(tile_num, tile);
    }

    Ok(tiles)
}

fn main() -> Result<(), Error> {
    let mut tiles = load_input(INPUT_FILE)?;

    println!("Number of tiles: {}", tiles.len());

    let mut tile_nums = Vec::new();
    for key in tiles.keys() {
        tile_nums.push(*key);
    }

    for i in 0..tile_nums.len() {
        let tile1 = tile_nums[i];
        for (edge_num1, edge1) in tiles.get(&tile1).unwrap().edges().iter().enumerate() {
            let edge1_rev: Vec<char> = edge1.clone().into_iter().rev().collect();
            for j in 0..tile_nums.len() {
                if i == j {
                    continue;
                }
                let tile2 = tile_nums[j];
                for (edge_num2, edge2) in tiles.get(&tile2).unwrap().edges().iter().enumerate() {
                    if edge2 == edge1 || &edge1_rev == edge2 {
                        tiles
                            .get_mut(&tile1)
                            .unwrap()
                            .set_edge(edge_num1, (tile2, edge_num2));
                        tiles
                            .get_mut(&tile2)
                            .unwrap()
                            .set_edge(edge_num2, (tile1, edge_num1));
                    }
                }
            }
        }
    }

    let mut corners = Vec::new();
    let mut answer = 1;
    for (tile_num, tile) in &tiles {
        let mut count = 0;
        for edge in &tile.edge_connections {
            if edge.is_some() {
                count += 1;
            }
        }
        assert!(count >= 2);
        if count == 2 {
            corners.push(*tile_num);
            answer *= *tile_num;

            println!("tile edges: {:?}", tile.edge_connections);
        }
    }

    println!("Corners: {:?}", corners);
    println!("Answer: {}", answer);

    Ok(())
}
