use std::collections::HashMap;

fn get_most_common_char(map: &HashMap<char, u32>) -> Option<&char> {
    map.iter().max_by(|a, b| a.1.cmp(&b.1)).map(|(k, _v)| k)
}

fn get_least_common_char(map: &HashMap<char, u32>) -> Option<&char> {
    map.iter().min_by(|a, b| a.1.cmp(&b.1)).map(|(k, _v)| k)
}

fn main() {
    let data = include_str!("../input.txt");
    let mut column_letter_counts: Vec<HashMap<char, u32>> = Vec::new();

    for l in data.lines() {
        for (i, c) in l.chars().enumerate() {
            match column_letter_counts.get_mut(i) {
                Some(hm) => {
                    hm.entry(c).and_modify(|x| *x += 1).or_insert(1);
                }
                None => {
                    column_letter_counts.push(HashMap::from([(c, 1)]));
                }
            };
        }
    }

    let mut p1_answer = String::new();
    let mut p2_answer = String::new();
    for map in column_letter_counts {
        match get_most_common_char(&map) {
            Some(c) => p1_answer.push(*c),
            None => panic!("No most common char found in {:?}", map),
        };
        match get_least_common_char(&map) {
            Some(c) => p2_answer.push(*c),
            None => panic!("No least common char found in {:?}", map),
        };
    }

    println!("Part 1: {}", p1_answer);
    println!("Part 2: {}", p2_answer);
}
