use std::collections::{HashSet, HashMap};

#[derive(Debug, Eq, Hash, PartialEq)]
struct AuntSue {
    id: u32,
    children: i32,
    cats: i32,
    samoyeds: i32,
    pomerians: i32,
    akitas: i32,
    vizslas: i32,
    goldfish: i32,
    trees: i32,
    cars: i32,
    perfumes: i32,
}

impl From<&str> for AuntSue {
    fn from(data: &str) -> AuntSue {
        let tokens: Vec<&str> = data.split(" ").collect();
        let id = tokens[1].trim_end_matches(":").parse().unwrap();
        let mut compounds: HashMap<&str, i32> = HashMap::new();
        for i in (2..tokens.len() - 1).step_by(2) {
            let compound = tokens[i].trim_end_matches(":");
            let quantity = tokens[i+1].trim_end_matches(",").parse().unwrap();
            compounds.insert(compound, quantity);
        }

        let mut aunt = AuntSue {
            id,
            children: i32::MIN,
            cats: i32::MIN,
            samoyeds: i32::MIN,
            pomerians: i32::MIN,
            akitas: i32::MIN,
            vizslas: i32::MIN,
            goldfish: i32::MIN,
            trees: i32::MIN,
            cars: i32::MIN,
            perfumes: i32::MIN
        };

        for (property, quantity) in compounds {
            match property {
                "children" => { aunt.children = quantity; },
                "cats" => { aunt.cats = quantity; },
                "samoyeds" => { aunt.samoyeds = quantity; },
                "pomerians" => { aunt.pomerians = quantity; },
                "akitas" => { aunt.akitas = quantity; },
                "vizslas" => { aunt.vizslas = quantity; },
                "goldfish" => { aunt.goldfish = quantity; },
                "trees" => { aunt.trees = quantity; },
                "cars" => { aunt.cars = quantity; },
                "perfumes" => { aunt.perfumes = quantity; },
                _ => { continue; },
            }
        }

        aunt
    }
}

fn possible_match(sue: &AuntSue, to_match: &AuntSue) -> bool {
    if sue.children != i32::MIN && sue.children != to_match.children { return false; }
    if sue.samoyeds != i32::MIN && sue.samoyeds != to_match.samoyeds { return false; }
    if sue.akitas != i32::MIN && sue.akitas != to_match.akitas { return false; }
    if sue.vizslas != i32::MIN && sue.vizslas != to_match.vizslas { return false; }
    if sue.cars != i32::MIN && sue.cars != to_match.cars { return false; }
    if sue.perfumes != i32::MIN && sue.perfumes != to_match.perfumes { return false; }

    // For part 1 comparison between sues needs to be !=
    // For part 2 comparison between sues needs to be <
    if sue.cats != i32::MIN && sue.cats < to_match.cats { return false; }
    if sue.trees != i32::MIN && sue.trees < to_match.trees { return false; }

    // For part 1 comparison between sues needs to be !=
    // For part 2 comparison between sues needs to be >
    if sue.pomerians != i32::MIN && sue.pomerians > to_match.pomerians { return false; }
    if sue.goldfish != i32::MIN && sue.goldfish > to_match.goldfish { return false; }
    true
}

fn main() {
    let data = include_str!("../input.txt");
    let mut sues: HashSet<AuntSue> = HashSet::new();
    let the_sue = AuntSue::from("Sue 0: children: 3, cats: 7, samoyeds: 2, pomerians: 3, akitas: 0, vizslas: 0, goldfish: 5, trees: 3, cars: 2, perfumes: 1");

    for l in data.lines() {
        sues.insert(AuntSue::from(l));
    }

    for sue in sues {
        if possible_match(&sue, &the_sue) {
            println!("Answer: {}", sue.id);
            break;
        }
    }
}