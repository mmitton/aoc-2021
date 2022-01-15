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

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
enum ID {
    Immune(usize),
    Infection(usize),
}

impl ID {
    fn same_side(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (Self::Immune(_), Self::Immune(_)) => true,
            (Self::Infection(_), Self::Infection(_)) => true,
            _ => false,
        }
    }

    fn num(&self) -> usize {
        match self {
            Self::Immune(id) | Self::Infection(id) => *id,
        }
    }
}

impl std::fmt::Display for ID {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Immune(id) => write!(fmt, "Immune System group {}", id),
            Self::Infection(id) => write!(fmt, "Infection group {}", id),
        }
    }
}

#[derive(Debug, Clone)]
struct Unit {
    id: ID,
    num: usize,
    hp: usize,
    weak: Vec<String>,
    immune: Vec<String>,
    attack: String,
    damage: usize,
    initiative: usize,
}

fn load_input(filename: &str) -> Result<Vec<Unit>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();
    let mut units = vec![Vec::new(); 2];
    let mut idx = usize::MAX;

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        let line = line.replace(")", "");
        let line = line.as_str();
        if line == "" || line.starts_with("#") {
            continue;
        }

        match line {
            "Immune System:" => idx = 0,
            "Infection:" => idx = 1,
            _ => {
                let parts: Vec<&str> = line.split(" with an attack that does ").collect();
                assert!(parts.len() == 2);

                let (num, hp, weak, immune) = {
                    let mut weak = Vec::new();
                    let mut immune = Vec::new();
                    let parts: Vec<&str> = parts[0].split("(").collect();
                    if parts.len() == 2 {
                        for part in parts[1].split("; ") {
                            if part.starts_with("weak to") {
                                for part in part[8..].split(", ") {
                                    weak.push(part.to_string());
                                }
                            } else if part.starts_with("immune to") {
                                for part in part[10..].split(", ") {
                                    immune.push(part.to_string());
                                }
                            }
                        }
                    }

                    let parts: Vec<&str> = parts[0].split(" ").collect();
                    let num = parts[0].parse()?;
                    let hp = parts[4].parse()?;

                    (num, hp, weak, immune)
                };

                let (damage, attack, initiative) = {
                    let parts: Vec<&str> = parts[1].split(" ").collect();
                    (parts[0].parse()?, parts[1].to_string(), parts[5].parse()?)
                };

                let id = units[idx].len() + 1;
                units[idx].push(Unit {
                    id: if idx == 0 {
                        ID::Immune(id)
                    } else {
                        ID::Infection(id)
                    },
                    num: num,
                    hp: hp,
                    weak: weak,
                    immune: immune,
                    attack,
                    damage,
                    initiative,
                });
            }
        }
    }

    let mut ret = Vec::new();
    for i in 0..=1 {
        for unit in units[i].drain(..) {
            ret.push(unit);
        }
    }

    Ok(ret)
}

fn battle(units: &Vec<Unit>, boost: usize) -> Option<usize> {
    let mut units = units.clone();
    for i in 0..units.len() {
        match units[i].id {
            ID::Immune(_) => units[i].damage += boost,
            _ => {}
        }
    }

    let mut picked = Vec::new();
    let mut attacks = BTreeMap::new();

    loop {
        let mut immune_cnt = 0;
        let mut infection_cnt = 0;
        let mut total_units = 0;
        units.sort_by(|a, b| a.id.cmp(&b.id));
        for i in 0..units.len() {
            if units[i].num > 0 {
                total_units += units[i].num;
                match units[i].id {
                    ID::Immune(_) => immune_cnt += 1,
                    ID::Infection(_) => infection_cnt += 1,
                }
            }
        }
        if infection_cnt == 0 {
            return Some(total_units);
        } else if immune_cnt == 0 {
            return None;
        }

        units.sort_by(|a, b| {
            if a.id.same_side(&b.id) {
                let a_ef = a.damage * a.num;
                let b_ef = b.damage * b.num;

                if a_ef == b_ef {
                    b.initiative.cmp(&a.initiative)
                } else {
                    b_ef.cmp(&a_ef)
                }
            } else {
                a.id.cmp(&b.id)
            }
        });

        picked.clear();
        attacks.clear();

        for i in 0..units.len() {
            if units[i].num == 0 {
                continue;
            }

            let mut best_damage = 0;
            let mut best_ef = 0;
            let mut best_idx = usize::MAX;
            let mut best_individual_damage = 0;

            for j in 0..units.len() {
                if i == j || units[j].num == 0 {
                    continue;
                }
                if picked.contains(&j) {
                    continue;
                }
                if units[i].id.same_side(&units[j].id) {
                    continue;
                }

                if units[j].immune.contains(&units[i].attack) {
                    continue;
                }

                let mut damage = units[i].num * units[i].damage;
                let mut individual_damage = units[i].damage;
                if units[j].weak.contains(&units[i].attack) {
                    damage *= 2;
                    individual_damage *= 2;
                }

                let ef = units[j].num * units[j].damage;
                let mut attack = false;
                if damage > best_damage {
                    attack = true;
                } else if damage == best_damage {
                    if ef > best_ef {
                        attack = true;
                    } else if ef == best_ef {
                        if units[j].initiative > units[best_idx].initiative {
                            attack = true;
                        }
                    }
                }

                if attack {
                    best_damage = damage;
                    best_idx = j;
                    best_ef = ef;
                    best_individual_damage = individual_damage;
                }
            }

            if best_idx != usize::MAX {
                picked.push(best_idx);
                attacks.insert(units[i].id, (units[best_idx].id, best_individual_damage));
            }
        }

        if attacks.len() == 0 {
            return None;
        }

        units.sort_by(|a, b| b.initiative.cmp(&a.initiative));
        for i in 0..units.len() {
            if units[i].num == 0 || !attacks.contains_key(&units[i].id) {
                continue;
            }
            let (attack_id, damage) = *attacks.get(&units[i].id).unwrap();
            let mut attack_idx = usize::MAX;
            for j in 0..units.len() {
                if units[j].id == attack_id {
                    attack_idx = j;
                    break;
                }
            }
            let damage = units[i].num * damage;
            let mut killed = damage / units[attack_idx].hp;
            if killed > units[attack_idx].num {
                killed = units[attack_idx].num;
            }

            units[attack_idx].num = units[attack_idx].num.checked_sub(killed).unwrap();
        }
    }
}

fn main() -> Result<(), Error> {
    let units = load_input(INPUT_FILE)?;

    for boost in 0..usize::MAX {
        println!("boost: {}", boost);
        if let Some(units_left) = battle(&units, boost) {
            println!("{} units left after a boost of {}", units_left, boost);
            break;
        }
    }

    Ok(())
}
