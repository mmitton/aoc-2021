#[allow(unused_imports)]
use helper::{
    print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, Point, RunOutput, Runner,
};

#[derive(Clone, Default)]
struct Map {
    grid: Vec<Vec<Tile>>,
    empty_key: usize,
}

#[derive(Copy, Clone, PartialEq)]
enum Tile {
    Empty(usize),
    Wall,
    Elf(isize),
    Goblin(isize),
}

impl Map {
    fn get(&self, p: Point<u8>) -> Tile {
        self.grid[p.y as usize][p.x as usize]
    }

    fn find_path(&mut self, from: Point<u8>, to: Point<u8>) -> Option<(u8, Point<u8>)> {
        self.empty_key += 1;
        let deltas = [(0, -1), (-1, 0), (1, 0), (0, 1)];
        let mut queue = Vec::new();
        for delta in &deltas {
            let next = Point::new(
                (from.x as i8 + delta.0) as u8,
                (from.y as i8 + delta.1) as u8,
            );

            if !matches!(self.get(next), Tile::Empty(_)) {
                continue;
            }
            if next == to {
                return Some((0, next));
            }
            queue.push((next, 1, next));
            self.grid[next.y as usize][next.x as usize] = Tile::Empty(self.empty_key);
        }

        let mut i = 0;
        while i < queue.len() {
            for delta in &deltas {
                let lp = queue[i].2;
                let next = Point::new((lp.x as i8 + delta.0) as u8, (lp.y as i8 + delta.1) as u8);

                if !matches!(self.get(next), Tile::Empty(e) if e != self.empty_key) {
                    continue;
                }

                if next == to {
                    return Some((queue[i].1, queue[i].0));
                }

                queue.push((queue[i].0, queue[i].1 + 1, next));
                self.grid[next.y as usize][next.x as usize] = Tile::Empty(self.empty_key);
            }
            i += 1;
        }

        None
    }

