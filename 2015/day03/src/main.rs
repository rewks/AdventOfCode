use std::time::Instant;
use std::collections::HashMap;

#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Clone)]
#[derive(Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn mv(&mut self, direction: char) {
        match direction {
            '<' => self.x -= 1,
            '>' => self.x += 1,
            'v' => self.y -= 1,
            '^' => self.y += 1,
            _ => (),
        };
    }
}

fn part_one(data: &str) -> usize {
    let mut houses = HashMap::new();
    let mut current_position = Position {
        x: 0,
        y: 0
    };
    houses.insert(current_position, true);

    for c in data.chars() {
        current_position.mv(c);
        houses.insert(current_position, true);
    }

    houses.len()
}

fn part_two(data: &str) -> usize {
    let mut houses = HashMap::new();
    let mut santa_position = Position {
        x: 0,
        y: 0
    };
    let mut robot_position = Position {
        x: 0,
        y: 0
    };
    houses.insert(santa_position, true);

    for (i, c) in data.chars().enumerate() {
        if i % 2 == 1 {
            santa_position.mv(c);
            houses.insert(santa_position, true);
        } else {
            robot_position.mv(c);
            houses.insert(robot_position, true);
        }
    }

    houses.len()
}

fn main() {
    let start = Instant::now();
    let data = include_str!("../input.txt");
    
    println!("Number of unique houses visited by Santa: {}", part_one(data));
    println!("Number of unique houses visited by Santa and Robot Santa: {}", part_two(data));
    println!("Execution time: {}ms", start.elapsed().as_millis());
}