#[derive(Debug)]
struct Elf {
    total_calories: u32,
    meals: Vec<u32>,
}

impl Elf {
    fn new(meals: &Vec<u32>) -> Self {
        let mut total_calories = 0;
        for meal in meals {
            total_calories += meal;
        }

        Self {
            total_calories,
            meals: meals.to_vec()
        }
    }

    fn get_total_calories(&self) -> u32 {
        self.total_calories
    }
}

fn get_most_cals(elves: &Vec<Elf>) -> u32 {
    let mut most_calories = 0;
    for elf in elves {
        most_calories = most_calories.max(elf.get_total_calories());
    }

    most_calories
}

fn get_three_most_cals(elves: &Vec<Elf>) -> u32 {
    let mut calorie_totals: Vec<u32> = Vec::new();

    for elf in elves {
        calorie_totals.push(elf.get_total_calories());
    }

    calorie_totals.sort();

    let mut three_combined_most = 0;
    for _ in 0..3 {
        three_combined_most += calorie_totals.pop().unwrap();
    }

    three_combined_most
}

fn main() {
    let data = include_str!("../input.txt");
    let mut elves: Vec<Elf> = Vec::new();

    let mut meals: Vec<u32> = Vec::new();
    for l in data.lines() {
        if l.len() == 0 {
            let elf = Elf::new(&meals);
            elves.push(elf);
            meals.clear();
        } else {
            let meal_calories: u32 = l.parse().unwrap();
            meals.push(meal_calories);
        }
        
    }
    let elf = Elf::new(&meals);
    elves.push(elf);

    println!("{}", get_most_cals(&elves));
    println!("{}", get_three_most_cals(&elves));
}