    fn process_round(&mut self, units: &mut [Unit]) -> bool {
        fn get_units(units: &[Unit], elf: bool) -> Vec<usize> {
            let mut pre_ret = Vec::new();
            for (i, unit) in units.iter().enumerate() {
                if unit.health <= 0 || unit.elf != elf {
                    continue;
                }
                pre_ret.push((unit.y, unit.x, i));
            }
            pre_ret.sort();
            let mut ret = Vec::new();
            for unit in pre_ret {
                ret.push(unit.2);
            }
            ret
        }

        let deltas = [(0, -1), (-1, 0), (1, 0), (0, 1)];

        units.sort();
        for i in 0..units.len() {
            if units[i].health <= 0 {
                continue;
            }

            let enemy_units = get_units(units, !units[i].elf);
            if enemy_units.is_empty() {
                return true;
            }

            let mut positions = Vec::new();
            let mut can_attack = Vec::new();
            for enemy_unit in &enemy_units {
                for delta in deltas {
                    let p = Point::new(
                        (units[*enemy_unit].x as i8 + delta.0) as u8,
                        (units[*enemy_unit].y as i8 + delta.1) as u8,
                    );
                    if p.x == units[i].x && p.y == units[i].y {
                        can_attack.push(*enemy_unit);
                    } else if matches!(self.get(p), Tile::Empty(_)) {
                        positions.push(p);
                    }
                }
            }

            if can_attack.is_empty() {
                if positions.is_empty() {
                    continue;
                }

                let mut best_dist = u8::MAX;
                let mut best_move = None;
                for position in positions.iter() {
                    if let Some(path) =
                        self.find_path(Point::new(units[i].x, units[i].y), *position)
                    {
                        if path.0 < best_dist {
                            best_dist = path.0;
                            best_move = Some(path.1);
                        }
                    }
                }

                if let Some(best_move) = best_move {
                    self.grid[units[i].y as usize][units[i].x as usize] = Tile::Empty(0);
                    units[i].x = best_move.x;
                    units[i].y = best_move.y;
                    self.grid[units[i].y as usize][units[i].x as usize] = units[i].tile();
                }

                // Look for attacks
                for delta in deltas {
                    let x = (units[i].x as i8 + delta.0) as u8;
                    let y = (units[i].y as i8 + delta.1) as u8;

                    for j in 0..units.len() {
                        if units[j].health > 0
                            && units[j].elf != units[i].elf
                            && units[j].x == x
                            && units[j].y == y
                        {
                            can_attack.push(j);
                        }
                    }
                }
            }

            if !can_attack.is_empty() {
                // Attack!
                let mut min_health = isize::MAX;
                let mut min_unit = usize::MAX;
                for i in can_attack {
                    if units[i].health < min_health {
                        min_health = units[i].health;
                        min_unit = i;
                    }
                }
                units[min_unit].health -= units[i].attack_power;
                self.grid[units[min_unit].y as usize][units[min_unit].x as usize] =
                    units[min_unit].tile();
            }
        }

        false
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Unit {
    y: u8,
    x: u8,
    elf: bool,
    health: isize,
    attack_power: isize,
}

impl Unit {
    fn tile(&self) -> Tile {
        if self.health <= 0 {
            Tile::Empty(0)
        } else if self.elf {
            Tile::Elf(self.health)
        } else {
            Tile::Goblin(self.health)
        }
    }
}

#[derive(Default)]
pub struct Day15 {
    map: Map,
    units: Vec<Unit>,
}

impl Day15 {
    pub fn new() -> Self {
        Self::default()
    }

    fn play_game(&self, attack_power: isize, part2: bool) -> Option<usize> {
        let mut map = self.map.clone();
        let mut units = self.units.clone();
        for unit in &mut units {
            if unit.elf {
                unit.attack_power = attack_power;
            }
        }

        for round in 1.. {
            let done = map.process_round(&mut units);
            if part2 && units.iter().filter(|u| u.elf && u.health <= 0).count() != 0 {
                return None;
            }

            let winning_round = if done {
                Some(round - 1)
            } else {
                let mut winning_round = None;
                // Check to see if one side is defeated
                for elf in [false, true] {
                    let mut all_dead = true;
                    for unit in &units {
                        if unit.elf == elf && unit.health > 0 {
                            all_dead = false;
                            break;
                        }
                    }
                    if all_dead {
                        winning_round = Some(round);
                        break;
                    }
                }

                winning_round
            };

            if let Some(winning_round) = winning_round {
                let mut hp = 0;
                let mut elf = false;
                let mut any_dead = false;
                for unit in &units {
                    if unit.elf && unit.health <= 0 {
                        any_dead = true;
                    }
                    if unit.health > 0 {
                        hp += unit.health as usize;
                        elf = unit.elf;
                    }
                }
                if part2 && any_dead {
                    return None;
                }
                if part2 && !elf {
                    return None;
                }
                return Some(winning_round * hp);
            }
        }
        None
    }
}

impl Runner for Day15 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for (y, line) in lines.iter().enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                row.push(match c {
                    'E' => {
                        self.units.push(Unit {
                            y: y as u8,
                            x: x as u8,
                            elf: true,
                            health: 200,
                            attack_power: 3,
                        });
                        Tile::Elf(200)
                    }
                    'G' => {
                        self.units.push(Unit {
                            y: y as u8,
                            x: x as u8,
                            elf: false,
                            health: 200,
                            attack_power: 3,
                        });
                        Tile::Goblin(200)
                    }
                    '.' => Tile::Empty(0),
                    '#' => Tile::Wall,
                    _ => return Err(Error::InvalidInput(c.into())),
                });
            }
            self.map.grid.push(row);
        }
        Ok(())
    }

    fn run_part(&mut self, part: u8) -> Result<RunOutput, Error> {
        match part {
            1 => self.part1(),
            2 => self.part2(),
            _ => Err(Error::Skipped),
        }
    }
}

impl Day15 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.play_game(3, false).unwrap().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        // Find first win
        for attack_power in (10..).step_by(10) {
            // Back up
            if let Some(outcome) = self.play_game(attack_power, true) {
                let mut last_outcome = outcome;
                for attack_power in (3..attack_power).rev() {
                    match self.play_game(attack_power, true) {
                        Some(outcome) => last_outcome = outcome,
                        None => return Ok(last_outcome.into()),
                    }
                }
            }
        }
        Err(Error::Unsolved)
    }
}
