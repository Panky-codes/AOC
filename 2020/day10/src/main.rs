use std::fs;
const _RAW_INP1: &str = r"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

const _RAW_INP2: &str = r"16
10
15
5
1
11
7
19
6
12
4";

fn puzzle_one(inp_vec: &Vec<u32>) {
    let one_offset = if inp_vec[0] == 1 { 1 } else { 0 };
    let three_offset = if inp_vec[0] == 3 { 1 } else { 0 };

    let val = inp_vec.iter();
    let next_val = inp_vec.iter().skip(1);

    let adj_diff: Vec<u32> = val.zip(next_val).map(|(cur, next)| next - cur).collect();

    let count_one: usize = adj_diff.iter().filter(|x| **x == 1).count() + one_offset; // Plus one for the 0th case
    let count_three: usize = adj_diff.iter().filter(|x| **x == 3).count() + three_offset + 1; // Plus one for last charger

    println!("Puzzle 1 ans is {:?}", count_one * count_three);
}

// Works for the first two sample inputs but it does not 
// scale for the main input
fn puzzle_two(inp_vec: &Vec<u32>, val: u32) -> u32 {
    let mut count: u32 = 0;

    if val == inp_vec[inp_vec.len() - 1] {
        return 1;
    }
    if inp_vec.contains(&(val + 1)) {
        count += puzzle_two(&inp_vec, val + 1);
    }
    if inp_vec.contains(&(val + 3)) {
        count += puzzle_two(&inp_vec, val + 3);
    }
    if inp_vec.contains(&(val + 2)) {
        count += puzzle_two(&inp_vec, val + 2);
    }
    count
}

fn main() {
    let contents =
        fs::read_to_string("src/input.txt").expect("Something went wrong reading the file");
    let inp = contents;
    let mut inp_vec: Vec<u32> = inp.split('\n').map(|s| s.parse::<u32>().unwrap()).collect();
    // First puzzle
    inp_vec.sort();
    puzzle_one(&inp_vec);

    // Second puzzle
    // let count = puzzle_two(&inp_vec, 0);
    // println!("{}", count);
}
