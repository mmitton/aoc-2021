#[derive(Debug, Copy, Clone)]
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

fn fight(player: &Player, boss: &Player, cost: usize) -> bool {
    let boss_damage = if boss.damage <= player.armor {
        1
    } else {
        boss.damage - player.armor
    };
    let player_damage = if player.damage <= boss.armor {
        1
    } else {
        player.damage - boss.armor
    };

    let boss_turns_to_die = (boss.hp + player_damage - 1) / player_damage;
    let player_turns_to_die = (player.hp + boss_damage - 1) / boss_damage;

    if player_turns_to_die >= boss_turns_to_die {
        println!(
            "Boss: {:?},{}  Player: {:?},{}  Cost: {}",
            boss, boss_turns_to_die, player, player_turns_to_die, cost
        );
        return true;
    }

    false
}

fn main() {
    if cfg!(debug_assertions) {
        assert!(fight(
            &Player {
                hp: 8,
                damage: 5,
                armor: 5,
            },
            &Player {
                hp: 12,
                damage: 7,
                armor: 2,
            },
            0,
        ));
        return;
    }
    let boss = Player {
        hp: 100,
        damage: 8,
        armor: 2,
    };

    let weapons = vec![
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

    let armors = vec![
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

    let rings = vec![
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

    let armor_options: Vec<Option<usize>> = vec![None, Some(0), Some(1), Some(2), Some(3), Some(4)];
    let mut ring_options: Vec<Vec<usize>> = Vec::new();
    ring_options.push(Vec::new());
    for i in 0..rings.len() {
        ring_options.push(vec![i]);
        for j in i + 1..rings.len() {
            ring_options.push(vec![i, j]);
            for k in j + 1..rings.len() {
                ring_options.push(vec![i, j, k]);
            }
        }
    }
    println!("ring_options: {:?}", ring_options);

    let mut answer = usize::MAX;
    for weapon in &weapons {
        for armor in &armor_options {
            for ring in &ring_options {
                let boss = boss.clone();
                let mut cost = weapon.cost;
                let mut player = Player {
                    hp: 100,
                    damage: weapon.damage,
                    armor: 0,
                };
                if let Some(armor) = armor {
                    cost += armors[*armor].cost;
                    player.armor += armors[*armor].armor;
                }
                for ring in ring {
                    cost += rings[*ring].cost;
                    player.armor += rings[*ring].armor;
                    player.damage += rings[*ring].damage;
                }

                if fight(&player, &boss, cost) && cost < answer {
                    answer = cost;
                }
            }
        }
    }

    println!("Answer: {}", answer);
}
