#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NoSolution,
}

fn load_input(filename: &str) -> Result<Vec<(Vec<Unit>, Map)>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();
    let mut map = Vec::new();
    let mut units = Vec::new();
    let mut y = 0;

    let mut ret = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim().to_string();
        if line == "" {
            if map.len() > 0 {
                ret.push((units, Map(map)));
            }
            map = Vec::new();
            units = Vec::new();
            y = 0;
            continue;
        }

        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            row.push(match c {
                'E' | 'G' => {
                    units.push(Unit {
                        y: y,
                        x: x,
                        elf: c == 'E',
                        health: 200,
                        attack_power: 3,
                    });
                    '.'
                }
                _ => c,
            });
        }
        map.push(row);

        y += 1;
    }
    if map.len() > 0 {
        ret.push((units, Map(map)));
    }

    Ok(ret)
}

struct Map(Vec<Vec<char>>);

impl Map {
    fn get(&self, x: usize, y: usize, units: &Vec<Unit>) -> char {
        let (c, _) = self.get_full(x, y, units);
        c
    }

    fn get_full(&self, x: usize, y: usize, units: &Vec<Unit>) -> (char, Option<isize>) {
        fn get_unit(x: usize, y: usize, units: &Vec<Unit>) -> Option<&Unit> {
            for unit in units {
                if unit.health > 0 && unit.x == x && unit.y == y {
                    return Some(unit);
                }
            }

            None
        }

        if let Some(unit) = get_unit(x, y, units) {
            if unit.elf {
                ('E', Some(unit.health))
            } else {
                ('G', Some(unit.health))
            }
        } else {
            (self.0[y][x], None)
        }
    }

    fn print(&self, units: &Vec<Unit>) {
        for y in 0..self.0.len() {
            let mut units_health = Vec::new();
            for x in 0..self.0[y].len() {
                let (c, health) = self.get_full(x, y, units);
                if let Some(health) = health {
                    units_health.push(format!("{}({})", c, health));
                }
                print!("{}", c);
            }
            if units_health.len() > 0 {
                print!("   {}", units_health.join(", "));
            }
            println!();
        }
    }

    fn find_path(
        &self,
        from: &(usize, usize),
        to: &(usize, usize),
        units: &Vec<Unit>,
    ) -> Option<(usize, (usize, usize))> {
        let deltas = [(0, -1), (-1, 0), (1, 0), (0, 1)];
        let mut queue = Vec::new();
        let mut seen = Vec::new();
        seen.push((from.0, from.1));
        for delta in &deltas {
            let x = (from.0 as isize + delta.0) as usize;
            let y = (from.1 as isize + delta.1) as usize;

            if self.get(x, y, units) != '.' {
                continue;
            }
            if x == to.0 && y == to.1 {
                return Some((0, (to.0, to.1)));
            }
            queue.push(vec![(x, y)]);
            seen.push((x, y));
        }

        let mut i = 0;
        while i < queue.len() {
            for delta in &deltas {
                let lp = queue[i][queue[i].len() - 1];
                let x = (lp.0 as isize + delta.0) as usize;
                let y = (lp.1 as isize + delta.1) as usize;

                if seen.contains(&(x, y)) || self.get(x, y, units) != '.' {
                    continue;
                }

                if x == to.0 && y == to.1 {
                    return Some((queue[i].len(), queue[i][0]));
                }

                let mut new_path = queue[i].clone();
                new_path.push((x, y));
                seen.push((x, y));
                queue.push(new_path);
            }
            i += 1;
        }

        None
    }

    fn process_round(&self, units: &mut Vec<Unit>) -> bool {
        fn get_units(units: &Vec<Unit>, elf: bool) -> Vec<usize> {
            let mut pre_ret = Vec::new();
            for i in 0..units.len() {
                if units[i].health <= 0 || units[i].elf != elf {
                    continue;
                }
                pre_ret.push((units[i].y, units[i].x, i));
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
            if enemy_units.len() == 0 {
                return true;
            }

            let mut positions = Vec::new();
            let mut can_attack = Vec::new();
            for enemy_unit in &enemy_units {
                for delta in deltas {
                    let x = (units[*enemy_unit].x as isize + delta.0) as usize;
                    let y = (units[*enemy_unit].y as isize + delta.1) as usize;
                    if x == units[i].x && y == units[i].y {
                        can_attack.push(*enemy_unit);
                    } else if self.get(x, y, units) == '.' {
                        positions.push((x, y));
                    }
                }
            }

            if can_attack.len() == 0 {
                if positions.len() == 0 {
                    continue;
                }

                let mut best_dist = usize::MAX;
                let mut best_move = None;
                for position in &positions {
                    if let Some(path) = self.find_path(&(units[i].x, units[i].y), &position, units)
                    {
                        if path.0 < best_dist {
                            best_dist = path.0;
                            best_move = Some(path.1);
                        }
                    }
                }

                if let Some(best_move) = best_move {
                    println!("Moving {:?} to {:?}", units[i], best_move);
                    units[i].x = best_move.0;
                    units[i].y = best_move.1;
                }

                // Look for attacks
                for delta in deltas {
                    let x = (units[i].x as isize + delta.0) as usize;
                    let y = (units[i].y as isize + delta.1) as usize;

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

            if can_attack.len() > 0 {
                // Attack!
                let mut min_health = isize::MAX;
                let mut min_unit = usize::MAX;
                for i in can_attack {
                    if units[i].health < min_health {
                        min_health = units[i].health;
                        min_unit = i;
                    }
                }
                println!("{:?} attacks {:?}", units[i], units[min_unit]);
                units[min_unit].health -= units[i].attack_power;
            }
        }

        false
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Unit {
    y: usize,
    x: usize,
    elf: bool,
    health: isize,
    attack_power: isize,
}

fn main() -> Result<(), Error> {
    let inputs = load_input(INPUT_FILE)?;

    'input_loop: for (mut units, map) in inputs {
        map.print(&units);
        for round in 1..usize::MAX {
            let done = map.process_round(&mut units);

            if cfg!(debug_assertions) {
                println!("After {} rounds", round);
                map.print(&units);
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
                for unit in &units {
                    if unit.health > 0 {
                        hp += unit.health as usize;
                        elf = unit.elf;
                    }
                }
                println!("Combat ends after {} full rounds", winning_round);
                println!(
                    "{} win with {} total hit points left",
                    if elf { "Elves" } else { "Goblins" },
                    hp
                );
                println!(
                    "Outcome: {} * {} = {}",
                    winning_round,
                    hp,
                    winning_round * hp
                );
                continue 'input_loop;
            }
        }
        return Err(Error::NoSolution);
    }

    return Ok(());
}
