
fn get_n(x: u64, y: u64) -> u64 {
    (0..=(x + y - 1)).sum::<u64>() - (y - 1)
}

fn get_code(previous_code: u64) -> u64 {
    (previous_code * 252533) % 33554393
}

fn main() {
    let data = include_str!("../input.txt");
    let tokens: Vec<&str> = data.split(" ").collect();
    let y: u64 = tokens[16].trim_end_matches(",").parse().unwrap();
    let x: u64 = tokens[18].trim_end_matches(".\n").parse().unwrap();

    let n = get_n(x, y);
    let mut code = 20151125;
    for _i in 2..=n {
        code = get_code(code);
    }

    println!("Secret Code: {}", code);
}