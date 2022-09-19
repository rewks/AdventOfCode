use itertools::Itertools;

fn sum(v: &Vec<&u64>) -> u64 {
    v.iter().fold(0u64, |sum, i| sum + **i)
}

fn product(v: &Vec<&u64>) -> u64 {
    v.iter().fold(1u64, |prod, i| prod * **i)
}

fn main() {
    let data = include_str!("../input.txt");
    let mut presents: Vec<u64> = Vec::new();

    for line in data.lines() {
        presents.push(line.parse().unwrap());
    }

    let total_weight: u64 = presents.iter().sum();
    let compartments: u64 = 4; // 3 for part 1, 4 for part 2

    let mut lowest_number_of_presents = usize::MAX;
    let mut lowest_quantum_entanglement = u64::MAX;

    for n in 0..presents.len()/(compartments as usize) {
        let combs = presents.iter().combinations(n);
        for c in combs {
            if sum(&c) == total_weight / compartments {
                if c.len() <= lowest_number_of_presents {
                    lowest_number_of_presents = c.len();
                    lowest_quantum_entanglement = lowest_quantum_entanglement.min(product(&c));
                }
            }
        }
    }

    println!("Lowest number of presents: {}, Quantum Entanglement: {}", lowest_number_of_presents, lowest_quantum_entanglement);
}
