use std::collections::HashMap;
use std::fs;

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

fn print2sum(inp_vec: &Vec<u32>) {
    let mut expense_map: HashMap<u32, usize> = HashMap::new();
    for (i, x) in inp_vec.iter().enumerate() {
        expense_map.insert(*x, i);
        if expense_map.contains_key(&(2020 - *x)) {
            println!("{}", *x * (2020 - *x));
        }
    }
}

fn print3sum(inp_vec: &Vec<u32>) {
    let mut expense_map: HashMap<u32, (usize, usize)> = HashMap::new();
    for i in 0..inp_vec.len() {
        for j in i..inp_vec.len() {
            expense_map.insert(inp_vec[i] + inp_vec[j], (i, j));
        }
    }
    for x in inp_vec.iter() {
        if expense_map.contains_key(&(2020 - x)) {
            let (y, z) = expense_map[&(2020 - x)];
            println!("{}", x * inp_vec[y] * inp_vec[z]);
        }
    }
}

fn main() {
    let contents = fs::read_to_string("src/input.txt").expect("Something went wrong reading the file");
    let inp_vec: Vec<u32> = contents
        .split('\n')
        .filter(|x| *x != "")
        .map(|s| remove_whitespace(&s).parse::<u32>().unwrap())
        .collect();
    print2sum(&inp_vec);
    print3sum(&inp_vec);
}
