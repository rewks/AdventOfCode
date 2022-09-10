use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq)]
struct Light {
    x: u32,
    y: u32,
}

impl Light {
    fn new (x: u32, y: u32) -> Self {
        Light {
            x,
            y
        }
    }
}

fn get_light(coords: &str) -> Light {
    Light::new(coords.split(',').collect::<Vec<&str>>()[0].parse().unwrap(), coords.split(',').collect::<Vec<&str>>()[1].parse().unwrap())
}

fn part1(data: &str) -> u32 {
    let mut lights: HashMap<Light, bool> = HashMap::new();

    for line in data.lines() {
        let words: Vec<&str> = line.split(' ').collect();
        let (from, to) = match words[1] {
            "on"|"off" => (get_light(words[2]), get_light(words[4])),
            _ => (get_light(words[1]), get_light(words[3])),
        };

        for i in from.x..=to.x {
            for j in from.y..=to.y {
                let l = Light::new(i, j);
                match words[1] {
                    "on" => lights.insert(l, true),
                    "off" => lights.insert(l, false),
                    _ => {
                        let state = lights.get(&l);
                        match state {
                            Some(s) => lights.insert(l, !s),
                            None => lights.insert(l, true)
                        }
                    },
                };
            }
        }
        
    }

    let mut lights_on = 0;
    for i in lights.values() {
        if *i {
            lights_on += 1;
        }
    }

    lights_on
}

fn part2(data: &str) -> u32 {
    let mut lights: HashMap<Light, u32> = HashMap::new();

    for line in data.lines() {
        let words: Vec<&str> = line.split(' ').collect();
        let (from, to) = match words[1] {
            "on"|"off" => (get_light(words[2]), get_light(words[4])),
            _ => (get_light(words[1]), get_light(words[3])),
        };

        for i in from.x..=to.x {
            for j in from.y..=to.y {
                let l = Light::new(i, j);
                match words[1] {
                    "on" => lights.entry(l).and_modify(|v| { *v += 1 }).or_insert(1),
                    "off" => lights.entry(l).and_modify(|v| { if *v > 0 { *v -= 1 }}).or_insert(0),
                    _ => lights.entry(l).and_modify(|v| { *v += 2 }).or_insert(2),
                };
            }
        }
    }

    let mut lights_brightness = 0;
    for i in lights.values() {
        lights_brightness += i;
    }

    lights_brightness
}

fn main() {
    let data = include_str!("../input.txt");

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}