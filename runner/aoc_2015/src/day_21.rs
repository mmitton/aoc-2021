#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Debug, Default, Copy, Clone)]
struct Player {
    hp: isize,
    damage: isize,
    armor: isize,
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
struct Item {
    name: &'static str,
    cost: usize,
    damage: isize,
    armor: isize,
}
const WEAPONS: [Item; 5] = [
    Item {
        name: "Dagger",
        cost: 8,
        damage: 4,
        armor: 0,
    },
    Item {
        name: "Shortsword",
        cost: 10,
        damage: 5,
        armor: 0,
    },
    Item {
        name: "Warhammer",
        cost: 25,
        damage: 6,
        armor: 0,
    },
    Item {
        name: "Longsword",
        cost: 40,
        damage: 7,
        armor: 0,
    },
    Item {
        name: "Greataxe",
        cost: 74,
        damage: 8,
        armor: 0,
    },
];

const ARMORS: [Item; 5] = [
    Item {
        name: "Leather",
        cost: 13,
        damage: 0,
        armor: 1,
    },
    Item {
        name: "Chainmail",
        cost: 31,
        damage: 0,
        armor: 2,
    },
    Item {
        name: "Splintmail",
        cost: 53,
        damage: 0,
        armor: 3,
    },
    Item {
        name: "Bandedmail",
        cost: 75,
        damage: 0,
        armor: 4,
    },
    Item {
        name: "Platemail",
        cost: 102,
        damage: 0,
        armor: 5,
    },
];

const RINGS: [Item; 6] = [
    Item {
        name: "Damage +1",
        cost: 25,
        damage: 1,
        armor: 0,
    },
    Item {
        name: "Damage +2",
        cost: 50,
        damage: 2,
        armor: 0,
    },
    Item {
        name: "Damage +3",
        cost: 100,
        damage: 3,
        armor: 0,
    },
    Item {
        name: "Defense +1",
        cost: 20,
        damage: 0,
        armor: 1,
    },
    Item {
        name: "Defense +2",
        cost: 40,
        damage: 0,
        armor: 2,
    },
    Item {
        name: "Defense +3",
        cost: 80,
        damage: 0,
        armor: 3,
    },
];

#[derive(Default)]
pub struct Day21 {
    boss: Player,
}

impl Day21 {
    pub fn new() -> Self {
        Self::default()
    }

    fn fight(&self, player: &Player) -> bool {
        let boss_damage = if self.boss.damage <= player.armor {
            1
        } else {
            self.boss.damage - player.armor
        };
        let player_damage = if player.damage <= self.boss.armor {
            1
        } else {
            player.damage - self.boss.armor
        };

        let boss_turns_to_die = (self.boss.hp + player_damage - 1) / player_damage;
        let player_turns_to_die = (player.hp + boss_damage - 1) / boss_damage;

        if player_turns_to_die >= boss_turns_to_die {
            return true;
        }

        false
    }

    fn solve(&self) -> (usize, usize) {
        let mut loss = usize::MIN;
        let mut win = usize::MAX;

        let armor_options: Vec<Option<usize>> =
            vec![None, Some(0), Some(1), Some(2), Some(3), Some(4)];
        let mut ring_options: Vec<Vec<usize>> = Vec::new();
        ring_options.push(Vec::new());
        for i in 0..RINGS.len() {
            ring_options.push(vec![i]);
            for j in i + 1..RINGS.len() {
                ring_options.push(vec![i, j]);
                for k in j + 1..RINGS.len() {
                    ring_options.push(vec![i, j, k]);
                }
            }
        }
        println!("ring_options: {:?}", ring_options);

        for weapon in WEAPONS.iter() {
            for armor in armor_options.iter() {
                for rings in ring_options.iter() {
                    let mut cost = weapon.cost;
                    let mut player = Player {
                        hp: 100,
                        damage: weapon.damage,
                        armor: 0,
                    };
                    if let Some(armor) = armor {
                        cost += ARMORS[*armor].cost;
                        player.armor += ARMORS[*armor].armor;
                    }
                    for ring in rings.iter() {
                        cost += RINGS[*ring].cost;
                        player.armor += RINGS[*ring].armor;
                        player.damage += RINGS[*ring].damage;
                    }

                    if !self.fight(&player) {
                        loss = loss.max(cost);
                    } else {
                        win = win.min(cost);
                    }
                }
            }
        }

        (win, loss)
    }
}

impl Runner for Day21 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            if let Some((name, value)) = line.split_once(": ") {
                match name {
                    "Hit Points" => self.boss.hp = value.parse()?,
                    "Damage" => self.boss.damage = value.parse()?,
                    "Armor" => self.boss.armor = value.parse()?,
                    _ => return Err(Error::InvalidInput(line.into())),
                }
            } else {
                return Err(Error::InvalidInput(line.into()));
            }
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.solve().0.into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self.solve().1.into())
    }
}
