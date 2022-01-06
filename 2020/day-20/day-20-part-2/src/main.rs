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

#[derive(Clone)]
struct Tile {
    grid: Vec<Vec<char>>,
    edge_connections: [Option<(usize, usize)>; 4],
    is_rotated: bool,
    x: isize,
    y: isize,
}

impl Tile {
    fn print(&self) {
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                print!("{}", self.grid[y][x]);
            }
            println!();
        }
    }

    fn edges(&self) -> Vec<Option<Vec<char>>> {
        let mut edges = Vec::new();

        let mut left = Vec::new();
        let mut right = Vec::new();

        for y in 0..self.grid.len() {
            left.push(self.grid[y][0]);
            right.push(self.grid[y][self.grid[y].len() - 1]);
        }

        edges.push(Some(self.grid[0].clone()));
        edges.push(Some(right));
        edges.push(Some(self.grid[self.grid.len() - 1].clone()));
        edges.push(Some(left));

        for i in 0..4 {
            if self.edge_connections[i].is_some() {
                edges[i] = None;
            }
        }

        edges
    }

    fn set_edge(&mut self, edge: usize, connect_to: (usize, usize)) {
        assert!(
            self.edge_connections[edge].is_none()
                || self.edge_connections[edge] == Some(connect_to)
        );

        self.edge_connections[edge] = Some(connect_to);
    }

    fn rotate(&mut self, count: usize, flip_x: bool, flip_y: bool) {
        assert!(self.is_rotated == false || (count == 0 && !flip_x && !flip_y));

        if self.is_rotated {
            return;
        }

        self.is_rotated = true;
        for _ in 0..count {
            let mut new_grid = Vec::new();
            for y in 0..self.grid.len() {
                let mut row = Vec::new();
                for x in 0..self.grid[y].len() {
                    row.push(self.grid[self.grid.len() - x - 1][y]);
                }
                new_grid.push(row);
            }

            self.grid = new_grid;
        }

        if flip_x {
            let mut new_grid = Vec::new();
            for y in 0..self.grid.len() {
                let mut row = Vec::new();
                for x in 0..self.grid[y].len() {
                    row.push(self.grid[y][self.grid[y].len() - x - 1]);
                }
                new_grid.push(row);
            }

            self.grid = new_grid;
        }

        if flip_y {
            let mut new_grid = Vec::new();
            for y in 0..self.grid.len() {
                let mut row = Vec::new();
                for x in 0..self.grid[y].len() {
                    row.push(self.grid[self.grid.len() - y - 1][x]);
                }
                new_grid.push(row);
            }

            self.grid = new_grid;
        }
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
        is_rotated: false,
        x: 0,
        y: 0,
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
                    is_rotated: false,
                    x: 0,
                    y: 0,
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

    let mut min_x = 0isize;
    let mut max_x = 0isize;
    let mut min_y = 0isize;
    let mut max_y = 0isize;

    let mut queue = vec![0];
    let mut i = 0;
    while i < queue.len() {
        let tile1 = tile_nums[queue[i]];
        for (edge_num1, edge1) in tiles.get(&tile1).unwrap().edges().iter().enumerate() {
            let edge_num2 = (edge_num1 + 2) % 4;
            if let Some(edge1) = edge1 {
                for j in 0..tile_nums.len() {
                    let tile2 = tile_nums[j];
                    if tile2 == tile1 {
                        continue;
                    }

                    let mut rotations = Vec::new();
                    {
                        let tile2 = tiles.get(&tile2).unwrap();
                        if tile2.is_rotated {
                            rotations.push((0, false, false, tile2.clone()));
                        } else {
                            for r in 0..4 {
                                for flip in 0..3 {
                                    let mut tile2 = tile2.clone();
                                    tile2.rotate(r, flip == 1, flip == 2);
                                    rotations.push((r, flip == 1, flip == 2, tile2.clone()));
                                }
                            }
                        }
                    }

                    'rotation_loop: for (rotation, flip_x, flip_y, tile) in &rotations {
                        let edges = tile.edges();
                        if let Some(edge2) = &edges[edge_num2] {
                            if edge2 == edge1 {
                                let (x, y) = {
                                    let tile1 = tiles.get_mut(&tile1).unwrap();

                                    tile1.set_edge(edge_num1, (tile2, edge_num2));
                                    (tile1.x, tile1.y)
                                };

                                let t2 = tiles.get_mut(&tile2).unwrap();
                                t2.rotate(*rotation, *flip_x, *flip_y);
                                t2.x = x;
                                t2.y = y;
                                match edge_num1 {
                                    0 => t2.y -= 1,
                                    1 => t2.x += 1,
                                    2 => t2.y += 1,
                                    3 => t2.x -= 1,
                                    _ => {}
                                }

                                if t2.x < min_x {
                                    min_x = t2.x
                                }
                                if t2.x > max_x {
                                    max_x = t2.x
                                }
                                if t2.y < min_y {
                                    min_y = t2.y
                                }
                                if t2.y > max_y {
                                    max_y = t2.y
                                }

                                println!(
                                    "{}/{} => {}/{}  {},{}  {},{}",
                                    tile1, edge_num1, tile2, edge_num2, x, y, t2.x, t2.y
                                );

                                t2.set_edge(edge_num2, (tile1, edge_num1));

                                if !queue.contains(&j) {
                                    queue.push(j);
                                }

                                break 'rotation_loop;
                            }
                        }
                    }
                }
            }
        }

        i += 1;
    }

    for (tile_num, tile) in tiles.iter_mut() {
        tile.x -= min_x;
        tile.y -= min_y;
        println!("Tile {} => {},{}", tile_num, tile.x, tile.y);
    }

    let max_x = (max_x - min_x) as usize;
    let max_y = (max_y - min_y) as usize;
    let min_x = 0usize;
    let min_y = 0usize;
    println!("{},{} -> {},{}", min_x, min_y, max_x, max_y);

    let mut corners = Vec::new();
    let mut answer = 1;
    let mut upper_left = 0;
    for (tile_num, tile) in tiles.iter_mut() {
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
            println!("Tile Edges: {:?}", tile.edge_connections);
            if tile.edge_connections[1].is_some() && tile.edge_connections[2].is_some() {
                upper_left = *tile_num;
                tile.x = 0;
                tile.y = 0;
            }
        }
    }

    println!("Corners: {:?}", corners);
    println!("Answer: {}", answer);
    println!("Upper Left is: {}", upper_left);

    let size = {
        let tile = tiles.get(&tile_nums[0]).unwrap();
        tile.grid.len()
    } - 2;

    let full_size = (max_y + 1) * size;

    let mut grid = Vec::new();
    for _ in 0..full_size {
        grid.push(vec!['.'; full_size]);
    }

    for (_, tile) in tiles.iter() {
        let sy = tile.y as usize * size;
        let sx = tile.x as usize * size;

        for y in 0..size {
            for x in 0..size {
                grid[sy + y][sx + x] = tile.grid[y + 1][x + 1];
            }
        }
    }

    let tile = Tile {
        grid: grid,
        edge_connections: [None; 4],
        x: 0,
        y: 0,
        is_rotated: false,
    };

    tile.print();

    let mut sea_monster = Vec::new();
    let sea_monster_array = [
        "                  # ",
        "#    ##    ##    ###",
        " #  #  #  #  #  #   ",
    ];
    for (y, line) in sea_monster_array.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                sea_monster.push((x, y));
            }
        }
    }

    println!("Sea Monster: {:?}", sea_monster);

    for rotate in 0..4 {
        for flip in 0..3 {
            let mut tile = tile.clone();
            tile.rotate(rotate, flip == 1, flip == 2);
            println!(
                "\n\nRotation: {}  Flip X: {}  Flip Y: {}",
                rotate,
                flip == 1,
                flip == 2
            );
            tile.print();

            let mut found = false;

            for y in 0..full_size {
                'search_loop: for x in 0..full_size {
                    for coord in &sea_monster {
                        if coord.0 + x >= full_size || coord.1 + y >= full_size {
                            continue 'search_loop;
                        }
                        if tile.grid[coord.1 + y][coord.0 + x] != '#' {
                            continue 'search_loop;
                        }
                    }

                    found = true;
                    for coord in &sea_monster {
                        tile.grid[coord.1 + y][coord.0 + x] = 'O';
                    }
                }
            }

            if found {
                println!("\n\n FOUND THEM");
                tile.print();

                let mut rough_water = 0;
                for y in 0..full_size {
                    for x in 0..full_size {
                        if tile.grid[y][x] == '#' {
                            rough_water += 1;
                        }
                    }
                }

                println!("Rough Water: {}", rough_water);
                return Ok(());
            }
        }
    }

    Ok(())
}
