use std::{collections::HashSet, collections::HashMap, fs};

const _RAW_INP1: &str = r"abc

a
b
c

ab
ac

a
a
a
a

b";

fn find_sum(group_str : &str) -> usize{
    let mut unique_ans = HashSet::new();

    for s in group_str.split('\n') {
        for c in s.chars(){
            unique_ans.insert(c);
        }
    }
    unique_ans.len()
}

// Puzzle 2
// Put everything in a hashmap. If the nr of persons
// is equal to count of an answer, then we found the common 
// answer
fn find_unique_sum(group_str: &str) -> usize {
    let mut unique_ans = HashMap::new();
    let mut everyone_ans = 0;
    let mut total_nr_persons = 0;

    for s in group_str.split('\n') {
        for c in s.chars() {
            let count = unique_ans.entry(c).or_insert(0);
            *count += 1;
        }
        total_nr_persons += 1;
    }
    for (_, val) in unique_ans.iter() {
        if *val == total_nr_persons {
            everyone_ans += 1
        }
    }
    everyone_ans
}

fn main() {
    let contents = fs::read_to_string("src/input.txt").expect("Something went wrong reading the file");
    
    let ans_sum : usize = contents.split("\n\n").map(find_sum).sum();
    let everyone_ans_sum: usize = contents.split("\n\n").map(find_unique_sum).sum();
    
    println!("The answer for puzzle one is: {}", ans_sum);
    println!("The answer for puzzle two is: {}", everyone_ans_sum);
}
