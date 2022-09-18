use std::collections::HashSet;

#[derive(Debug)]
struct NPC {
    health: i32,
    damage: i32,
    armour: i32,
}

impl From<&str> for NPC {
    fn from(data: &str) -> Self {
        let (mut health, mut damage, mut armour) = (0, 0, 0);
        for l in data.lines() {
            let t: Vec<&str> = l.split(": ").collect();
            match t[0] {
                "Hit Points" => health = t[1].parse().unwrap(),
                "Damage" => damage = t[1].parse().unwrap(),
                "Armor" => armour = t[1].parse().unwrap(),
                _ => continue,
            };
        }

        NPC {
            health,
            damage,
            armour,
        }
    }
}

impl NPC {
    fn new(health: i32, damage: i32, armour: i32) -> Self {
        NPC {
            health,
            damage,
            armour,
        }
    }

    fn take_damage(&mut self, amount: i32) {
        self.health -= amount;
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Item {
    name: String,
    cost: i32,
    damage: i32,
    armour: i32,
}

impl Item {
    fn new(name: &str, cost: i32, damage: i32, armour: i32) -> Self {
        Item {
            name: name.to_string(),
            cost,
            damage,
            armour,
        }
    }
}

fn build_weapons_list() -> HashSet<Item> {
    let mut items = HashSet::new();
    items.insert(Item::new("Dagger", 8, 4, 0));
    items.insert(Item::new("Shortsword", 10, 5, 0));
    items.insert(Item::new("Warhammer", 25, 6, 0));
    items.insert(Item::new("Longsword", 40, 7, 0));
    items.insert(Item::new("Greataxe", 74, 8, 0));
    items
}

fn build_armour_list() -> HashSet<Item> {
    let mut items = HashSet::new();
    items.insert(Item::new("No Armour", 0, 0, 0)); // Armour not required by ruleset
    items.insert(Item::new("Leather", 13, 0, 1));
    items.insert(Item::new("Chainmail", 31, 0, 2));
    items.insert(Item::new("Splintmail", 53, 0, 3));
    items.insert(Item::new("Bandedmail", 75, 0, 4));
    items.insert(Item::new("Platemail", 102, 0, 5));
    items
}

fn build_rings_list() -> HashSet<Item> {
    let mut items = HashSet::new();
    items.insert(Item::new("No Ring 1", 0, 0, 0)); // Ring 1 not required by ruleset
    items.insert(Item::new("No Ring 2", 0, 0, 0)); // Ring 2 not required by ruleset
    items.insert(Item::new("Damage +1", 25, 1, 0));
    items.insert(Item::new("Damage +2", 50, 2, 0));
    items.insert(Item::new("Damage +3", 100, 3, 0));
    items.insert(Item::new("Defence +1", 20, 0, 1));
    items.insert(Item::new("Defence +2", 40, 0, 2));
    items.insert(Item::new("Defence +3", 80, 0, 3));
    items
}

fn fight(player: &mut NPC, opponent: &mut NPC) -> bool {
    let mut rounds = 0;
    while player.health > 0 && opponent.health > 0 {
        if rounds % 2 == 0 {
            let damage = (player.damage - opponent.armour).max(1);
            opponent.take_damage(damage);
        } else {
            let damage = (opponent.damage - player.armour).max(1);
            player.take_damage(damage);
        }
        rounds += 1;
    }

    //println!("Fight over after {} rounds. Player: {}  Opponent: {}", rounds / 2, player.health, opponent.health);
    player.health > opponent.health
}

fn main() {
    let data = include_str!("../input.txt");
    let weapons = build_weapons_list();
    let armours = build_armour_list();
    let rings = build_rings_list();

    // Brute force approach below. Observe fight with all possible gear sets and record lowest cost victory and highest cost loss
    let mut lowest_cost_victory = i32::MAX;
    let mut highest_cost_loss = i32::MIN;
    for weapon in &weapons {
        for armour in &armours {
            for ring1 in &rings {
                for ring2 in &rings {
                    let gold_cost = weapon.cost + armour.cost + ring1.cost + ring2.cost;
                    let total_damage = weapon.damage + armour.damage + ring1.damage + ring2.damage;
                    let total_armour = weapon.armour + armour.armour + ring1.armour + ring2.armour;
                    //println!("Weapon: {}, Armour: {}, Ring1: {}, Ring2: {} | Cost: {}", weapon.name, armour.name, ring1.name, ring2.name, gold_cost);
                    let mut boss = NPC::from(data);
                    let mut p1 = NPC::new(100, total_damage, total_armour);
                    match fight(&mut p1, &mut boss) {
                        true => {
                            if gold_cost < lowest_cost_victory {
                                lowest_cost_victory = gold_cost;
                            }
                        }
                        false => {
                            if gold_cost > highest_cost_loss {
                                highest_cost_loss = gold_cost;
                            }
                        }
                    };
                }
            }
        }
    }

    println!("Part 1: {}", lowest_cost_victory);
    println!("Part 2: {}", highest_cost_loss);
}
