#[allow(dead_code)]
struct Item<'a> {
    name: &'a str,
    damage: i32,
    armor: i32,
    cost: i32,
}

#[derive(Debug, Copy, Clone)]
struct Player {
    hp: i32,
    damage: i32,
    armor: i32,
}

fn wins(mut player: Player, mut boss: Player) -> bool {
    loop {
        boss.hp -= 1.max(player.damage - boss.armor);
        if boss.hp <= 0 {
            return true;
        }
        player.hp -= 1.max(boss.damage - player.armor);
        if player.hp <= 0 {
            return false;
        }
    }
}

pub fn part_a() -> i32 {
    let mut weapons = Vec::new();
    let mut armors = Vec::new();
    let mut rings = Vec::new();
    weapons.push(Item {
        name: "Dagger",
        damage: 4,
        armor: 0,
        cost: 8,
    });
    weapons.push(Item {
        name: "Shortsword",
        damage: 5,
        armor: 0,
        cost: 10,
    });
    weapons.push(Item {
        name: "Warhammer",
        damage: 6,
        armor: 0,
        cost: 25,
    });
    weapons.push(Item {
        name: "Longsword",
        damage: 7,
        armor: 0,
        cost: 40,
    });
    weapons.push(Item {
        name: "Greataxe",
        damage: 8,
        armor: 0,
        cost: 74,
    });
    armors.push(Item {
        name: "Leather",
        damage: 0,
        armor: 1,
        cost: 13,
    });
    armors.push(Item {
        name: "Chainmail",
        damage: 0,
        armor: 2,
        cost: 31,
    });
    armors.push(Item {
        name: "Splintmail",
        damage: 0,
        armor: 3,
        cost: 53,
    });
    armors.push(Item {
        name: "Bandedmail",
        damage: 0,
        armor: 4,
        cost: 75,
    });
    armors.push(Item {
        name: "Platemail",
        damage: 0,
        armor: 5,
        cost: 102,
    });
    rings.push(Item {
        name: "Damage +1",
        damage: 1,
        armor: 0,
        cost: 25,
    });
    rings.push(Item {
        name: "Damage +2",
        damage: 2,
        armor: 0,
        cost: 50,
    });
    rings.push(Item {
        name: "Damage +3",
        damage: 3,
        armor: 0,
        cost: 100,
    });
    rings.push(Item {
        name: "Defense +1",
        damage: 0,
        armor: 1,
        cost: 20,
    });
    rings.push(Item {
        name: "Defense +2",
        damage: 0,
        armor: 2,
        cost: 40,
    });
    rings.push(Item {
        name: "Defense +3",
        damage: 0,
        armor: 3,
        cost: 80,
    });

    // without armor
    armors.push(Item {
        name: "",
        damage: 0,
        armor: 0,
        cost: 0,
    });
    // without two rings
    rings.push(Item {
        name: "None 1",
        damage: 0,
        armor: 0,
        cost: 0,
    });
    rings.push(Item {
        name: "None 2",
        damage: 0,
        armor: 0,
        cost: 0,
    });

    let mut min_cost = std::i32::MAX;
    let boss = Player {
        hp: 103,
        damage: 9,
        armor: 2,
    };
    for weapon in &weapons {
        for armor in &armors {
            for (i, ring_1) in rings.iter().enumerate() {
                for ring_2 in &rings[i + 1..] {
                    let player = Player {
                        hp: 100,
                        damage: weapon.damage + ring_1.damage + ring_2.damage,
                        armor: armor.armor + ring_1.armor + ring_2.armor,
                    };
                    if wins(player, boss) {
                        min_cost =
                            min_cost.min(weapon.cost + armor.cost + ring_1.cost + ring_2.cost);
                    }
                }
            }
        }
    }
    min_cost
}

pub fn part_b() -> i32 {
    let mut weapons = Vec::new();
    let mut armors = Vec::new();
    let mut rings = Vec::new();
    weapons.push(Item {
        name: "Dagger",
        damage: 4,
        armor: 0,
        cost: 8,
    });
    weapons.push(Item {
        name: "Shortsword",
        damage: 5,
        armor: 0,
        cost: 10,
    });
    weapons.push(Item {
        name: "Warhammer",
        damage: 6,
        armor: 0,
        cost: 25,
    });
    weapons.push(Item {
        name: "Longsword",
        damage: 7,
        armor: 0,
        cost: 40,
    });
    weapons.push(Item {
        name: "Greataxe",
        damage: 8,
        armor: 0,
        cost: 74,
    });
    armors.push(Item {
        name: "Leather",
        damage: 0,
        armor: 1,
        cost: 13,
    });
    armors.push(Item {
        name: "Chainmail",
        damage: 0,
        armor: 2,
        cost: 31,
    });
    armors.push(Item {
        name: "Splintmail",
        damage: 0,
        armor: 3,
        cost: 53,
    });
    armors.push(Item {
        name: "Bandedmail",
        damage: 0,
        armor: 4,
        cost: 75,
    });
    armors.push(Item {
        name: "Platemail",
        damage: 0,
        armor: 5,
        cost: 102,
    });
    rings.push(Item {
        name: "Damage +1",
        damage: 1,
        armor: 0,
        cost: 25,
    });
    rings.push(Item {
        name: "Damage +2",
        damage: 2,
        armor: 0,
        cost: 50,
    });
    rings.push(Item {
        name: "Damage +3",
        damage: 3,
        armor: 0,
        cost: 100,
    });
    rings.push(Item {
        name: "Defense +1",
        damage: 0,
        armor: 1,
        cost: 20,
    });
    rings.push(Item {
        name: "Defense +2",
        damage: 0,
        armor: 2,
        cost: 40,
    });
    rings.push(Item {
        name: "Defense +3",
        damage: 0,
        armor: 3,
        cost: 80,
    });

    // without armor
    armors.push(Item {
        name: "",
        damage: 0,
        armor: 0,
        cost: 0,
    });
    // without two rings
    rings.push(Item {
        name: "None 1",
        damage: 0,
        armor: 0,
        cost: 0,
    });
    rings.push(Item {
        name: "None 2",
        damage: 0,
        armor: 0,
        cost: 0,
    });

    let mut most_cost = std::i32::MIN;
    let boss = Player {
        hp: 103,
        damage: 9,
        armor: 2,
    };
    for weapon in &weapons {
        for armor in &armors {
            for (i, ring_1) in rings.iter().enumerate() {
                for ring_2 in &rings[i + 1..] {
                    let player = Player {
                        hp: 100,
                        damage: weapon.damage + ring_1.damage + ring_2.damage,
                        armor: armor.armor + ring_1.armor + ring_2.armor,
                    };
                    if !wins(player, boss) {
                        most_cost =
                            most_cost.max(weapon.cost + armor.cost + ring_1.cost + ring_2.cost);
                    }
                }
            }
        }
    }
    most_cost
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_result() {
        assert_eq!(part_a(), 121);
    }

    #[test]
    fn part_b_result() {
        assert_eq!(part_b(), 201);
    }
}
