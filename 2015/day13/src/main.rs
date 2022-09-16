use std::collections::{HashMap, HashSet};
use itertools::Itertools;

fn get_max_happiness(relationships: &HashMap<(&str, &str), i32>, guests: &HashSet<&str>) -> i32 {
    let mut best_happiness = 0;
    for order in guests.iter().permutations(guests.len()) {
        let mut ph = 0;
        ph += match relationships.get(&(order[0], order[order.len() - 1])) {
            Some(v) => v,
            None => relationships.get(&(order[order.len() - 1], order[0])).unwrap()
        };
        for i in 0..order.len() - 1 {
            let p1 = *order[i];
            let p2 = *order[i + 1];
            ph += match relationships.get(&(p1, p2)) {
                Some(v) => v,
                None => relationships.get(&(p2, p1)).unwrap()
            }
        }

        if best_happiness < ph {
            best_happiness = ph;
        }
    }
    best_happiness
}

fn main() {
    let data = include_str!("../input.txt");
    let mut relationships: HashMap<(&str, &str), i32> = HashMap::new();
    let mut guests: HashSet<&str> = HashSet::new();

    // Build dictionary of all relationships, with their effective happiness score
    // Also build dictionary of guests and the number of neighbours they have
    for line in data.lines() {
        let tokens: Vec<&str> = line.split(" ").collect();
        let p1 = tokens[0];
        let p2 = tokens[10].strip_suffix(".").unwrap();
        let mut happiness: i32 = tokens[3].parse().unwrap();
        if tokens[2] == "lose" {
            happiness = 0 - happiness;
        }

        match relationships.contains_key(&(p2, p1)) {
            true => {
                relationships
                    .entry((p2, p1))
                    .and_modify(|v| *v += happiness);
            }
            false => {
                relationships.insert((p1, p2), happiness);
            }
        }

        guests.insert(p1);
    }

    // Calc part1 answer
    let part1_happiness = get_max_happiness(&relationships, &guests);
    
    // Add self to guestlist
    for p in &guests {
        relationships.insert(("me", *p), 0);
    }
    guests.insert("me");

    // Calc part2 answer
    let part2_happiness = get_max_happiness(&relationships, &guests);

    println!("Part 1: {}", part1_happiness);
    println!("Part 2: {}", part2_happiness);
}