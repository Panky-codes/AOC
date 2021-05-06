use std::collections::HashMap;
use std::fs;

const _RAW_INP1: &str = r"939
7,13,x,x,59,x,31,19";


// fn puzzle2_bad_idea(inp: &HashMap<i32, i32>, big_input: bool) -> i64 {
//     let mult_fac: i64 = (inp.get(&0).unwrap() * inp.get(inp.get(&0).unwrap()).unwrap()).into();

//     let mut target_val: i64 = if big_input {
//         let close_fac = 100000000000000 / mult_fac;
//         close_fac * mult_fac
//     } else {
//         mult_fac
//     }; // Shortcut to reduce time

//     loop {
//         let mut cond_reached = false;
//         for h in inp {
//             if (target_val - inp.get(&0).unwrap().clone() as i64 + h.0.clone() as i64)
//                 % (h.1.clone() as i64)
//                 != 0
//             {
//                 cond_reached = false;
//                 break;
//             } else {
//                 cond_reached = true;
//             }
//         }
//         if cond_reached {
//             println!(
//                 "The puzzle two value is {}",
//                 (target_val - inp.get(&0).unwrap().clone() as i64)
//             );
//             break;
//         }
//         target_val += mult_fac;
//     }
//     0
// }

fn parse_inp(inp: &str) -> (i32, Vec<i32>) {
    let inp_split: Vec<&str> = inp.split('\n').collect();
    let target_val = inp_split[0].parse::<i32>().unwrap();
    // Filter map makes the life a bit easier
    let filter_inp: Vec<i32> = inp_split[1]
        .split(',')
        .into_iter()
        .filter_map(|x| x.parse::<i32>().ok())
        .collect();
    (target_val, filter_inp)
}

fn parse_inp2(inp: &str) -> HashMap<i32, i32> {
    let inp_split: Vec<&str> = inp.split('\n').collect();
    let mut index = 0;
    let mut ret_hash: HashMap<i32, i32> = HashMap::new();

    for c in inp_split[1].split(',') {
        if c != "x" {
            ret_hash.insert(index, c.parse::<i32>().unwrap());
        }
        index += 1;
    }
    ret_hash
}

// https://brilliant.org/wiki/extended-euclidean-algorithm/
fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

// https://brilliant.org/wiki/chinese-remainder-theorem/#solving-systems-of-congruences
fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

//Copied from https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
fn chinese_remainder(residues: &Vec<i64>, modulii: &Vec<i64>) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }
    Some(sum % prod)
}

//  --------------------------------

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
    let puz1_max = inp_vec
        .iter()
        .map(|x| {
            puzz1_hash.insert(target % x, *x);
            target % x
        })
        .max()
        .unwrap();

    let puz1_max_inp = puzz1_hash.get(&puz1_max).unwrap();

    // The min diff will be the input for which the diff is min - the modulo times the input
    // i.e., for the given example, it is (59 - 54) * 59
    println!(
        "Puzzle 1 ans : {}",
        (puz1_max_inp - puz1_max) * puz1_max_inp
    );

    let puzz2_inp = parse_inp2(inp);

    let residues: Vec<i64> = puzz2_inp
        .iter()
        .map(|(index, val)| (val - index) as i64)
        .collect();

    let modulii: Vec<i64> = puzz2_inp
        .iter()
        .map(|(_, val)| (*val) as i64)
        .collect();
    println!("{}", chinese_remainder(&residues, &modulii).unwrap());
}
