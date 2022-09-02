use fancy_regex::Regex;

fn is_string_nice(s: &str) -> bool {
    let re_unwanted = Regex::new(r"ab|cd|pq|xy").unwrap();
    if re_unwanted.is_match(&s).unwrap() {
        return false;
    }

    let re_vowels = Regex::new(r"[aeiou]").unwrap();
    let vowel_count = re_vowels.find_iter(&s).count();

    let re_repeat = Regex::new(r"(\w){1}\1").unwrap();
    let repeated_letter = re_repeat.is_match(&s).unwrap();

    vowel_count >= 3 && repeated_letter
}

fn is_string_nice2(s: &str) -> bool {
    let re_pairs = Regex::new(r"(\w\w)\w*\1").unwrap();
    let re_repeat_letter = Regex::new(r"(\w)\w{1}\1").unwrap();
    re_pairs.is_match(&s).unwrap() && re_repeat_letter.is_match(&s).unwrap()
}

fn main() {
    let data = include_str!("../input.txt");
    let mut nice_strings = 0;
    let mut nice_strings2 = 0;
     for l in data.lines() {
        if is_string_nice(&l) {
            nice_strings += 1;
        }
        if is_string_nice2(&l) {
            nice_strings2 += 1;
        }
    }
    println!("Part 1: {}", nice_strings);
    println!("Part 2: {}", nice_strings2);
}