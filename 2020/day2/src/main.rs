use std::fs;

const _RAW_INP1: &str = r"1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

fn puzzle_one(cur_inp : &String) {
    let mut valid_pwd = 0;
    for s in cur_inp.split('\n') {
        let mut min_occ = 0;
        let mut max_occ = 0;
        let mut policy = "a";
        let mut policy_occ = 0;

        for (counter, c) in s.split_whitespace().enumerate() {
            match counter {
                0 => {
                    let occ: Vec<&str> = c.split('-').collect();
                    min_occ = occ[0].parse::<u32>().unwrap();
                    max_occ = occ[1].parse::<u32>().unwrap();
                }
                1 => {
                    let ref_vec: Vec<&str> = c.split(':').collect();
                    policy = ref_vec[0];
                }
                2 => {
                    for ch in c.chars() {
                        //if policy == ch {
                        if policy.contains(ch) {
                            policy_occ += 1;
                        }
                    }
                }
                _ => println!("Not a valid number"),
            }
        }
        if (policy_occ >= min_occ) && (policy_occ <= max_occ) {
            valid_pwd += 1;
        }
    }
    println!("{}", valid_pwd);
}

fn puzzle_two(cur_inp : &String) {
    let mut valid_pwd = 0;

    for s in cur_inp.split('\n') {
        let mut min_pos : usize = 0;
        let mut max_pos : usize = 0;
        let mut policy = "a";
        let mut is_valid_pwd = false;

        for (counter, c) in s.split_whitespace().enumerate() {
            match counter {
                0 => {
                    let occ: Vec<&str> = c.split('-').collect();
                    min_pos = occ[0].parse::<usize>().unwrap() - 1;
                    max_pos = occ[1].parse::<usize>().unwrap() - 1;
                }
                1 => {
                    let ref_vec: Vec<&str> = c.split(':').collect();
                    policy = ref_vec[0];
                }
                2 => {
                    for (i, ch) in c.chars().enumerate() {
                        if (policy.contains(ch)) && ((i == min_pos) || (i == max_pos)) {
                            is_valid_pwd ^= true;
                        }
                    }
                }
                _ => println!("Not a valid number"),
            }
        }
        if is_valid_pwd {
            valid_pwd += 1;
       }
    }
    println!("{}", valid_pwd);
}

fn main() {
    let cur_inp = fs::read_to_string("src/input.txt").expect("Something went wrong reading the file");
    puzzle_one(&cur_inp);
    puzzle_two(&cur_inp);
}
