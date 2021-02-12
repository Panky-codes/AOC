use std::fs;
use std::{collections::HashSet, vec};
const _RAW_INP1: &str = r"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

struct CircBuffer {
    arr: Vec<i64>,
    sz: usize,
    cap: usize,
    head: usize,
}

// For puzzle 1: Using a simple circular buffer to add elements
// Circular buffer is not really needed as we can simply use a slice
// as done in the part 2
impl CircBuffer {
    fn add(&mut self, val: i64) {
        if self.sz < self.cap {
            self.arr.push(val);
            self.sz += 1;
            self.head = self.sz;
        } else {
            self.head %= self.cap;
            self.arr[self.head as usize] = val;
            self.head += 1;
        }
    }
    // This helper function checks if the sum of a pair in the buffer is equal
    // the next element
    fn find_pair(&self, val: i64) -> bool {
        let mut set: HashSet<i64> = HashSet::new();
        let mut ret_val = false;
        for v in &self.arr {
            let diff = val - *v;

            set.insert(*v);
            if set.contains(&(diff)) && (*v != val - *v) {
                ret_val = true;
            }
        }
        ret_val
    }
}

// For puzzle two: Take a slice backwards from the target element
// and find the target value
fn min_max_prod(inp_vec: &Vec<i64>, val_index: usize) -> Option<i64> {
    for i in (0..val_index).rev() {
        for j in (0..i).rev() {
            let sum: i64 = inp_vec[j..=i].iter().sum();
            if sum == inp_vec[val_index] {
                let min = inp_vec[j..=i].iter().min().unwrap();
                let max = inp_vec[j..=i].iter().max().unwrap();
                println!("{:?} {}", min, max);
                return Some(min + max);
            }
        }
    }
    None
}

fn main() {
    let contents =
        fs::read_to_string("src/input.txt").expect("Something went wrong reading the file");
    let inp = contents;
    let mut val_index: usize = 0;
    let mut circ = CircBuffer {
        arr: Vec::new(),
        sz: 0,
        cap: 25,
        head: 0,
    };

    let vec_inp: Vec<i64> = inp.split("\n").map(|s| s.parse::<i64>().unwrap()).collect();
    for i in 0..circ.cap {
        circ.add(vec_inp[i]);
    }
    for i in (circ.cap as usize)..vec_inp.len() {
        let val = vec_inp[i as usize];
        if !circ.find_pair(val) {
            val_index = i;
            println!("Found the first number {} at index: {}", val, val_index);
            break;
        }
        circ.add(val);
    }
    println!(
        "Puzzle 2 soln is {}",
        min_max_prod(&vec_inp, val_index).unwrap()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn verify_circ_buffer() {
        let mut circ = CircBuffer {
            arr: Vec::new(),
            sz: 0,
            cap: 2,
            head: 0,
        };

        circ.add(1);
        circ.add(2);
        circ.add(3);

        assert_eq!(3, circ.arr[0]);
        assert_eq!(2, circ.arr[1]);

        circ.add(4);
        assert_eq!(4, circ.arr[1]);
    }

    #[test]
    fn verify_find_sum() {
        let mut circ = CircBuffer {
            arr: Vec::new(),
            sz: 0,
            cap: 2,
            head: 0,
        };

        circ.add(1);
        circ.add(2);

        let true_ret = circ.find_pair(3);
        assert_eq!(true, true_ret);
        let false_ret = circ.find_pair(4);
        assert_eq!(false, false_ret);
        circ.add(5);
        circ.add(15);
        assert_eq!(true, circ.find_pair(20));
    }
}
