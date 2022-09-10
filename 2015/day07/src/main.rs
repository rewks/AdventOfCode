use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Gate {
    in1: String,
    in2: String,
    op: String,
}

impl From<&str> for Gate {
    fn from(instruction: &str) -> Gate {
        let instr: Vec<&str> = instruction.split(" ").collect();
        return match instr.len() {
            1 => Gate {
                in1: String::from(instr[0]),
                in2: String::from(""),
                op: String::from("IS"),
            },
            2 => Gate {
                in1: String::from(instr[1]),
                in2: String::from(""),
                op: String::from(instr[0]),
            },
            _ => Gate {
                in1: String::from(instr[0]),
                in2: String::from(instr[2]),
                op: String::from(instr[1]),
            }
        };
    }
}

struct Task {
    cache: HashMap<String, i32>,
    gates: HashMap<String, Gate>
}

impl Task {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
            gates: HashMap::new()
        }
    }

    fn get_wire_value(&mut self, target: String) -> i32 {
        let t = target.as_str();
        if !self.cache.contains_key(t) {
            let gate = self.gates.get(t).unwrap().clone();
            let op = gate.op.as_str();
            let in1 = match gate.in1.parse::<i32>() {
                Ok(n) => n,
                Err(_) => self.get_wire_value(gate.in1),
            };
            let value = match op {
                "IS" => in1,
                "NOT" => !in1,
                _ => {
                    let in2 = match gate.in2.parse::<i32>() {
                        Ok(n) => n,
                        Err(_) => self.get_wire_value(gate.in2),
                    };
                    match op {
                        "AND" => in1 & in2,
                        "OR" => in1 | in2,
                        "RSHIFT" => in1 >> in2,
                        "LSHIFT" => in1 << in2,
                        _ => panic!("wtf")
                    }
                }
            };
            self.cache.insert(String::from(t), value);
        }

        *self.cache.get(t).unwrap()
    }
}

fn main() {
    let data = include_str!("../input.txt");
    let mut task = Task::new();

    for line in data.lines() {
        let (instruction, destination) = line.split_once(" -> ").unwrap();
        task.gates.insert(String::from(destination), Gate::from(instruction));
    }

    let a_value = task.get_wire_value(String::from("a"));
    println!("Part 1: {}", a_value);

    task.gates.entry(String::from("b")).and_modify(|g| {
        g.in1 = format!("{}", a_value);
        g.in2 = String::new();
        g.op = String::from("IS");
    });
    task.cache.clear();
    let new_a_value = task.get_wire_value(String::from("a"));
    println!("Part 2: {}", new_a_value);
}