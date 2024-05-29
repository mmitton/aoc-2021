#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Clone)]
struct Tile {
    grid: [[char; 10]; 10],
    edge_connections: [Option<(usize, usize)>; 4],
    is_rotated: bool,
    x: isize,
    y: isize,
}

impl Tile {
    fn edges(&self) -> Vec<Option<[char; 10]>> {
        let mut edges = Vec::new();

        let mut left = [' '; 10];
        let mut right = [' '; 10];

        for y in 0..10 {
            left[y] = self.grid[y][0];
            right[y] = self.grid[y][9];
        }

        edges.push(Some(self.grid[0].clone()));
        edges.push(Some(right));
        edges.push(Some(self.grid[9].clone()));
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
            let mut new_grid = [[' '; 10]; 10];
            for y in 0..10 {
                for x in 0..10 {
                    new_grid[y][x] = self.grid[9 - x][y];
                }
            }

            self.grid = new_grid;
        }

        if flip_x {
            let mut new_grid = [[' '; 10]; 10];
            for y in 0..10 {
                for x in 0..10 {
                    new_grid[y][x] = self.grid[y][9 - x];
                }
            }

            self.grid = new_grid;
        }

        if flip_y {
            let mut new_grid = [[' '; 10]; 10];
            for y in 0..self.grid.len() {
                for x in 0..self.grid[y].len() {
                    new_grid[y][x] = self.grid[9 - y][x];
                }
            }

            self.grid = new_grid;
        }
    }
}

pub struct Day20 {
    tiles: HashMap<usize, Tile>,
}

impl Day20 {
    pub fn new() -> Self {
        Self {
            tiles: HashMap::default(),
        }
    }

    fn place_tiles(&mut self) -> Vec<usize> {
        let tile_nums: Vec<usize> = self.tiles.keys().copied().collect();

        let mut min_x = 0isize;
        let mut max_x = 0isize;
        let mut min_y = 0isize;
        let mut max_y = 0isize;

        let mut queue = vec![0];
        let mut i = 0;
        while i < queue.len() {
            let tile1 = tile_nums[queue[i]];
            for (edge_num1, edge1) in self.tiles.get(&tile1).unwrap().edges().iter().enumerate() {
                let edge_num2 = (edge_num1 + 2) % 4;
                if let Some(edge1) = edge1 {
                    for j in 0..tile_nums.len() {
                        let tile2 = tile_nums[j];
                        if tile2 == tile1 {
                            continue;
                        }

                        let mut rotations = Vec::new();
                        {
                            let tile2 = self.tiles.get(&tile2).unwrap();
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
                                        let tile1 = self.tiles.get_mut(&tile1).unwrap();

                                        tile1.set_edge(edge_num1, (tile2, edge_num2));
                                        (tile1.x, tile1.y)
                                    };

                                    let t2 = self.tiles.get_mut(&tile2).unwrap();
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

        for tile in self.tiles.values_mut() {
            tile.x -= min_x;
            tile.y -= min_y;
        }

        let mut corners = Vec::new();
        for (tile_num, tile) in self.tiles.iter_mut() {
            let mut count = 0;
            for edge in &tile.edge_connections {
                if edge.is_some() {
                    count += 1;
                }
            }

            assert!(count >= 2);
            if count == 2 {
                corners.push(*tile_num);
                if tile.edge_connections[1].is_some() && tile.edge_connections[2].is_some() {
                    tile.x = 0;
                    tile.y = 0;
                }
            }
        }
        corners
    }
}

impl Runner for Day20 {
    fn parse(&mut self, path: &str, _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_path(path, LinesOpt::RAW)?;

        for lines in lines.chunks(12) {
            let mut tile = Tile {
                grid: [[' '; 10]; 10],
                edge_connections: [None; 4],
                is_rotated: false,
                x: 0,
                y: 0,
            };
            let tile_num: usize = lines[0][5..lines[0].len() - 1].parse()?;

            for (y, line) in lines[1..11].iter().enumerate() {
                line.chars()
                    .enumerate()
                    .for_each(|(x, c)| tile.grid[y][x] = c);
            }
            self.tiles.insert(tile_num, tile);
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.place_tiles().iter().product::<usize>().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        self.place_tiles();

        let mut waves: HashSet<(usize, usize)> = HashSet::default();
        let mut waves_width = usize::MIN;
        let mut waves_height = usize::MIN;

        for tile in self.tiles.values() {
            let sx = tile.x as usize * 8;
            let sy = tile.y as usize * 8;
            for y in 0..8 {
                for x in 0..8 {
                    if tile.grid[y + 1][x + 1] == '#' {
                        waves.insert((sx + x, sy + y));
                        waves_width = waves_width.max(sx + x + 1);
                        waves_height = waves_height.max(sy + y + 1);
                    }
                }
            }
        }

        let sea_monster_array = [
            "                  # ",
            "#    ##    ##    ###",
            " #  #  #  #  #  #   ",
        ];

        #[derive(Debug)]
        struct SeaMonster {
            deltas: Vec<(usize, usize)>,
            width: usize,
            height: usize,
        }
        fn rotate(sea_monster: &[&str], count: usize, flip_x: bool, flip_y: bool) -> SeaMonster {
            let mut sea_monster: Vec<Vec<char>> = sea_monster
                .iter()
                .map(|row| row.chars().collect())
                .collect();
            for _ in 0..count {
                let mut new_sea_monster = Vec::new();
                for x in 0..sea_monster[0].len() {
                    let mut row = Vec::new();
                    for y in (0..sea_monster.len()).rev() {
                        row.push(sea_monster[y][x]);
                    }
                    new_sea_monster.push(row);
                }

                sea_monster = new_sea_monster;
            }

            if flip_x {
                sea_monster.iter_mut().for_each(|row| row.reverse());
            }

            if flip_y {
                sea_monster.reverse();
            }

            let mut deltas = Vec::new();
            sea_monster.iter().enumerate().for_each(|(y, row)| {
                row.iter().enumerate().for_each(|(x, c)| {
                    if *c == '#' {
                        deltas.push((x, y));
                    }
                })
            });

            let width = deltas.iter().map(|c| c.0).max().unwrap() + 1;
            let height = deltas.iter().map(|c| c.1).max().unwrap() + 1;
            SeaMonster {
                deltas,
                width,
                height,
            }
        }

        let mut sea_monsters = Vec::new();
        for r in 0..4 {
            for flip in 0..3 {
                let sea_monster = rotate(&sea_monster_array, r, flip == 1, flip == 2);
                sea_monsters.push(sea_monster);
            }
        }

        let mut sea_monster_waves = HashSet::default();
        for sea_monster in sea_monsters.iter() {
            for y in 0..=waves_height - sea_monster.height {
                'search: for x in 0..=waves_width - sea_monster.width {
                    for (dx, dy) in sea_monster.deltas.iter() {
                        if !waves.contains(&(x + dx, y + dy)) {
                            continue 'search;
                        }
                    }

                    // Sea monster found
                    for (dx, dy) in sea_monster.deltas.iter() {
                        sea_monster_waves.insert((x + dx, y + dy));
                    }
                }
            }
        }

        Ok((waves.len() - sea_monster_waves.len()).into())
    }
}
