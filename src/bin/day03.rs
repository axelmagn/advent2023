use std::io::stdin;

use once_cell::sync::Lazy;
use regex::Regex;

static NUM_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());
static SYM_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^a-zA-Z0-9.]").unwrap());

pub fn main() {
    let lines: Vec<String> = stdin().lines().map(|line_res| line_res.unwrap()).collect();
    println!("{}", add_part_nums(&lines));
}

fn add_part_nums(lines: &[String]) -> u32 {
    let mut sum: u32 = 0;
    for line_i in 0..lines.len() {
        let line = &lines[line_i];
        for num_m in NUM_RE.find_iter(line) {
            if has_adjacent_symbol(&lines, line_i, num_m.start() as i32, num_m.end() as i32) {
                let num: u32 = num_m.as_str().parse().unwrap();
                sum += num;
            }
        }
    }
    sum
}

fn has_adjacent_symbol(lines: &[String], line_i: usize, start: i32, end: i32) -> bool {
    if start > 0 && SYM_RE.is_match(&lines[line_i][(start - 1) as usize..start as usize])
        || end < lines[line_i].len() as i32
            && SYM_RE.is_match(&lines[line_i][end as usize..(end + 1) as usize])
        || line_i > 0 && has_symbol(&lines[line_i - 1], start - 1, end + 1)
        || line_i + 1 < lines.len() && has_symbol(&lines[line_i + 1], start - 1, end + 1)
    {
        true
    } else {
        false
    }
}

fn has_symbol(line: &str, start: i32, end: i32) -> bool {
    let start: usize = start.max(0) as usize;
    let end: usize = end.min(line.len() as i32) as usize;
    let term = &line[start..end];
    SYM_RE.is_match(term)
}

#[test]
fn test_has_symbol() {
    assert_eq!(has_symbol("...+", 0, 4), true);
    assert_eq!(has_symbol("....", 0, 4), false);
}

/// Get all numbers that are adjacent to cell (i, j).
fn adjacent_nums(lines: &[String], idx: (usize, usize)) -> Vec<u32> {
    // check top left corner
    todo!();
}
