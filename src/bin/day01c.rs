use std::{collections::HashMap, io};

pub fn main() {
    let stdin = io::stdin();
    let mut num_dict: HashMap<&str, u32> = HashMap::new();
    num_dict.insert("one", 1);
    num_dict.insert("two", 2);
    num_dict.insert("three", 3);
    num_dict.insert("four", 4);
    num_dict.insert("five", 5);
    num_dict.insert("six", 6);
    num_dict.insert("seven", 7);
    num_dict.insert("eight", 8);
    num_dict.insert("nine", 9);
    num_dict.insert("zero", 0);

    let mut sum = 0;

    for line in stdin.lines() {
        let line = line.unwrap();
        let mut nums: Vec<u32> = Vec::new();
        // println!("START: {}", line);
        for i in 0..line.len() {
            // first try parsing line[i] as a digit
            let digit_opt = line.chars().nth(i).unwrap().to_digit(10);
            match digit_opt {
                Some(d) => {
                    // println!("\t\tDIGIT: {}", d);
                    nums.push(d);
                    continue;
                }
                None => {}
            }
            for (key, value) in num_dict.iter() {
                let end = i + key.len();
                if end > line.len() {
                    continue;
                }
                let hay = &line[i..end];
                // println!("\t{}:{}", key, hay);
                if hay == *key {
                    nums.push(*value);
                    // println!("\t\t{}", value);
                }
            }
        }
        let calibration_value = nums.first().unwrap() * 10 + nums.last().unwrap();
        // println!("{}: {}", line, calibration_value);
        sum += calibration_value;
    }

    println!("{}", sum);
}
