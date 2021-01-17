use std::{collections::HashSet, fs};

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

fn main() {
    let contents = fs::read_to_string("src/input.txt").expect("Something went wrong reading the file");
    let ans_sum : usize = contents.split("\n\n").map(find_sum).sum();
    println!("The answer for puzzle one is: {}", ans_sum);
}
