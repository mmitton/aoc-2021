use std::collections::BinaryHeap;

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

    fn effect(&self, player: &mut Player, boss: &mut Player) -> &'static str {
        match self {
            Self::MagicMissle | Self::Drain => panic!("{:?} is not an effect spell", self),
            Self::Shield => {
                player.armor += 7;
                ""
            }
            Self::Poison => {
                boss.hp -= 3;
                "Poison deals 3 damage"
            }
            Self::Recharge => {
                player.mana += 101;
                "Rechange provides 101 mana"
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

#[derive(Debug, Clone, Eq, PartialEq)]
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
            let msg = spell.effect(&mut self.player, &mut self.boss);
            if cfg!(debug_assertions) {
                if msg != "" {
                    println!("{}; its timer is now {}", msg, self.player.spells[i].0 - 1);
                } else {
                    println!(
                        "{:?}'s timer is now {}",
                        self.player.spells[i].1,
                        self.player.spells[i].0 - 1
                    );
                }
            }
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
                if cfg!(debug_assertions) {
                    println!("Player casts Magic Missle, dealing 4 damage");
                }
                self.boss.hp -= 4;
            }
            Spell::Drain => {
                if cfg!(debug_assertions) {
                    println!("Player casts Drain, dealing 2 damage, and healing 2 hit points.");
                }
                self.boss.hp -= 2;
                self.player.hp += 2
            }
            _ => {
                if cfg!(debug_assertions) {
                    println!("Player casts {:?}", spell);
                }
                self.player.spells.push((spell.duration(), spell.clone()));
            }
        }
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.mana_spent.partial_cmp(&self.mana_spent)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.mana_spent.cmp(&self.mana_spent)
    }
}

fn main() {
    let mut states: BinaryHeap<State> = BinaryHeap::new();
    let mut forced_order = Vec::new();
    if cfg!(debug_assertions) {
        forced_order.push(Spell::Recharge);
        forced_order.push(Spell::Shield);
        forced_order.push(Spell::Drain);
        forced_order.push(Spell::Poison);
        forced_order.push(Spell::MagicMissle);

        states.push(State {
            mana_spent: 0,
            player: Player {
                hp: 10,
                damage: 0,
                armor: 0,
                mana: 250,
                spells: Vec::new(),
            },
            boss: Player {
                hp: 14,
                damage: 8,
                armor: 0,
                mana: 0,
                spells: Vec::new(),
            },
        });
    } else {
        states.push(State {
            mana_spent: 0,
            player: Player {
                hp: 50,
                damage: 0,
                armor: 0,
                mana: 500,
                spells: Vec::new(),
            },
            boss: Player {
                hp: 71,
                damage: 10,
                armor: 0,
                mana: 0,
                spells: Vec::new(),
            },
        });
    }
    let spells = vec![
        Spell::MagicMissle,
        Spell::Drain,
        Spell::Shield,
        Spell::Poison,
        Spell::Recharge,
    ];

    while states.len() > 0 {
        let state = states.pop().unwrap();

        let mut spells = spells.clone();
        if forced_order.len() > 0 {
            let spell = forced_order.remove(0);
            spells.clear();
            spells.push(spell);
        }
        'spell_loop: for spell in &spells {
            if state.player.mana >= spell.mana() {
                for i in 0..state.player.spells.len() {
                    if state.player.spells[i].1 == *spell && state.player.spells[i].0 > 1 {
                        continue 'spell_loop;
                    }
                }

                // Woo hoo!
                if cfg!(debug_assertions) {
                    println!("\n-- Player turn --");
                    println!(
                        "- Player has {} hit points, {} armor, {} mana",
                        state.player.hp, state.player.armor, state.player.mana
                    );
                    println!("- Boss has {} hit points", state.boss.hp);
                }
                let mut new_state = state.clone();
                new_state.player.hp -= 1;
                if new_state.player.hp <= 0 {
                    if cfg!(debug_assertions) {
                        println!("Lose... {:?}", new_state);
                    }
                    continue 'spell_loop;
                }

                new_state.mana_spent += spell.mana();
                new_state.player.mana -= spell.mana();
                new_state.apply_spells();
                new_state.cast(spell);

                if new_state.boss.hp <= 0 {
                    println!("Win!  {:?}", new_state);
                    return;
                }

                if cfg!(debug_assertions) {
                    println!("\n-- Boss turn --");
                    println!(
                        "- Player has {} hit points, {} armor, {} mana",
                        state.player.hp, state.player.armor, state.player.mana
                    );
                    println!("- Boss has {} hit points", state.boss.hp);
                }
                new_state.apply_spells();
                if new_state.boss.hp <= 0 {
                    println!("Win!  {:?}", new_state);
                    return;
                }

                let attack = if new_state.boss.damage <= new_state.player.armor {
                    1
                } else {
                    new_state.boss.damage - new_state.player.armor
                };
                if cfg!(debug_assertions) {
                    println!("Boss attacks for {} damage!", attack);
                }
                new_state.player.hp -= attack;

                if new_state.player.hp <= 0 {
                    if cfg!(debug_assertions) {
                        println!("Lose... {:?}", new_state);
                    }
                } else {
                    states.push(new_state);
                }
            }
        }
    }

    // println!("Answer: {}", answer);
}
