fn main() {
    let data = include_str!("../input.txt");
    let mut floor: i32 = 0;
    let mut basement = 0;

    for (p, c) in data.chars().enumerate() {
        if c == '(' {
            floor += 1;
        } else if c == ')' {
            floor -= 1;
        }
        if floor < 0 && basement < 1 {
            basement = p + 1;
        }
    }
    
    println!("Floor: {}", floor);
    println!("Entered basement at position: {}", basement);
}