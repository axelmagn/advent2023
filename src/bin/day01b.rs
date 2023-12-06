use std::io::{self, BufRead};

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    combinator::{map, value},
    multi::fold_many0,
    IResult, Parser,
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum SolutionError {
    NotEnoughDigits,
    IOError,
    ParseError,
}

type Result<T> = std::result::Result<T, SolutionError>;

pub fn main() {
    let stdin = io::stdin().lock();
    let sum = sum_calibration_values(stdin).unwrap();
    println!("{}", sum);
}

fn sum_calibration_values<T: BufRead>(reader: T) -> Result<u32> {
    let mut sum = 0;
    for line in reader.lines() {
        match line {
            Ok(line) => sum += extract_calibration_value(&line)?,
            Err(_) => return Err(SolutionError::IOError),
        }
    }
    Ok(sum)
}

#[test]
fn test_sum_calibration_values() {
    let test_str = r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
    assert_eq!(sum_calibration_values(test_str.as_bytes()), Ok(281));
}

fn extract_calibration_value(line: &str) -> Result<u32> {
    let digits = nums(line)?;
    if digits.len() < 1 {
        return Err(SolutionError::NotEnoughDigits);
    }
    let value = digits.first().unwrap() * 10 + digits.last().unwrap();
    println!("{}: {}", line, value);
    Ok(value)
}

#[test]
fn test_extract_calibration_value() {
    assert_eq!(extract_calibration_value("two1nine"), Ok(29));
    assert_eq!(extract_calibration_value("eightwothree"), Ok(83));
    assert_eq!(extract_calibration_value("abcone2threexyz"), Ok(13));
    assert_eq!(extract_calibration_value("xtwone3four"), Ok(24));
}

// fn one(input: &str) -> IResult<&str, u32> {
//     value(1, alt((tag("one"), tag("1")))).parse(input)
// }

macro_rules! num {
    ($word:ident, $digit:expr) => {
        |input| -> IResult<_, u32> {
            value(
                $digit,
                alt((tag(stringify!($word)), tag(stringify!($digit)))),
            )
            .parse(input)
        }
    };
}

#[test]
fn test_num() {
    let one = num!(one, 1);
    assert_eq!(one("one"), Ok(("", 1)));
    assert_eq!(one("1"), Ok(("", 1)));
    match one("abc") {
        Ok(_) => unreachable!(),
        Err(_) => {}
    }
}

fn any_num(input: &str) -> IResult<&str, u32> {
    alt((
        num!(zero, 0),
        num!(one, 1),
        num!(two, 2),
        num!(three, 3),
        num!(four, 4),
        num!(five, 5),
        num!(six, 6),
        num!(seven, 7),
        num!(eight, 8),
        num!(nine, 9),
    ))
    .parse(input)
}

#[test]
fn test_any_num() {
    assert_eq!(any_num("one"), Ok(("", 1)));
    assert_eq!(any_num("0"), Ok(("", 0)));
    assert_eq!(any_num("six"), Ok(("", 6)));
    assert_eq!(any_num("9"), Ok(("", 9)));
}

fn take_num(input: &str) -> IResult<&str, Option<u32>> {
    alt((map(any_num, |n| Some(n)), value(None, take(1u8)))).parse(input)
}

#[test]
fn test_take_num() {
    assert_eq!(take_num("six"), Ok(("", Some(6))));
    assert_eq!(take_num("abc"), Ok(("bc", None)));
}

fn nums(input: &str) -> Result<Vec<u32>> {
    let parse_res = fold_many0(take_num, Vec::new, |mut acc: Vec<_>, item_opt| {
        if let Some(item) = item_opt {
            acc.push(item);
        }
        acc
    })(input);
    match parse_res {
        Ok((_, res)) => Ok(res),
        Err(_) => Err(SolutionError::ParseError),
    }
}

#[test]
fn test_nums() {
    assert_eq!(nums("eightwothree"), Ok(vec![8, 3]));
    assert_eq!(nums("abcone2threexyz"), Ok(vec![1, 2, 3]));
}
