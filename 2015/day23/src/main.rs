use std::collections::HashMap;

fn parse_increment(text: &str) -> i32 {
    let sign = &text[0..1];
    let value = &text[1..];

    let result = match sign {
        "+" => value.parse::<i32>().unwrap(),
        "-" => 0i32 - value.parse::<i32>().unwrap(),
        _ => panic!("Oh crap"),
    };

    result
}

fn main() {
    let data = include_str!("../input.txt");
    let mut instructions: Vec<Vec<&str>> = Vec::new();

    for line in data.lines() {
        let tokens: Vec<&str> = line.split(" ").collect();
        instructions.push(tokens);
    }

    let mut registers: HashMap<String, u32> = HashMap::new();
    registers.insert(String::from("a"), 0); // change to 1 for part 2
    registers.insert(String::from("b"), 0);

    let mut i: usize = 0;

    while i < instructions.len() {
        let tokens = instructions.get(i).unwrap();

        match tokens[0] {
            "hlf" => {
                let r = String::from(tokens[1]);
                registers.entry(r).and_modify(|x| *x /= 2);
                i += 1;
            }
            "tpl" => {
                let r = String::from(tokens[1]);
                registers.entry(r).and_modify(|x| *x *= 3);
                i += 1;
            }
            "inc" => {
                let r = String::from(tokens[1]);
                registers.entry(r).and_modify(|x| *x += 1);
                i += 1;
            }
            "jmp" => {
                let inc = parse_increment(tokens[1]);
                i = (i as i32 + inc) as usize;
            }
            "jie" => {
                let inc = parse_increment(tokens[2]);
                if registers.get(&String::from(tokens[1].trim_end_matches(","))).unwrap() % 2 == 0 {
                    i = (i as i32 + inc) as usize;
                } else {
                    i += 1;
                }
            }
            "jio" => {
                let inc = parse_increment(tokens[2]);
                if *registers.get(&String::from(tokens[1].trim_end_matches(","))).unwrap() == 1 {
                    i = (i as i32 + inc) as usize;
                } else {
                    i += 1;
                }
            }
            _ => break,
        };
    }

    println!("a:{}, b:{}", registers.get(&String::from("a")).unwrap(), registers.get(&String::from("b")).unwrap());
}
