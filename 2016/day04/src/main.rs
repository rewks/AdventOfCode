use std::collections::HashMap;
use regex::Regex;

struct Room {
    name: String,
    sector_id: u32,
    checksum: String,
}

impl Room {
    fn new(data: &str) -> Self {
        let re_checksum = Regex::new(r"\[[a-z]+\]$").unwrap();
        let re_sector_id = Regex::new(r"\d+").unwrap();

        let (checksum_s, checksum_e) = match re_checksum.find(data) {
            Some(m) => (m.start() + 1, m.end() - 1),
            None => panic!("No checksum found in data '{}'", data),
        };
        let (id_s, id_e) = match re_sector_id.find(data) {
            Some(m) => (m.start(), m.end()),
            None => panic!("No sector id found in data '{}'", data),
        };

        let checksum = &data[checksum_s..checksum_e];
        let sector_id = &data[id_s..id_e];
        let letters = &data[..id_s - 1];

        Self {
            name: letters.to_string(),
            sector_id: sector_id.parse::<u32>().unwrap(),
            checksum: checksum.to_string()
        }
    }

    fn is_real(&self) -> bool {
        let mut name = HashMap::new();
        for c in self.name.chars() {
            if c == '-' {
                continue;
            }
            name.entry(c).and_modify(|x| *x += 1).or_insert(1);
        }

        let mut prev_quantity = usize::MAX;
        let mut prev_letter = 'Z';
    
        for c in self.checksum.chars() {
            match name.get(&c) {
                Some(n) => {
                    if *n < prev_quantity {
                        prev_quantity = *n;
                        prev_letter = c;
                        continue;
                    } else if *n == prev_quantity {
                        if c > prev_letter {
                            prev_quantity = *n;
                            prev_letter = c;
                            continue;
                        }
                    }
                    return false;
                },
                None => {
                    return false;
                }
            };
        }
    
        true
    }

    fn get_sector_id(&self) -> u32 {
        self.sector_id
    }

    fn get_decrypted_name(&self) -> String{
        let rot = (self.sector_id % 26) as u8;
        let mut decrypted = String::new();

        for b in self.name.bytes() {
            if b == 0x2D {
                decrypted.push(' ');
                continue;
            }
            let mut nb = b + rot;
            if nb > 0x7A {
                nb = 0x60 + (nb - 0x7A);
            }
            decrypted.push(nb as char);
        }

        decrypted
    }
}

fn main() {
    let data = include_str!("../input.txt");
    let mut p1_sector_sum = 0;

    for line in data.lines() {
        let room = Room::new(line);
        if room.is_real() {
            p1_sector_sum += room.get_sector_id();
            println!("{} : {}", room.get_decrypted_name(), room.get_sector_id()); // just grep for answer to p2... grep north
        }
    }

    println!("Part 1: {}", p1_sector_sum);
}