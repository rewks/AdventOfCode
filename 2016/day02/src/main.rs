struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn move_pos1(&mut self, instructions: &str) {
        for c in instructions.chars() {
            match c {
                'U' => self.y = 0.max(self.y - 1),
                'D' => self.y = 2.min(self.y + 1),
                'L' => self.x = 0.max(self.x - 1),
                'R' => self.x = 2.min(self.x + 1),
                _ => panic!("Unexpected char '{}' in input!", c)
            };
        }
    }

    fn get_key1(&self) -> i32 {
        self.y * 3 + self.x + 1
    }

    fn move_pos2(&mut self, instructions: &str) {
        for c in instructions.chars() {
            let row_width = 2 - (self.y - 2).max(2 - self.y);
            let col_height = 2 - (self.x - 2).max(2 - self.x);
            match c {
                'U' => self.y = (self.y - 1).max(2 - col_height),
                'D' => self.y = (self.y + 1).min(2 + col_height),
                'L' => self.x = (self.x - 1).max(2 - row_width),
                'R' => self.x = (self.x + 1).min(2 + row_width),
                _ => panic!("Unexpected char '{}' in input!", c)
            };
        }
    }

    fn get_key2(&self) -> char {
        match (self.x, self.y) {
            (2, 0) => '1',
            (1, 1) => '2',
            (2, 1) => '3',
            (3, 1) => '4',
            (0, 2) => '5',
            (1, 2) => '6',
            (2, 2) => '7',
            (3, 2) => '8',
            (4, 2) => '9',
            (1, 3) => 'A',
            (2, 3) => 'B',
            (3, 3) => 'C',
            (2, 4) => 'D',
            _ => panic!("Unexpected position")
        }
    }
}

fn main() {
    let data = include_str!("../input.txt").trim_end();
    let mut pos1 = Pos { x: 1, y: 1};
    let mut code1 = String::new();
    let mut pos2 = Pos { x: 0, y: 2 };
    let mut code2 = String::new();

    for line in data.lines() {
        pos1.move_pos1(line);
        code1.push_str(&pos1.get_key1().to_string());
        pos2.move_pos2(line);
        code2.push(pos2.get_key2());
    }

    println!("Part 1: {}", code1);
    println!("Part 2: {}", code2);
}
