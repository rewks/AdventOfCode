use itertools::Itertools;

struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl From<&str> for Ingredient {
    fn from(data: &str) -> Ingredient {
        let tokens: Vec<&str> = data.split(" ").collect();
        Ingredient {
            capacity: tokens[2].trim_end_matches(",").parse().unwrap(),
            durability: tokens[4].trim_end_matches(",").parse().unwrap(),
            flavor: tokens[6].trim_end_matches(",").parse().unwrap(),
            texture: tokens[8].trim_end_matches(",").parse().unwrap(),
            calories: tokens[10].trim_end_matches(",").parse().unwrap(),
        }
    }
}

fn all_combinations(n: usize) -> impl Iterator<Item = Vec<i32>> {
    itertools::repeat_n(0..=100, n)
        .multi_cartesian_product()
        .filter(|v| v.iter().sum::<i32>() == 100)
}

fn do_chal(ingredients: &Vec<Ingredient>) -> (i32, i32) {
    let mut p1_high_score: i32 = i32::MIN;
    let mut p2_high_score: i32 = i32::MIN;

    for quantities in all_combinations(4) {
        let mut capacity = (quantities[0] * ingredients[0].capacity)
            + (quantities[1] * ingredients[1].capacity)
            + (quantities[2] * ingredients[2].capacity)
            + (quantities[3] * ingredients[3].capacity);
        let mut durability = (quantities[0] * ingredients[0].durability)
            + (quantities[1] * ingredients[1].durability)
            + (quantities[2] * ingredients[2].durability)
            + (quantities[3] * ingredients[3].durability);
        let mut flavor = (quantities[0] * ingredients[0].flavor)
            + (quantities[1] * ingredients[1].flavor)
            + (quantities[2] * ingredients[2].flavor)
            + (quantities[3] * ingredients[3].flavor);
        let mut texture = (quantities[0] * ingredients[0].texture)
            + (quantities[1] * ingredients[1].texture)
            + (quantities[2] * ingredients[2].texture)
            + (quantities[3] * ingredients[3].texture);

        let calories = (quantities[0] * ingredients[0].calories)
            + (quantities[1] * ingredients[1].calories)
            + (quantities[2] * ingredients[2].calories)
            + (quantities[3] * ingredients[3].calories);

        // Rule: any negatives become 0
        capacity = capacity.max(0);
        durability = durability.max(0);
        flavor = flavor.max(0);
        texture = texture.max(0);

        // Calc score for recipe and if better than previous best, set new score
        let score = capacity * durability * flavor * texture;
        p1_high_score = p1_high_score.max(score);

        if calories == 500 {
            p2_high_score = p2_high_score.max(score);
        }
    }
    (p1_high_score, p2_high_score)
}

fn main() {
    let data = include_str!("../input.txt");
    let mut ings: Vec<Ingredient> = Vec::new(); //HashSet<Ingredient> = HashSet::new();

    for l in data.lines() {
        ings.push(Ingredient::from(l));
    }

    let (part1, part2) = do_chal(&ings);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
