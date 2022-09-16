use serde_json::Value;

fn part1(o: &Value) -> i64 {
    let n = match o.is_number() {
        true => o.as_i64().unwrap(),
        false => match o.is_string() {
            true => 0,
            false => match o.is_array() {
                true => {
                    let mut st = 0;
                    for item in o.as_array().unwrap() {
                        st += part1(item);
                    }
                    st
                },
                false => match o.is_object() {
                    true => {
                        let mut st = 0;
                        for (k, v) in o.as_object().unwrap() {
                            st += match k.parse::<i64>() {
                                Ok(x) => x,
                                Err(_) => 0,
                            };
                            st += part1(v);
                        }
                        st
                    },
                    false => panic!("What the shittery is going on here then")
                }
            }
        }
    };

    n
}

fn part2(o: &Value) -> (i64, bool) {
    let (n, r) = match o.is_number() {
        true => (o.as_i64().unwrap(), false),
        false => match o.is_string() {
            true => if o.as_str().unwrap() == "red" {
                return (0, true);
            } else {
                return (0, false);
            }
            false => match o.is_array() {
                true => {
                    let mut st = 0;
                    for item in o.as_array().unwrap() {
                        let p = part2(item);
                        st += p.0;
                    }
                    (st, false)
                },
                false => match o.is_object() {
                    true => {
                        let mut st = 0;
                        let mut red_check = false;
                        for (k, v) in o.as_object().unwrap() {
                            st += match k.parse::<i64>() {
                                Ok(x) => x,
                                Err(_) => 0
                            };
                            let p = part2(v);
                            st += p.0;
                            red_check = red_check | p.1;
                        }
                        if red_check {
                            st = 0;
                        }
                        (st, false)
                    },
                    false => panic!("What the shittery is going on here then")
                }
            }
        }
    };

    (n, r)
}

fn main() {
    let data = include_str!("../input.txt");
    let json_data: Value = serde_json::from_str(data).unwrap();
    
    println!("Part 1: {}", part1(&json_data));

    println!("Part 2: {}", part2(&json_data).0);
}