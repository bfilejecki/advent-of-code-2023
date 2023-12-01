use regex::Regex;
use std::io::BufRead;
use std::{fs::File, io::BufReader};

fn main() {
    println!("Part one result is {}", part_one());
    println!("Part two result is {}", part_two());
}

fn part_one() -> u32 {
    let input_file = File::open("./input1.txt").expect("File should exist");
    let reader = BufReader::new(input_file);

    let mut sum = 0;
    for line in reader.lines() {
        if let Ok(word) = line {
            for c in word.chars() {
                if c.is_digit(10) {
                    sum = sum
                        + (c.to_digit(10)
                            .expect("This should always be valid decimal digit")
                            * 10);
                    break;
                }
            }
            for c in word.chars().rev() {
                if c.is_digit(10) {
                    sum = sum
                        + c.to_digit(10)
                            .expect("This should always be valid decimal digit");
                    break;
                }
            }
        }
    }
    return sum;
}

fn part_two() -> u32 {
    let input_file = File::open("./input1.txt").expect("File should exist");
    let reader = BufReader::new(input_file);

    let mut sum = 0;
    let first_digit_pattern =
        Regex::new(r"\d{1}|one|two|three|four|five|six|seven|eight|nine|zero").unwrap();
    let last_digit_pattern =
        Regex::new(r"(.*)(\d{1}|one|two|three|four|five|six|seven|eight|nine|zero)").unwrap();
    for line in reader.lines() {
        if let Ok(word) = line {
            // println!("Parsing word: {}", word.as_str());
            if let Some(hit) = first_digit_pattern.find(&word) {
                // println!("First digit: {}", hit.as_str());
                let first_digit = match hit.as_str() {
                    "zero" => 0,
                    "one" => 1,
                    "two" => 2,
                    "three" => 3,
                    "four" => 4,
                    "five" => 5,
                    "six" => 6,
                    "seven" => 7,
                    "eight" => 8,
                    "nine" => 9,
                    other => other.parse().unwrap(),
                };
                sum = sum + first_digit * 10;
            }
            if let Some(caps) = last_digit_pattern.captures(&word) {
                // println!("Last digit: {}", caps.get(2).unwrap().as_str());
                let last_digit = match caps.get(2).unwrap().as_str() {
                    "zero" => 0,
                    "one" => 1,
                    "two" => 2,
                    "three" => 3,
                    "four" => 4,
                    "five" => 5,
                    "six" => 6,
                    "seven" => 7,
                    "eight" => 8,
                    "nine" => 9,
                    other => other.parse().unwrap(),
                };
                sum = sum + last_digit;
            }
            // println!("Current sum: {}", sum);
        }
    }
    return sum;
}
