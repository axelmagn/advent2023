use std::io::stdin;

use regex::Regex;

pub fn main() {
    let mut sum = 0;
    let digit_re = Regex::new(r"\d").unwrap();
    for line in stdin().lines() {
        let line = line.unwrap();
        let digit_matches: Vec<u32> = digit_re
            .find_iter(&line)
            .map(|s| s.as_str().parse().unwrap())
            .collect();
        sum += digit_matches.first().unwrap() * 10 + digit_matches.last().unwrap();
    }
    println!("{}", sum);
}
