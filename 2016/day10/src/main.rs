use std::collections::{VecDeque, HashMap};

#[derive(PartialEq, Copy, Clone, Debug)]
enum Chip {
    Empty,
    Value(i32),
}

#[derive(Debug)]
struct Bot {
    chip1: Chip,
    chip2: Chip,
}

impl Bot {
    fn new(chip1: Chip) -> Self {
        Self {
            chip1: chip1,
            chip2: Chip::Empty,
        }
    }

    fn receive_chip(&mut self, chip: Chip) {
        if self.chip1 == Chip::Empty {
            self.chip1 = chip;
        } else if self.chip2 == Chip::Empty {
            self.chip2 = chip;
        }
    }

    fn remove_chip(&mut self, high_value: bool) -> Chip {
        let mut res = Chip::Empty;
        match self.chip1 {
            Chip::Empty => match self.chip2 {
                Chip::Empty => (),
                Chip::Value(_) => {
                    res = self.chip2.clone();
                    self.chip2 = Chip::Empty;
                }
            },
            Chip::Value(a) => match self.chip2 {
                Chip::Empty => {
                    res = self.chip1.clone();
                    self.chip1 = Chip::Empty;
                }
                Chip::Value(b) => {
                    match high_value {
                        true => {
                            if a > b {
                                res = self.chip1.clone();
                                self.chip1 = Chip::Empty;
                            } else {
                                res = self.chip2.clone();
                                self.chip2 = Chip::Empty;
                            }
                        },
                        false => {
                            if a < b {
                                res = self.chip1.clone();
                                self.chip1 = Chip::Empty;
                            } else {
                                res = self.chip2.clone();
                                self.chip2 = Chip::Empty;
                            }
                        }
                    };
                }
            },
        };
        res
    }

    fn get_number_of_chips(&self) -> u32 {
        let mut n = 0;
        if self.chip1 != Chip::Empty {
            n += 1;
        }

        if self.chip2 != Chip::Empty {
            n += 1;
        }
        n
    }
}

fn main() {
    let data = include_str!("../input.txt");
    let mut instructions: VecDeque<&str> = data.lines().collect();
    let mut bots: HashMap<u32, Bot> = HashMap::new();
    let mut outputs: HashMap<u32, Chip> = HashMap::new();

    while let Some(inst) = instructions.pop_front() {
        let tokens: Vec<&str> = inst.split(" ").collect();
        // Straight assignment of a chip to a bot
        if *tokens.get(0).unwrap() == "value" {
            let chip = Chip::Value(tokens.get(1).unwrap().parse::<i32>().unwrap());
            let bot_id = tokens.get(5).unwrap().parse::<u32>().unwrap();
            bots.entry(bot_id).and_modify(|b| b.receive_chip(chip)).or_insert(Bot::new(chip));
            continue;
        }

        // Instruction for a bot, check that bot exists and has 2 chips. If not, push instruction to back of queue
        let bot_id = tokens.get(1).unwrap().parse::<u32>().unwrap();
        match bots.get(&bot_id) {
            Some(b) => {
                if b.get_number_of_chips() != 2 {
                    instructions.push_back(inst);
                    continue;
                }
            },
            None => {
                instructions.push_back(inst);
                 continue;
            }
        }

        // Part 1 answer check:
        let c1 = bots.get(&bot_id).unwrap().chip1;
        let c2 = bots.get(&bot_id).unwrap().chip2;
        match c1 {
            Chip::Value(a) => match c2 {
                Chip::Value(b) => {
                    if (a == 61 && b == 17) || (b == 61 && a == 17) {
                        println!("Part 1: {}", bot_id);
                    }
                },
                _ => (),
            },
            _ => (),
        };

        // Bot exists and has two chips, process first half of instruction (always refers to lower value chip)
        let target_id = tokens.get(6).unwrap().parse::<u32>().unwrap();
        let chip = bots.get_mut(&bot_id).unwrap().remove_chip(false);
        // Check if chip is being given to another bot or an output and process appropriately
        match *tokens.get(5).unwrap() {
            "output" => { 
                outputs.entry(target_id).and_modify(|c| *c = chip).or_insert(chip);
            },
            _ => {
                bots.entry(target_id).and_modify(|b| b.receive_chip(chip)).or_insert(Bot::new(chip));
            },
        };

        // process second half of instruction (always refers to higher value chip)
        let target_id = tokens.get(11).unwrap().parse::<u32>().unwrap();
        let chip = bots.get_mut(&bot_id).unwrap().remove_chip(true);
        // Check if chip is being given to another bot or an output and process appropriately
        match *tokens.get(10).unwrap() {
            "output" => { 
                outputs.entry(target_id).and_modify(|c| *c = chip).or_insert(chip);
            },
            _ => {
                bots.entry(target_id).and_modify(|b| b.receive_chip(chip)).or_insert(Bot::new(chip));
            },
        };
    }

    // part 2 answer
    let (a, b, c) = match outputs.get(&0).unwrap() {
        Chip::Value(v1) => match outputs.get(&1).unwrap() {
            Chip::Value(v2) => match outputs.get(&2).unwrap() {
                Chip::Value(v3) => (*v1, *v2, *v3),
                _ => (*v1, *v2, 0i32),
            },
            _ => (*v1, 0i32, 0i32),
        },
        _ => (0i32, 0i32, 0i32),
    };
    let p2 = a * b * c;
    println!("Part 2: {}", p2);
}