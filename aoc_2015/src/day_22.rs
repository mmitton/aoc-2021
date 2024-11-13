#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Spell {
    MagicMissle,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    fn duration(&self) -> usize {
        match self {
            Self::MagicMissle => 0,
            Self::Drain => 0,
            Self::Shield => 6,
            Self::Poison => 6,
            Self::Recharge => 5,
        }
    }

    fn effect(&self, player: &mut Player, boss: &mut Player) {
        match self {
            Self::MagicMissle | Self::Drain => panic!("{:?} is not an effect spell", self),
            Self::Shield => {
                player.armor += 7;
            }
            Self::Poison => {
                boss.hp -= 3;
            }
            Self::Recharge => {
                player.mana += 101;
            }
        }
    }

    fn mana(&self) -> usize {
        match self {
            Self::MagicMissle => 53,
            Self::Drain => 73,
            Self::Shield => 113,
            Self::Poison => 173,
            Self::Recharge => 229,
        }
    }
}

enum Round {
    Win,
    Lose,
    Continue,
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
struct Player {
    hp: isize,
    damage: isize,
    armor: isize,
    mana: usize,
    spells: Vec<(usize, Spell)>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct State {
    mana_spent: usize,
    player: Player,
    boss: Player,
}

impl State {
    fn apply_spells(&mut self) {
        self.player.armor = 0;
        for i in (0..self.player.spells.len()).rev() {
            let spell = self.player.spells[i].1.clone();
            spell.effect(&mut self.player, &mut self.boss);
            if self.player.spells[i].0 <= 1 {
                self.player.spells.remove(i);
            } else {
                self.player.spells[i].0 -= 1;
            }
        }
    }

    fn cast(&mut self, spell: &Spell) {
        for active_spell in &self.player.spells {
            assert!(active_spell.1 != *spell);
        }
        match spell {
            Spell::MagicMissle => {
                self.boss.hp -= 4;
            }
            Spell::Drain => {
                self.boss.hp -= 2;
                self.player.hp += 2
            }
            _ => {
                self.player.spells.push((spell.duration(), spell.clone()));
            }
        }
    }

    fn play_round(&mut self, spell: &Spell, difficult: bool) -> Round {
        if difficult {
            self.player.hp -= 1;
            if self.player.hp <= 0 {
                return Round::Lose;
            }
        }

        self.mana_spent += spell.mana();
        self.player.mana -= spell.mana();
        self.apply_spells();
        self.cast(spell);

        if self.boss.hp <= 0 {
            return Round::Win;
        }

        self.apply_spells();
        if self.boss.hp <= 0 {
            return Round::Win;
        }

        let attack = if self.boss.damage <= self.player.armor {
            1
        } else {
            self.boss.damage - self.player.armor
        };
        self.player.hp -= attack;

        if self.player.hp > 0 {
            Round::Continue
        } else {
            Round::Lose
        }
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.mana_spent.cmp(&self.mana_spent)
    }
}

#[derive(Default)]
pub struct Day22 {
    boss: Player,
}

impl Day22 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn play(&self, difficult: bool) -> usize {
        let spells = [
            Spell::MagicMissle,
            Spell::Drain,
            Spell::Shield,
            Spell::Poison,
            Spell::Recharge,
        ];

        let mut states: BTreeMap<usize, Vec<State>> = BTreeMap::new();
        states.entry(0).or_default().push(State {
            mana_spent: 0,
            player: Player {
                hp: 50,
                damage: 0,
                armor: 0,
                mana: 500,
                spells: Vec::new(),
            },
            boss: self.boss.clone(),
        });

        while let Some((_, state)) = states.pop_first() {
            for state in state.iter() {
                'spell_loop: for spell in spells.iter() {
                    if state.player.mana >= spell.mana() {
                        for i in 0..state.player.spells.len() {
                            if state.player.spells[i].1 == *spell && state.player.spells[i].0 > 1 {
                                continue 'spell_loop;
                            }
                        }

                        let mut new_state = state.clone();

                        match new_state.play_round(spell, difficult) {
                            Round::Win => return new_state.mana_spent,
                            Round::Continue => states
                                .entry(new_state.mana_spent)
                                .or_default()
                                .push(new_state),
                            Round::Lose => {}
                        }
                    }
                }
            }
        }

        0
    }
}

impl Runner for Day22 {
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
        Ok(self.play(false).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self.play(true).into())
    }
}
