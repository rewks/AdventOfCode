use regex::Regex;

fn decompress_str(s: &str) -> String {
    let re = Regex::new(r"\(\d+x\d+\)").unwrap();

    let mut decompressed = String::new();
    let mut matches = re.find_iter(s);
    let mut prev_seq_end = usize::MIN;
    while let Some(marker) = matches.next() {
        let start = marker.start();
        // If current match starts within the previously decompressed sequence, treat as plain text
        if start + 1 < prev_seq_end {
            continue;
        }
        let end = marker.end();
        let marker_str = &s[start..end];
        let marker_tokens: Vec<&str> = marker_str.split("x").collect();
        let sequence_len = marker_tokens
            .get(0)
            .unwrap()
            .trim_start_matches("(")
            .parse::<usize>()
            .unwrap();
        let repetitions = marker_tokens
            .get(1)
            .unwrap()
            .trim_end_matches(")")
            .parse::<usize>()
            .unwrap();

        decompressed.push_str(&s[prev_seq_end..start]);
        let sequence = &s[end..end + sequence_len];
        for _i in 0..repetitions {
            decompressed.push_str(sequence);
        }

        prev_seq_end = end + sequence_len;
    }
    decompressed.push_str(&s[prev_seq_end..s.len()]);

    decompressed
}

fn decompress_v2(s: &str) -> usize {
    let re = Regex::new(r"\(\d+x\d+\)").unwrap();
    let mut weights = vec![1; s.len()];

    let mut matches = re.find_iter(s);
    while let Some(marker) = matches.next() {
        let start = marker.start();
        let end = marker.end();
        let marker_str = &s[start..end];
        let marker_tokens: Vec<&str> = marker_str.split("x").collect();
        let sequence_len = marker_tokens
            .get(0)
            .unwrap()
            .trim_start_matches("(")
            .parse::<usize>()
            .unwrap();
        let repetitions = marker_tokens
            .get(1)
            .unwrap()
            .trim_end_matches(")")
            .parse::<usize>()
            .unwrap();

        for i in end..end + sequence_len {
            weights[i] *= repetitions;
        }

        for i in start..end {
            weights[i] = 0;
        }
    }

    weights.iter().sum::<usize>()
}

fn main() {
    let data = include_str!("../input.txt").trim();

    assert_eq!(decompress_str("ADVENT").len(), 6);
    assert_eq!(decompress_str("A(1x5)BC").len(), 7);
    assert_eq!(decompress_str("(3x3)XYZ").len(), 9);
    assert_eq!(decompress_str("(6x1)(1x3)A").len(), 6);
    assert_eq!(decompress_str("X(8x2)(3x3)ABCY").len(), 18);
    assert_eq!(decompress_str("(4x2)XFDV(3x3)ABC").len(), 17);

    assert_eq!(decompress_v2("(3x3)XYZ"), 9);
    assert_eq!(decompress_v2("X(8x2)(3x3)ABCY"), 20);
    assert_eq!(decompress_v2("(27x12)(20x12)(13x14)(7x10)(1x12)A"), 241920);
    assert_eq!(
        decompress_v2("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"),
        445
    );

    println!("Part1: {}", decompress_str(data).len());
    println!("Part2: {}", decompress_v2(data));
}
