use std::{collections::HashMap, convert::TryInto};

fn main() {
    let input: u64 = 29000000;
    let mut test: HashMap<u64, u64> = HashMap::new();

    let mut house_n = 0;
    for i in 1..input/10 {
        for j in (i..input/10).step_by(i.try_into().unwrap()) {
            test.entry(j).and_modify(|x| { *x += i * 10 }).or_insert(i * 10);
            if test.get(&j).unwrap() > &input {
                break;
            }
        }
    }

    for n in 0..test.len() {
        match test.get(&(n as u64)) {
            Some(x) => if x > &input {
                house_n = n as u64;
                break;
            },
            None => { continue; }
        };
    }

    println!("Answer: {}", house_n);
}