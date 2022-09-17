#[derive(Debug)]
struct LightGrid {
    lights: [[bool; 100]; 100],
}

impl From<&str> for LightGrid {
    fn from(data: &str) -> LightGrid {
        let mut lg: LightGrid = LightGrid {
            lights: [[false; 100]; 100]
        };
        for (y, line) in data.lines().enumerate() {
            for (x, state) in line.trim().chars().enumerate() {
                lg.lights[y][x] = match state {
                    '#' => true,
                    '.' => false,
                    _ => panic!("Unexpected input '{}'", state)
                };
            }
        }
        lg
    }
}

impl LightGrid {
    fn count_on(self) -> u32 {
        let mut n = 0;
        for column in self.lights {
            for l in column {
                if l {
                    n += 1;
                }
            }
        }
        n
    }

    fn update_state(&mut self) {
        let prev_state = self.lights.clone();
        let size = self.lights[0].len();
        for x in 0..size {
            for y in 0..size {
                let mut lit_neighbours = 0;
                // Check neighbours NW, W and SW
                if x > 0 {
                    if y > 0 {
                        lit_neighbours += prev_state[x - 1][y - 1] as u32;
                    }
                    lit_neighbours += prev_state[x - 1][y] as u32;
                    if y < size - 1 {
                        lit_neighbours += prev_state[x - 1][y + 1] as u32;
                    }
                }

                // Check neighbours N and S
                if y > 0 {
                    lit_neighbours += prev_state[x][y - 1] as u32;
                }
                if y < size - 1 {
                    lit_neighbours += prev_state[x][y + 1] as u32;
                }

                // Check neighbours NE, E and SE
                if x < size - 1 {
                    if y > 0 {
                        lit_neighbours += prev_state[x + 1][y - 1] as u32;
                    }
                    lit_neighbours += prev_state[x + 1][y] as u32;
                    if y < size - 1 {
                        lit_neighbours += prev_state[x + 1][y + 1] as u32;
                    }
                }

                // Update light state based on number of lit neighbours
                self.lights[x][y] = match prev_state[x][y] {
                    true => match lit_neighbours {
                        2|3 => true,
                        _ => false
                    },
                    false => match lit_neighbours {
                        3 => true,
                        _ => false
                    }
                }
            }
        }

        // Ugly but fast way of implementing part2 constraint (corners always on). Comment out to get part1 answer
        self.lights[0][0] = true;
        self.lights[0][99] = true;
        self.lights[99][0] = true;
        self.lights[99][99] = true;
    }
}

fn main() {
    let data = include_str!("../input.txt");
    let mut l = LightGrid::from(data);

    for _n in 0..100 {
        l.update_state();
    }
    println!("Answer: {}", l.count_on());
}