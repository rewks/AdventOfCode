use fancy_regex::Regex;

fn supports_tls(s: &str, re_abba: &Regex, re_hyper: &Regex) -> bool {
    let contains_abba = re_abba.is_match(s).unwrap();
    let contains_hyper = re_hyper.is_match(s).unwrap();

    match contains_abba {
        true => !contains_hyper,
        false => contains_abba,
    }
}

fn supports_ssl(s: &str, re_ssl: &Regex) -> bool {
    re_ssl.is_match(s).unwrap()
}

fn main() {
    let data = include_str!("../input.txt");
    let re_abba = Regex::new(r"(\w)(\w(?!\1))\2\1").unwrap();
    let re_hyper = Regex::new(r"(\w)(\w(?!\1))\2\1(?=\w*\])").unwrap();
    let re_ssl = Regex::new(r"(?:\[\w*(\w)(?!\1)(\w)\1\w*\](?:\w*\[\w*\]\w*)*\w*(?=\2\1\2))|(?:(\w)(?!\3)(\w)\3(?!\w*\])(?=[\[\]\w]*\[\w*\4\3\4\w*\]))").unwrap();

    assert!(supports_tls("abba[mnop]qrst", &re_abba, &re_hyper));
    assert!(!supports_tls("abcd[bddb]xyyx", &re_abba, &re_hyper));
    assert!(!supports_tls("aaaa[qwer]tyui", &re_abba, &re_hyper));
    assert!(supports_tls("ioxxoj[asdfgh]zxcvb", &re_abba, &re_hyper));
    assert!(!supports_tls("ioxxoj[asdthhtfgh]zxcvb", &re_abba, &re_hyper));

    assert!(supports_ssl("aba[bab]xyz", &re_ssl));
    assert!(!supports_ssl("xyx[xyx]xyx", &re_ssl));
    assert!(supports_ssl("aaa[kek]eke", &re_ssl));
    assert!(supports_ssl("zazbz[bzb]cdb", &re_ssl));

    let mut ip_tls_count = 0;
    let mut ip_ssl_count = 0;
    for ip in data.lines() {
        if supports_tls(ip, &re_abba, &re_hyper) {
            ip_tls_count += 1;
        }
        if supports_ssl(ip, &re_ssl) {
            ip_ssl_count += 1;
        }

    }
    println!("Part 1: {}", ip_tls_count);
    println!("Part 2: {}", ip_ssl_count);
}
