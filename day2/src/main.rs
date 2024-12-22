// Notes:
// - Couldn't use CSV Reader this time because there are a variable number of levels on each line.

use std::fs;

fn part1() {
    let contents = fs::read_to_string("./src/input.csv").expect("Couldn't read file.");
    let records: Vec<&str> = contents.split("\n").collect();
    let mut num_safe = 0;
    for record in records {
        let mut is_safe = 1;
        let levels: Vec<i32> = record.split(" ").map(|c| c.parse::<i32>().unwrap() ).collect();
        let mut difference_sum = 0; 
        for i in 1..levels.len() {
            let difference = levels[i] - levels[i-1];
            let new_difference_sum = difference_sum + difference;
            if new_difference_sum.abs() - difference_sum.abs() < 1 || new_difference_sum.abs() - difference_sum.abs() > 3 {
                is_safe = 0;
                break;
            }
            difference_sum = new_difference_sum;
        }
        num_safe += is_safe;
    }
    println!("Part 1: Num safe: {}", num_safe);
}

fn part2() {
    // TO DO
}

fn main() {
    part1();
    part2();
}
