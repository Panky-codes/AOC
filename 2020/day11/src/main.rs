use std::{clone, convert::TryInto, fs, usize};
const _RAW_INP1: &str = r"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

#[derive(Copy, Clone, PartialEq)]
enum Seat {
    Floor,
    Empty,
    Occupied,
}

fn get_neighbours(idx: isize, row: isize, col: isize) -> Vec<isize> {
    let mut idx_vec: Vec<isize> = vec![];
    let has_left: bool = if idx % col > 0 { true } else { false };
    let has_right: bool = if idx % col < (col - 1) { true } else { false };
    let has_upper: bool = if idx >= col { true } else { false };
    let has_bottom: bool = if idx < ((row * col) - col) {
        true
    } else {
        false
    };

    // Left most column
    if has_left {
        idx_vec.push(idx - 1);
    }
    if has_right {
        idx_vec.push(idx + 1);
    }
    if has_upper {
        idx_vec.push(idx - col);
        if has_left {
            idx_vec.push(idx - 1 - col);
        }
        if has_right {
            idx_vec.push(idx + 1 - col);
        }
    }

    if has_bottom {
        idx_vec.push(idx + col);
        if has_left {
            idx_vec.push(idx - 1 + col);
        }
        if has_right {
            idx_vec.push(idx + 1 + col);
        }
    }

    idx_vec
        .into_iter()
        .filter(|x| ((*x >= 0) && (*x < (row * col) as isize)))
        .collect()
}

fn check_empty_condition(inp: &Vec<Seat>, idx: isize, row: isize, col: isize) -> bool {
    let idx_vec = get_neighbours(idx, row, col);

    let res = idx_vec.iter().find(|x| inp[**x as usize] == Seat::Occupied);

    if let Some(_) = res {
        return false;
    }
    true
}

fn check_occu_condition(inp: &Vec<Seat>, idx: isize, row: isize, col: isize) -> bool {
    let idx_vec = get_neighbours(idx, row, col);
    let res = idx_vec
        .iter()
        .filter(|x| inp[**x as usize] == Seat::Occupied)
        .count();
    if res >= 4 {
        return true;
    }
    false
}

fn debug_print_layout(inp: &Vec<Seat>, row: usize, col: usize) {
    for r in 0..row {
        for c in 0..col {
            let ch = match inp[(r * col) + c] {
                Seat::Empty => "L",
                Seat::Floor => ".",
                Seat::Occupied => "#",
            };
            print!("{}", ch);
        }
        println!("");
    }
    println!("\n")
}

fn puzzle_one(inp: &mut Vec<Seat>, row: usize, col: usize) -> usize {
    let mut is_changed = true;

    while is_changed {
        is_changed = false;
        let mut cloned_inp = inp.clone();
        for idx in 0..(col * row) {
            let seat = inp[idx].clone();
            match seat {
                Seat::Floor => continue,
                Seat::Empty => {
                    // This will fill the seat
                    if check_empty_condition(
                        &inp,
                        idx.try_into().unwrap(),
                        row.try_into().unwrap(),
                        col.try_into().unwrap(),
                    ) {
                        is_changed = true;
                        cloned_inp[idx] = Seat::Occupied;
                    }
                }
                Seat::Occupied => {
                    // This will empty the seat
                    if check_occu_condition(
                        &inp,
                        idx.try_into().unwrap(),
                        row.try_into().unwrap(),
                        col.try_into().unwrap(),
                    ) {
                        is_changed = true;
                        cloned_inp[idx] = Seat::Empty;
                    }
                }
            }
        }
        *inp = cloned_inp;
        // debug_print_layout(&inp, row, col);
    }
    inp.iter().filter(|x| **x == Seat::Occupied).count()
}

fn main() {
    let _contents =
        fs::read_to_string("src/input.txt").expect("Something went wrong reading the file");

    let input = _contents;

    let mut inp_vec: Vec<Seat> = input
        .chars()
        .filter(|c| *c != '\n')
        .map(|c| match c {
            'L' => Seat::Empty,
            '.' => Seat::Floor,
            '#' => Seat::Occupied,
            _ => unreachable!(),
        })
        .collect();
    let row_len = input.split('\n').count();
    let col_len = inp_vec.len() / row_len;
    let count = puzzle_one(&mut inp_vec, row_len, col_len);
    println!("{:?}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_neighbors() {
        let ret_vec = get_neighbours(0, 3, 3);
        assert_eq!(vec![1, 3, 4], ret_vec);

        let ret_vec = get_neighbours(1, 3, 3);
        assert_eq!(vec![0, 2, 4, 3, 5], ret_vec);

        let ret_vec = get_neighbours(2, 3, 3);
        assert_eq!(vec![1, 5, 4], ret_vec);

        let ret_vec = get_neighbours(4, 3, 3);
        assert_eq!(vec![3, 5, 1, 0, 2, 7, 6, 8], ret_vec);

        let ret_vec = get_neighbours(8, 3, 3);
        assert_eq!(vec![7, 5, 4], ret_vec);
    }

    #[test]
    fn verify_neighbors_even_odd() {
        let ret_vec = get_neighbours(0, 3, 4);
        assert_eq!(vec![1, 4, 5], ret_vec);

        let ret_vec = get_neighbours(1, 3, 3);
        assert_eq!(vec![0, 2, 4, 3, 5], ret_vec);

        let ret_vec = get_neighbours(2, 3, 3);
        assert_eq!(vec![1, 5, 4], ret_vec);

        let ret_vec = get_neighbours(4, 3, 3);
        assert_eq!(vec![3, 5, 1, 0, 2, 7, 6, 8], ret_vec);

        let ret_vec = get_neighbours(8, 3, 3);
        assert_eq!(vec![7, 5, 4], ret_vec);
    }
}
