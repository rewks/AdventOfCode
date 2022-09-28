use md5;
use colored::Colorize;
use std::io::{self, Write};

fn part1(data: &str) -> String {
    let mut password = String::new();
    let mut i: u32 = 0;
    while password.len() < 8 {
        let i_str = i.to_string();
        let s: String = format!("{}{}", data, i_str);
        let digest = md5::compute(s.as_bytes());
        let hash = format!("{:x}", digest);
        if &hash[0..5] == "00000" {
            password.push_str(&hash[5..6]);
        }
        i += 1;
    }
    password
}

fn part2(data: &str) -> String {
    let mut password: [u8; 8] = [b'-'; 8];
    let mut i = 0;
    hacker_print(password);
    while password.contains(&b'-') {
        let s = format!("{}{}", data, i.to_string());
        let hash = format!("{:x}", md5::compute(s));
        if &hash[0..5] == "00000" {
            let j = match &hash[5..6].parse::<usize>() {
                Ok(v) => if *v < 8 {
                    *v
                } else {
                    i += 1;
                    continue
                },
                Err(_) => {
                    i += 1;
                    continue
                }
            };
            if password[j] != b'-' {
                i += 1;
                continue;
            }
            password[j] = *&hash[6..7].as_bytes()[0];
            hacker_print(password);
        }
        i += 1;
    }

    String::from_utf8(password.into_iter().collect()).unwrap()
}

// for fun, because we're in a hacker movie right?
fn hacker_print(pw: [u8; 8]) {
    print!("{esc}c", esc = 27 as char);
    print!("Part 2: ");
    for c in pw {
        if c == b'-' {
            print!("{}", (c as char).to_string().red().bold());
        } else {
            print!("{}", (c as char).to_string().green().bold());
        }
    }
    io::stdout().flush().unwrap();
}

fn main() {
    let data = "abbhdwsy";

    //println!("Part 1: {}", part1(data));
    println!("\nPart 2: {}", part2(data));
}
