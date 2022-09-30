use std::io::{self, Write};

#[derive(Debug)]
struct PixelGrid {
    grid: [[bool; 6]; 50],
}

impl PixelGrid {
    fn new() -> Self {
        Self {
            grid: [[false; 6]; 50],
        }
    }

    fn light_rect(&mut self, dimensions: &str) {
        let t: Vec<&str> = dimensions.split("x").collect();
        let width: usize = t.get(0).unwrap().parse().unwrap();
        let height: usize = t.get(1).unwrap().parse().unwrap();

        for x in 0..width.min(self.grid.len()) {
            for y in 0..height.min(self.grid[0].len()) {
                self.grid[x][y] = true;
            }
        }
    }

    fn rotate_row(&mut self, row: usize, amount: usize) {
        let mut new_row = [false; 50];
        for x in 0..self.grid.len() {
            let nx = (x + amount) % self.grid.len();
            new_row[nx] = self.grid[x][row];
        }
        for x in 0..self.grid.len() {
            self.grid[x][row] = new_row[x];
        }
    }

    fn rotate_column(&mut self, col: usize, amount: usize) {
        let mut new_col = [false; 6];
        for y in 0..self.grid[0].len() {
            let ny = (y + amount) % self.grid[0].len();
            new_col[ny] = self.grid[col][y];
        }
        self.grid[col] = new_col;
    }

    fn print_grid(&self) {
        for y in 0..self.grid[0].len() {
            for x in 0..self.grid.len() {
                match self.grid[x][y] {
                    true => print!("#"),
                    false => print!("."),
                };
            }
            print!("\n");
            io::stdout().flush().unwrap();
        }
    }

    fn count_on(&self) -> u32 {
        let mut n = 0;
        for x in 0..self.grid.len() {
            for y in 0..self.grid[0].len() {
                if self.grid[x][y] {
                    n += 1;
                }
            }
        }
        n
    }
}

fn main() {
    let data = include_str!("../input.txt");
    let mut pixel_grid = PixelGrid::new();

    for line in data.lines() {
        let tokens: Vec<&str> = line.split(" ").collect();
        if tokens.len() == 2 {
            pixel_grid.light_rect(tokens.get(1).unwrap());
        } else {
            let row_col: usize = tokens
                .get(2)
                .unwrap()
                .split("=")
                .collect::<Vec<&str>>()
                .get(1)
                .unwrap()
                .parse()
                .unwrap();
            let amount: usize = tokens.get(4).unwrap().parse().unwrap();
            match *tokens.get(1).unwrap() {
                "row" => pixel_grid.rotate_row(row_col, amount),
                "column" => pixel_grid.rotate_column(row_col, amount),
                _ => continue,
            };
        }
    }

    println!("Part 1: {}", pixel_grid.count_on());
    println!("Part 2:");
    pixel_grid.print_grid();
}
