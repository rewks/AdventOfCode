use fancy_regex::{Regex, Captures};
use std::collections::{HashMap, HashSet};

struct Recombobulator {
    ops: HashMap<String, Vec<String>>,
}

impl From<&str> for Recombobulator {
    fn from(data: &str) -> Recombobulator {
        let mut ops: HashMap<String, Vec<String>> = HashMap::new();

        for l in data.lines() {
            if !l.contains("=>") {
                continue;
            }
            let tokens: Vec<&str> = l.split(" ").collect();
            ops.entry(tokens[0].to_string())
                .and_modify(|v| v.push(tokens[2].to_string()))
                .or_insert(vec![tokens[2].to_string()]);
        }

        Recombobulator { ops }
    }
}

impl Recombobulator {
    fn find_possibilities(&self, molecule: &String) -> HashSet<String> {
        let mut possibilities: HashSet<String> = HashSet::new();

        for (pattern, replacements) in &self.ops {
            let re = Regex::new(pattern.as_str()).unwrap();
            let caps = re.captures_iter(&molecule);

            for cap in caps {
                let (start_index, end_index) = match cap {
                    Ok(i) => (i.get(0).unwrap().start(), i.get(0).unwrap().end()),
                    Err(_) => continue,
                };

                for r in replacements {
                    let mut new_m = molecule.clone();
                    new_m.replace_range(start_index..end_index, r.as_str());
                    possibilities.insert(new_m);
                }
            }
        }

        possibilities
    }
}

fn main() {
    let data = include_str!("../input.txt");
    let recomb = Recombobulator::from(data);

    let target_molecule = data.lines().last().unwrap().to_string();

    println!(
        "Part 1: {}",
        recomb.find_possibilities(&target_molecule).len()
    );

    /* First effort: Brute force, exponential, impossible.

    let mut possibs: HashSet<String> = HashSet::new();
    possibs.insert("e".to_string());
    let mut steps = 0;
    loop {
        let mut current_step_possibs = HashSet::new();
        println!("Step {}: {:?} possibilities", steps, possibs.len());
        for m in &possibs {
            let t = m.as_str();
            current_step_possibs.extend(recomb.find_possibilities(&t.to_string()));
        }
        steps += 1;
        if current_step_possibs.contains(&target_molecule) {
            break;
        }
        possibs = current_step_possibs;
    }
    */

    /* Second effort: learning from smarter people at https://www.reddit.com/r/adventofcode/comments/3xflz8/day_19_solutions/ */
    let mut mol_r: String = target_molecule.chars().rev().collect();
    let mut ops_r: HashMap<String, String> = HashMap::new();
    for (k, v) in recomb.ops {
        for o in v {
            ops_r.insert(o.chars().rev().collect(), k.chars().rev().collect());
        }
    }
    
    let mut steps = 0;
    let pat = ops_r.keys().map(|s| &**s).collect::<Vec<&str>>().join("|");
    let re = Regex::new(pat.as_str()).unwrap();
    while mol_r.as_str() != "e" {
        mol_r = re.replace(mol_r.as_str(), |cg: &Captures| { ops_r.get(cg.get(0).unwrap().as_str()).unwrap() }).to_string();
        steps += 1;
    }

    println!("Part 2: {}", steps);
}