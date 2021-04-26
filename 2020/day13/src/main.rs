use std::fs;
use std::collections::HashMap;

const _RAW_INP1: &str = r"939
7,13,x,x,59,x,31,19";

fn parse_inp(inp: &str) -> (i32, Vec<i32>) {
    let inp_split: Vec<&str> = inp.split('\n').collect();
    let target_val = inp_split[0].parse::<i32>().unwrap();
    // Filter map makes the life a bit easier
    let filter_inp : Vec<i32> = inp_split[1]
        .split(',')
        .into_iter()
        .filter_map(|x| x.parse::<i32>().ok())
        .collect();
    (target_val, filter_inp)
}

fn main() {
    let contents =
        fs::read_to_string("src/input.txt").expect("Something went wrong reading the file");
    // let inp = _RAW_INP1;
    let inp = contents.as_str();
    let mut puzz1_hash: HashMap<i32, i32> = HashMap::new();

    let (target, inp_vec) = parse_inp(inp);

    // Invert the logic. Find the max diff with a modulo. The max diff with modulo will
    // give the lowest diff in the next round of multiplication
    // The max diff with modulo comes for 59 and the modulo value is 54.
    let puz1_max = inp_vec.iter().map(|x| {puzz1_hash.insert(target % x, *x); target % x}).max().unwrap();

    let puz1_max_inp = puzz1_hash.get(&puz1_max).unwrap();

    // The min diff will be the input for which the diff is min - the modulo times the input
    // i.e., for the given example, it is (59 - 54) * 59
    println!("Puzzle 1 ans : {}", (puz1_max_inp - puz1_max) * puz1_max_inp);
}
