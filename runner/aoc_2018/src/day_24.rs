#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};
use std::cmp::Ordering;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
enum ID {
    None,
    Immune(usize),
    Infection(usize),
}

impl ID {
    fn same_side(&self, rhs: &Self) -> bool {
        matches!(
            (self, rhs),
            (Self::Immune(_), Self::Immune(_)) | (Self::Infection(_), Self::Infection(_))
        )
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Damage {
    Radiation,
    Bludgeoning,
    Fire,
    Slashing,
    Cold,
}

impl FromStr for Damage {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "radiation" => Ok(Self::Radiation),
            "bludgeoning" => Ok(Self::Bludgeoning),
            "fire" => Ok(Self::Fire),
            "slashing" => Ok(Self::Slashing),
            "cold" => Ok(Self::Cold),
            _ => Err(Error::InvalidInput(s.into())),
        }
    }
}

#[derive(Debug, Clone)]
struct Unit {
    id: ID,
    num: usize,
    hp: usize,
    weak: Vec<Damage>,
    immune: Vec<Damage>,
    attack: Damage,
    damage: usize,
    initiative: usize,
}

impl FromStr for Unit {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn split_line(s: &str) -> Result<(&str, Option<&str>, &str), Error> {
            let open_paren = s.find('(');
            let close_paren = s.find(')');
            match (open_paren, close_paren) {
                (Some(open_paren), Some(close_paren)) => Ok((
                    &s[..open_paren - 12],
                    Some(&s[open_paren + 1..close_paren]),
                    &s[close_paren + 27..],
                )),
                (None, None) => {
                    if let Some((front, back)) =
                        s.split_once(" hit points with an attack that does ")
                    {
                        Ok((front, None, back))
                    } else {
                        Err(Error::InvalidInput(s.into()))
                    }
                }
                _ => Err(Error::InvalidInput(s.into())),
            }
        }
        let (num_hp, characteristics, damage_initiative) = split_line(s)?;
        let mut num_hp = num_hp.split_whitespace();
        let num: usize = num_hp.next().unwrap().parse()?;
        let hp: usize = num_hp.last().unwrap().parse()?;
        let mut weak = Vec::new();
        let mut immune = Vec::new();

        if let Some(characteristics) = characteristics {
            for characteristics in characteristics.split("; ") {
                if let Some(weak_to) = characteristics.strip_prefix("weak to ") {
                    for part in weak_to.split(", ") {
                        weak.push(part.parse()?);
                    }
                } else if let Some(immune_to) = characteristics.strip_prefix("immune to ") {
                    for part in immune_to.split(", ") {
                        immune.push(part.parse()?);
                    }
                } else {
                    return Err(Error::InvalidInput(s.into()));
                }
            }
        }

        let mut damage_initiative = damage_initiative.split_whitespace();
        let damage: usize = damage_initiative.next().unwrap().parse()?;
        let attack: Damage = damage_initiative.next().unwrap().parse()?;
        let initiative: usize = damage_initiative.last().unwrap().parse()?;

        Ok(Self {
            id: ID::None,
            num,
            hp,
            weak,
            immune,
            damage,
            attack,
            initiative,
        })
    }
}

#[derive(Default)]
pub struct Day24 {
    units: Vec<Unit>,
}

impl Day24 {
    pub fn new() -> Self {
        Self::default()
    }

    fn battle(&self, boost: usize) -> Option<usize> {
        let mut units = self.units.clone();

        // Boost immune
        for unit in units.iter_mut() {
            if matches!(unit.id, ID::Immune(_)) {
                unit.damage += boost;
            }
        }

        let mut picked = Vec::new();
        let mut attacks = HashMap::default();

        loop {
            let mut immune_cnt = 0;
            let mut infection_cnt = 0;
            let mut total_units = 0;
            units.sort_by(|a, b| a.id.cmp(&b.id));
            for unit in units.iter_mut() {
                if unit.num > 0 {
                    total_units += unit.num;
                    match unit.id {
                        ID::Immune(_) => immune_cnt += 1,
                        ID::Infection(_) => infection_cnt += 1,
                        ID::None => unreachable!(),
                    }
                }
            }
            if infection_cnt == 0 || (boost == 0 && immune_cnt == 0) {
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
                    let attack = match damage.cmp(&best_damage) {
                        Ordering::Greater => true,
                        Ordering::Equal => {
                            ef > best_ef
                                || (ef == best_ef
                                    && units[j].initiative > units[best_idx].initiative)
                        }
                        Ordering::Less => false,
                    };

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

            if attacks.is_empty() {
                return None;
            }

            units.sort_by(|a, b| b.initiative.cmp(&a.initiative));
            for i in 0..units.len() {
                if units[i].num == 0 || !attacks.contains_key(&units[i].id) {
                    continue;
                }
                let (attack_id, damage) = *attacks.get(&units[i].id).unwrap();
                let attack_idx = units.iter().position(|u| u.id == attack_id).unwrap();
                let damage = units[i].num * damage;
                let mut killed = damage / units[attack_idx].hp;
                if killed > units[attack_idx].num {
                    killed = units[attack_idx].num;
                }

                units[attack_idx].num = units[attack_idx].num.checked_sub(killed).unwrap();
            }
        }
    }
}

impl Runner for Day24 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::REMOVE_EMPTY)?;
        let mut lines = lines.iter();

        let line = lines.next();
        if line != Some("Immune System:") {
            return Err(Error::InvalidInput(format!(
                "Expected Immune System, got {line:?}"
            )));
        }

        let mut id = 0;
        for line in &mut lines {
            if line == "Infection:" {
                break;
            }
            let mut unit: Unit = line.parse()?;
            id += 1;
            unit.id = ID::Immune(id);
            self.units.push(unit);
        }

        id = 0;
        for line in lines {
            let mut unit: Unit = line.parse()?;
            id += 1;
            unit.id = ID::Infection(id);
            self.units.push(unit);
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.battle(0).unwrap().into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        for boost in 1.. {
            if let Some(units_left) = self.battle(boost) {
                return Ok(units_left.into());
            }
        }
        Err(Error::Unsolved)
    }
}
