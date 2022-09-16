use std::collections::{HashMap, HashSet};

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
struct Reindeer {
    speed: u32,
    duration: u32,
    rest: u32,
}

impl From<&str> for Reindeer {
    fn from(data: &str) -> Reindeer {
        let tokens: Vec<&str> = data.split(" ").collect();
        Reindeer {
            speed: tokens[3].parse().unwrap(),
            duration: tokens[6].parse().unwrap(),
            rest: tokens[13].parse().unwrap()
        }
    }
}


fn get_distance_travelled(reindeer: &Reindeer, seconds: u32) -> u32{
    let movement_cycle = reindeer.duration + reindeer.rest;
    let quot = seconds / movement_cycle;
    let rem = seconds % movement_cycle;

    let mut dist = quot * reindeer.speed * reindeer.duration;
    dist += rem.min(reindeer.duration) * reindeer.speed;

    dist
}

fn get_winner(reindeer: &HashSet<Reindeer>,seconds: u32) -> u32 {
    let mut results: HashMap<Reindeer, u32> = HashMap::new();

    // calculate distance travelled of all reindeer
    for r in reindeer {
        results.insert(*r, get_distance_travelled(r, seconds));
    }
    // find entry with maximum value (dist travelled) and return
    let (_r, d) = results.iter().max_by_key(|v|  v.1).unwrap();
    *d
}

fn get_winner2(reindeer: &HashSet<Reindeer>,seconds: u32) -> u32 {
    let mut results: HashMap<Reindeer, u32> = HashMap::new();
    let mut points: HashMap<Reindeer, u32> = HashMap::new();

    // iterate n times so points can be awarded each second
    for i in 1..=seconds {
        // calculate distance travelled of all reindeer at current time
        for r in reindeer {
            results.insert(*r, get_distance_travelled(r, i));
        }
        // increment points for reindeer (or reindeers) with furthest dist travelled at current time
        let max_d = *results.iter().max_by_key(|v|  v.1).unwrap().1;
        for r in results.keys() {
            match *results.get(r).unwrap() == max_d {
                true => {
                    points.entry(*r).and_modify(|v| { *v += 1 } ).or_insert(1);
                },
                false => {
                    continue;
                }
            };
        }
    }

    *points.iter().max_by_key(|v| v.1).unwrap().1
}

fn main() {
    let data = include_str!("../input.txt");
    let mut reindeer: HashSet<Reindeer> = HashSet::new();

    // Generate set of Reindeer objects
    for line in data.lines() {
        let r = Reindeer::from(line);
        reindeer.insert(r);
    }

    let part1_winning_distance = get_winner(&reindeer, 2503);

    let part2_winning_points = get_winner2(&reindeer, 2503);
    
    println!("Part 1: {}", part1_winning_distance);
    println!("Part 2: {}", part2_winning_points);
}