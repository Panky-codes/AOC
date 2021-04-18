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
    let (count_one, count_three) = inp_vec.windows(2).fold(
        (0, 0), // Plus one for the last charger
        |(count_one, count_three), x| match x[1] - x[0] {
            1 => (count_one + 1, count_three),
            3 => (count_one, count_three + 1),
            _ => unreachable!(),
        },
    );

    println!("Puzzle 1 ans is {:?}", count_one * count_three);
}

// Works for the first two sample inputs but it does not
// scale for the main input
fn _puzzle_two_recur(inp_vec: &Vec<u32>, val: u32) -> u32 {
    let mut count: u32 = 0;

    if val == inp_vec[inp_vec.len() - 1] {
        print!(".");
        return 1;
    }
    if inp_vec.contains(&(val + 1)) {
        count += _puzzle_two_recur(&inp_vec, val + 1);
    }
    if inp_vec.contains(&(val + 3)) {
        count += _puzzle_two_recur(&inp_vec, val + 3);
    }
    if inp_vec.contains(&(val + 2)) {
        count += _puzzle_two_recur(&inp_vec, val + 2);
    }
    count
}

fn puzzle_two(inp_vec: &Vec<u32>) -> u64 {
    // A big Vector (like a hashmap) with index used as the charger joltage and value corresponds
    // to the nr. of possible ways to reach it
    // TODO: Change it to hashmap to save space
    let mut conn_count: Vec<u64> = vec![0; (inp_vec.last().unwrap() + 1) as usize];

    conn_count[0] = 1; // The first adapter is the starting point

    for (index, val) in inp_vec.iter().skip(1).enumerate() {
        // Skip the first element as it is the first starting point
        conn_count[*val as usize] += conn_count[(*val - 1) as usize];
        if (*val as i32 - 2) >= 0 {
            conn_count[*val as usize] += conn_count[(*val - 2) as usize];
        }
        if (*val as i32 - 3) >= 0 {
            conn_count[*val as usize] += conn_count[(*val - 3) as usize];
        }
    }
    *conn_count.last().unwrap()
}

fn main() {
    let contents =
        fs::read_to_string("src/input.txt").expect("Something went wrong reading the file");
    let inp = contents;
    let mut inp_vec: Vec<u32> = inp.split('\n').map(|s| s.parse::<u32>().unwrap()).collect();
    // First puzzle
    inp_vec.insert(0, 0);

    inp_vec.sort_unstable();

    inp_vec.push(*inp_vec.last().unwrap() + 3);

    puzzle_one(&inp_vec);

    // Second puzzle
    let count = puzzle_two(&inp_vec);
    println!("{}", count);
}
