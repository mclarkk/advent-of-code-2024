use regex::Regex;
use std::fs;
// Part 2 only
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn part1() {
    let source = fs::read_to_string("./src/input.txt").unwrap();
    let re = Regex::new(r#"mul\([0-9]{1,3},[0-9]{1,3}\)"#).unwrap();
    let mut sum = 0;
    for matching in re.find_iter(&source) {
        let digit_re = Regex::new(r#"[0-9]+"#).unwrap();
        let digits: Vec<i32> = digit_re
            .find_iter(matching.as_str())
            .map(|d| d.as_str().parse::<i32>().unwrap())
            .collect();
        sum += digits[0] * digits[1];
    }
    println!("Sum: {}", sum);
}

fn part2() {
    let source = fs::read_to_string("./src/input.txt").unwrap();
    let mul_re = Regex::new(r#"mul\([0-9]{1,3},[0-9]{1,3}\)"#).unwrap();
    let dont_re = Regex::new(r#"don\'t\(\)"#).unwrap();
    let do_re = Regex::new(r#"do\(\)"#).unwrap();
    // Make the do/don't heap
    let mut heap = BinaryHeap::<(Reverse<usize>, bool)>::new();
    for matching in dont_re.find_iter(&source) {
        heap.push((Reverse(matching.start()), false));
    }
    for matching in do_re.find_iter(&source) {
        heap.push((Reverse(matching.start()), true));
    }
    // Process multiplication instructions
    let mut sum = 0;
    let mut do_mul = true;
    for matching in mul_re.find_iter(&source) {
        // Pop stuff off the heap to see whether we're currently in a Do or Don't section
        // Need to fast-forward heap to where the current mult instruction is
        let mut caught_up = false;
        while !caught_up {
            match heap.peek() {
                None => {
                    caught_up = true;
                }
                Some((Reverse(next_instr_pos), _)) => {
                    if matching.start() > *next_instr_pos {
                        let (_, last_instr) = heap.pop().unwrap();
                        do_mul = last_instr;
                    } else {
                        caught_up = true;
                    }
                }
            }
        }
        if do_mul {
            let digit_re = Regex::new(r#"[0-9]+"#).unwrap();
            let digits: Vec<i32> = digit_re
                .find_iter(matching.as_str())
                .map(|d| d.as_str().parse::<i32>().unwrap())
                .collect();
            sum += digits[0] * digits[1];
        }
    }
    println!("Sum: {}", sum);
}

fn main() {
    part1();
    part2();
}
