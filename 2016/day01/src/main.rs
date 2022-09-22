
use std::collections::HashSet;
use num::complex::Complex;

fn main() {
    let data = include_str!("../input.txt").trim();

    let (mut x, mut y) = (0, 0);
    let mut orientation = Complex::new(0, 1);
    // Part 2 tracker
    let mut visited_locations: HashSet<(i32, i32)> = HashSet::from([(x, y)]);

    let steps: Vec<&str> = data.split(", ").collect();
    
    'outer: for step in steps {
        let direction = &step[..1];
        let distance: i32 = step[1..].parse().unwrap();

        if direction == "L" {
            orientation *= Complex::i();
        }
        if direction == "R" {
            orientation *= -Complex::i();
        }

        for _i in 0..distance {
            x += orientation.re;
            y += orientation.im;

            // Part 2 condition.
            if !visited_locations.insert((x, y)) {
                break 'outer;
            }
        }
    }

    println!("Answer: {}", (x + y).abs());
}