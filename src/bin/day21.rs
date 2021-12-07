fn main() {
    let boss = Character { hp: 109, damage: 8, armor: 2 };

    let equipment = available_equipment();

    let cheapest = equipment.iter()
        .filter(|items| {
            let player = Character { hp: 100, damage: items.damage, armor: items.armor };
            fight(&player, &boss) == Winner::Player
        })
        .map(|items| items.cost).min();
    println!("Part one: the least you can spend to win is {}", cheapest.unwrap());

    let lamest = equipment.iter()
        .filter(|items| {
            let player = Character { hp: 100, damage: items.damage, armor: items.armor };
            fight(&player, &boss) == Winner::Boss
        })
        .map(|items| items.cost).max();
    println!("Part one: the most you can spend while losing is {}", lamest.unwrap());
}

struct Character {
    hp: i32,
    damage: i32,
    armor: i32,
}

#[derive(PartialEq, Eq)]
enum Winner { Player, Boss }

fn fight(player: &Character, boss: &Character) -> Winner {
    let hits_player = div_ceil(boss.hp, (player.damage - boss.armor).max(1));
    let hits_boss = div_ceil(player.hp, (boss.damage - player.armor).max(1));
    if hits_player <= hits_boss { Winner::Player } else { Winner::Boss }
}

fn div_ceil(a: i32, b: i32) -> i32 {
    let q = a / b;
    let r = a % b;
    if r == 0 { q } else { q + 1 }
}

#[derive(Clone, Copy)]
struct Item {
    cost: i32,
    damage: i32,
    armor: i32,
}

const Weapons: &[Item] = &[
    Item { cost: 8, damage: 4, armor: 0 },
    Item { cost: 10, damage: 5, armor: 0 },
    Item { cost: 25, damage: 6, armor: 0 },
    Item { cost: 40, damage: 7, armor: 0 },
    Item { cost: 74, damage: 8, armor: 0 },
];

const Armors: &[Item] = &[
    Item { cost: 0, damage: 0, armor: 0 },
    Item { cost: 13, damage: 0, armor: 1 },
    Item { cost: 31, damage: 0, armor: 2 },
    Item { cost: 53, damage: 0, armor: 3 },
    Item { cost: 75, damage: 0, armor: 4 },
    Item { cost: 102, damage: 0, armor: 5 },
];

const Rings: &[Item] = &[
    Item { cost: 0, damage: 0, armor: 0 },
    Item { cost: 0, damage: 0, armor: 0 },
    Item { cost: 25, damage: 1, armor: 0 },
    Item { cost: 50, damage: 2, armor: 0 },
    Item { cost: 100, damage: 3, armor: 0 },
    Item { cost: 20, damage: 0, armor: 1 },
    Item { cost: 40, damage: 0, armor: 2 },
    Item { cost: 80, damage: 0, armor: 3 },
];

impl std::ops::Add for Item {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Item {
            cost: self.cost + other.cost,
            damage: self.damage + other.damage,
            armor: self.armor + other.armor
        }
    }
}

fn available_equipment() -> Vec<Item> {
    let mut result = Vec::new();
    for weapon in Weapons {
        for armor in Armors {
            for ring1 in Rings {
                for ring2 in Rings {
                    if !std::ptr::eq(ring1, ring2) {
                        result.push(*weapon + *armor + *ring1 + *ring2);
                    }
                }
            }
        }
    }
    result
}
