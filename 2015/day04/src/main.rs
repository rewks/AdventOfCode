fn find_hash_starts_with(sequence: &str, prefix: &str) -> u32 {
    let mut suffix = 0;

    loop {
        let mut full_string = String::from(prefix);
        full_string.push_str(&suffix.to_string());
        let digest = md5::compute(full_string);
        let hash = format!("{:x}", digest);
        if hash.starts_with(sequence) {
            break;
        }
        suffix += 1;
    }

    suffix
}

fn main() {
    let data = include_str!("../input.txt").trim();
    
    println!("First suffix value that causes hash to begin with five zeroes: {}", find_hash_starts_with("00000", &data));
    println!("First suffix value that causes hash to begin with six zeroes: {}", find_hash_starts_with("000000", &data));
}