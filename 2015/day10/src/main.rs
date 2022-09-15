use itertools::Itertools;

fn look_and_say(s: String) -> String {
    let mut new_s = String::new();
    for (key, group) in &s.chars().group_by(|c| *c) {
        let n = group.collect::<Vec<char>>().len();
        new_s.push_str(&n.to_string());
        new_s.push(key);
    }

    new_s
}

/* naive solution... could not handle part 2
fn look_and_say(mut s: String) -> String {
    let mut new_s = String::new();

    while !s.is_empty() {
        let mut n = s.len();
        let c = s.chars().nth(0).unwrap();
        s = s.trim_start_matches(c).to_string();
        n -= s.len();
        new_s.push_str(&n.to_string());
        new_s.push(c);
    }

    new_s
}*/

fn main() {
    let mut data = "3113322113".to_string();

    for _i in 0..40 {
        data = look_and_say(data);
    }
    println!("Part 1: {}", data.len());

    for _i in 0..10 {
        data = look_and_say(data);
    }
    println!("Part 2: {}", data.len());
}