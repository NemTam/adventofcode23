use std::collections::HashMap;
use std::fs;
use regex::Regex;

const RADIX: u32 = 10;

use num_traits::FromPrimitive;

fn main() {
    second();
}

const RE1: &str = "(?:one|two|three|four|five|six|seven|eight|nine|1|2|3|4|5|6|7|8|9)";
const RE2: &str = "(?:eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|1|2|3|4|5|6|7|8|9)";

fn second() {
    let map = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9)
    ]);

    let data = fs::read_to_string("input/1_a.txt").expect("Unable to read file");
    let vec_str: Vec<&str> = data.split("\n").collect();
    let re1 = Regex::new(RE1).unwrap();
    let re2 = Regex::new(RE2).unwrap();
    let mut res = 0;
    for elem in vec_str {
        let first_match = re1.find(elem).unwrap().as_str();
        let rev: String = elem.chars().rev().collect();
        let last_match = re2.find(&rev).unwrap().as_str();
        let last_match: String = last_match.chars().rev().collect();
        res += map.get(first_match).unwrap() * 10 + map.get(&*last_match).unwrap();
        println!("elem: {} \nfirst match: {:?} \nlast match: {:?} \nsum:{}", elem, first_match, last_match, res);
    }
}

fn first() {
    let data = fs::read_to_string("input/1_a.txt").expect("Unable to read file");
    let vec_str: Vec<&str> = data.split("\n").collect();
    let mut res = 0;
    for elem in vec_str {
        let mut first = 0;
        let mut last = 0;
        for c in elem.chars() {
            if c.is_numeric() {
                first = c.to_digit(RADIX).unwrap();
                break;
            }
        }
        for c in elem.chars().rev() {
            if c.is_numeric() {
                last = c.to_digit(RADIX).unwrap();
                break;
            }
        }
        res += first * 10 + last
    }
    println!("Result: {}", res)
}