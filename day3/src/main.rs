use std::fs;
use regex::Regex;

fn part1() {
    let source = fs::read_to_string("./src/input.txt").unwrap();
    let re = Regex::new(r#"mul\([0-9]{1,3},[0-9]{1,3}\)"#).unwrap();
    let mut sum = 0;
    for matching in re.find_iter(&source) {
        let digit_re = Regex::new(r#"[0-9]+"#).unwrap();
        let digits: Vec<i32> = digit_re.find_iter(matching.as_str()).map(|d| d.as_str().parse::<i32>().unwrap() ).collect();
        sum += digits[0] * digits[1];
    }
    println!("Sum: {}", sum);
}

fn main() {
    part1();
}