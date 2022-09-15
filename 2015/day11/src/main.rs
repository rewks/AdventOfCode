use std::{str::from_utf8, collections::HashSet};
use fancy_regex::Regex;


fn incr_pw(p: &mut String) -> String {
    unsafe {
        let old_pw = p.as_bytes_mut();

        for b in old_pw.iter_mut().rev() {
            if *b < 0x7a {
                *b += 0x01;
                break;
            }
            *b = 0x61;
        }
        from_utf8(old_pw).unwrap().to_string()
    }
}

fn test1(p: &String) -> bool {
    let bytes = p.as_bytes();
    for i in 0..bytes.len()-2 {
        let b = bytes[i];
        let b2 = bytes[i + 1];
        let b3 = bytes[i + 2];
        if ((b + 0x01) == b2) && ((b + 0x02) == b3) {
            return true;
        }
    }
    false
}

fn test2(p: &String) -> bool {
    if p.contains("i") || p.contains("o") || p.contains("l") {
        return false;
    }
    true
}

fn test3(p: &String) -> bool {
    let re = Regex::new(r"([a-z])\1{1}").unwrap();
    let mut start = 0;
    let mut seen_doubles = HashSet::new();

    while let Some(m) = re.captures_from_pos(p, start).unwrap() {
        seen_doubles.insert(m.get(1).unwrap().as_str());
        start = m.get(0).unwrap().end() + 1;
        if start > p.len() - 1 {
            break;
        }
    }

    seen_doubles.len() > 1
}

fn main() {
    let mut data = "hepxcrrq".to_string();

    loop {
        data = incr_pw(&mut data);
        if test1(&data) && test2(&data) && test3(&data) {
            break;
        }
    }

    println!("Part 1: {}", data);

    loop {
        data = incr_pw(&mut data);
        if test1(&data) && test2(&data) && test3(&data) {
            break;
        }
    }

    println!("Part 2: {}", data);
}