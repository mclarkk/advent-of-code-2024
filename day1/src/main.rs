// Reflection: After writing this, probably a faster way to do it would be
// to build the data structures when iterating through the CSV.
// Specifically, you should build a min heap for left list, a min heap
// for right list, and then a HashMap with element counts for right list.
// You don't even need a list or a sorted list at all, just min heap.
// Then it should be EVEN FASTER (though the time for this less optimal
// solution is already like six times faster than the heap solution with 
// Python with Pandas).

use csv::ReaderBuilder;
use std::collections::HashMap;

// Reflection: My first time using the csv crate.
// I'm so surprised there doesn't seem to be a built-in way pull columns out.
// Also, the delimiter can only be a single character?? WTH! Doesn't work
// for this file, which has a three space delimiter. I have to manually split each row!
// Ridiculous, almost no reason to use a CSV reader at that point.
// Also, I hoisted myself with an off-by-one error because the default Reader
// has headers on by default and skipped the first line. You have to use a ReaderBuilder
// and configure the reader to NOT use headers if you don't want that.

// get the two lists out of the csv
fn get_lists() -> (Vec<i32>, Vec<i32>) {
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .from_path("src/input.csv")
        .unwrap();
    let mut list1 = Vec::<i32>::new();
    let mut list2 = Vec::<i32>::new();
    for row in reader.records()  {
        match row {
            Ok(record) => {
                if let Some(string_record) = record.get(0) {
                    let s: Vec<&str> = string_record.split("   ").collect();
                    if let Some(num_str) = s.first() {
                        list1.push(num_str.parse().unwrap());
                    } else {
                        println!("First element is None: {:?}", s)
                    }
                    if let Some(num_str) = s.get(1) {
                        list2.push(num_str.parse().unwrap());
                    } else {
                        println!("Second element is None: {:?}", s)
                    }
                } else {
                    println!("Couldn't get the string out of record {:?}", record);
                }
            }
            Err(err) => println!("{:?}", err)
        }
    }
    (list1, list2)
}

fn main() {
    let start = std::time::Instant::now();
    let (mut list1, mut list2) = get_lists();
    if list1.len() == list2.len() {
        list1.sort();
        list2.sort();

        // Part 1
        let distances: Vec<i32> = list1.clone()
            .iter()
            .zip(list2.clone())
            .map(|(e1, e2)| (e1-e2).abs())
            .collect();
        let total_distance: i32 = distances.iter().sum();
        println!("Total Distance: {:?}", total_distance);

        // Part 2
        let mut num_occurrences= HashMap::<i32, i32>::new();
        let _: Vec<_> = list2
            .iter()
            .map(|e| {
                if let Some(val) = num_occurrences.get(e) { 
                    num_occurrences.insert(*e, val+1)
                } else {
                    num_occurrences.insert(*e, 1)
                }
            })
            .collect();

        let similarities: Vec<i32> = list1
            .iter()
            .map(|e| {
                if let Some(count) = num_occurrences.get(e) {
                    e * count
                } else {
                    0
                }
            })
            .collect();
        let total_similarity: i32 = similarities.iter().sum();
        println!("Total Similarity: {:?}", total_similarity)

    } else {
        println!("Lists are not the same length, cannot calculate pairwise distances.")
    }
    eprintln!("{:.2?}", start.elapsed());
}
