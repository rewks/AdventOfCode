use fancy_regex::Regex;

fn part1(s: &str) -> i32 {
    let pattern = Regex::new(r#"(\\\\)|(\\")|(\\x[a-f0-9]{2})"#).unwrap();
    let mem_s = pattern.replace_all(s, "-").to_string();
    let memory: i32 = mem_s.len().try_into().unwrap();
    memory - 2
}

fn part2(s: &str) -> i32 {
    let mut text = s.to_string();
    let p1 = Regex::new(r"\\").unwrap();
    text = p1.replace_all(&text, r"\\").to_string();
    let p2 = Regex::new(r#"""#).unwrap();
    text = p2.replace_all(&text, r#"\""#).to_string();

    let new_len: i32 = text.len().try_into().unwrap();
    new_len + 2
}

fn main() {
    let data = include_str!("../input.txt");
    let mut total_chars_in_code: i32 = 0;
    let mut total_chars_in_memory: i32 = 0;
    let mut total_chars_after_encoding: i32 = 0;

    for line in data.lines() {
        total_chars_in_code += TryInto::<i32>::try_into(line.len()).unwrap();
        total_chars_in_memory += part1(line);
        total_chars_after_encoding += part2(line);
    }

    println!("Part 1: {}", total_chars_in_code - total_chars_in_memory);

    println!("Part 2: {}", total_chars_after_encoding - total_chars_in_code);
}