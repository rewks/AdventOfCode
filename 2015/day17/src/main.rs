use itertools::Itertools;

fn get_number_of_possible_combinations(containers: &Vec<u32>, use_minimum_containers: bool) -> u32 {
    let mut possible_combinations = 0;
    for n in 1..=containers.len() {
        let combs = containers.iter().combinations(n);
        for c in combs {
            let total: u32 = c.iter().copied().sum();
            if total == 150 {
                possible_combinations += 1;
            }
        }
        if use_minimum_containers && possible_combinations > 0 {
            break;
        }
    }

    possible_combinations
}

fn main() {
    let data = include_str!("../input.txt");
    let mut containers: Vec<u32> = Vec::new();
    for line in data.lines() {
        containers.push(line.parse().unwrap());
    }

    println!("Part 1: {}", get_number_of_possible_combinations(&containers, false));
    println!("Part 2: {}", get_number_of_possible_combinations(&containers, true));
}