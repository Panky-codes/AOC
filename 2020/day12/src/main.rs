use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;
use std::{fs, hash::Hash};

const _RAW_INP1: &str = r"F10
N3
F7
L270
F11";

const _RAW_INP2: &str = r"F10
N3
F7
R90
F11
R90
F2";

#[derive(Copy, Clone, PartialEq, Hash, Eq)]
enum Dir {
    North,
    East,
    South,
    West,
    Right,
    Left,
    Forward,
}

fn parse_input(input: &str) -> Vec<(Dir, isize)> {
    let re = Regex::new(r"^([A-Z])(\d*)").unwrap();
    let mut ret_vec: Vec<(Dir, isize)> = vec![];

    for line in input.split("\n") {
        let re_cap = re.captures(line).unwrap();
        let dir;
        let mut dist = 0;
        match re_cap.get(1).unwrap().as_str() {
            "N" => {
                dir = Dir::North;
                dist = isize::from_str(re_cap.get(2).unwrap().as_str()).unwrap();
            }
            "E" => {
                dir = Dir::East;
                dist = isize::from_str(re_cap.get(2).unwrap().as_str()).unwrap();
            }
            "W" => {
                dir = Dir::West;
                dist = -isize::from_str(re_cap.get(2).unwrap().as_str()).unwrap();
            }
            "S" => {
                dir = Dir::South;
                dist = -isize::from_str(re_cap.get(2).unwrap().as_str()).unwrap();
            }
            "R" => {
                dir = Dir::Right;
                dist = (isize::from_str(re_cap.get(2).unwrap().as_str()).unwrap()) / 90;
            }
            "L" => {
                dir = Dir::Left;
                dist = (-isize::from_str(re_cap.get(2).unwrap().as_str()).unwrap()) / 90;
            }
            "F" => {
                dir = Dir::Forward;
                dist = isize::from_str(re_cap.get(2).unwrap().as_str()).unwrap();
            }
            _ => unreachable!(),
        }
        ret_vec.push((dir, dist));
    }
    ret_vec
}

fn puzzle2(inp: &Vec<(Dir, isize)>) -> isize {
    let mut NS = 1;
    let mut EW = 10;
    let mut way_ns: isize = 0;
    let mut way_ew: isize = 0;

    for val in inp {
        match val.0 {
            Dir::North => NS += val.1,
            Dir::South => NS += val.1,
            Dir::East => EW += val.1,
            Dir::West => EW += val.1,
            Dir::Right | Dir::Left => {
                //R90 or L270 are the same
                if val.1 == 1 || val.1 == -3 {
                    let tmp_ew = EW.clone();
                    EW = NS;
                    NS = -1 * tmp_ew;
                }
                //R180 or L180 are the same
                if val.1 == 2 || val.1 == -2 {
                    // let tmp_ew = EW.clone();
                    EW = -1 * EW;
                    NS = -1 * NS;
                }
                //R270 or L90 are the same
                if val.1 == 3 || val.1 == -1 {
                    let tmp_ew = EW.clone();
                    EW = -1 * NS;
                    NS = tmp_ew;
                }
            }
            Dir::Forward => {
                way_ew += val.1 * EW;
                way_ns += val.1 * NS;
            }
        }
    }

    way_ew.abs() + way_ns.abs()
}

fn puzzle1(inp: &Vec<(Dir, isize)>) -> isize {
    let mut NS = 0;
    let mut EW = 0;
    let mut cur_dir = Dir::East; // Always start with east

    let find_next_dir = |cur: Dir, mov_ind: isize| -> Dir {
        let circ_dir: Vec<Dir> = vec![Dir::North, Dir::East, Dir::South, Dir::West];
        let vec_len = circ_dir.len();
        let cur_index = circ_dir.iter().position(|&x| x == cur).unwrap();
        if mov_ind.is_positive() {
            let next_index: usize = (cur_index + mov_ind as usize) % vec_len;
            return circ_dir[next_index];
        } else {
            // If the value is negative (reverse move), add the neg value with length and use it to
            // move forward
            let offset: isize = vec_len as isize + mov_ind;
            let next_index: usize = (cur_index + offset as usize) % vec_len;
            return circ_dir[next_index];
        }
    };

    for val in inp {
        match val.0 {
            Dir::North => NS += val.1,
            Dir::South => NS += val.1,
            Dir::East => EW += val.1,
            Dir::West => EW += val.1,
            Dir::Right => {
                cur_dir = find_next_dir(cur_dir, val.1);
            }
            Dir::Left => {
                cur_dir = find_next_dir(cur_dir, val.1);
            }
            Dir::Forward => match cur_dir {
                Dir::East => EW += val.1,
                Dir::West => EW += (-val.1),
                Dir::North => NS += val.1,
                Dir::South => NS += (-val.1),
                _ => unreachable!(),
            },
        }
    }

    EW.abs() + NS.abs()
}

fn main() {
    let contents =
        fs::read_to_string("src/input.txt").expect("Something went wrong reading the file");
    let input = contents.as_str();
    // let input = _RAW_INP1;

    let dir_pair = parse_input(input);

    let manhattan_dist = puzzle1(&dir_pair);

    println!("Puzzle 1 ans: {}", manhattan_dist);

    let manhattan_dist2 = puzzle2(&dir_pair);

    println!("Puzzle 2 ans: {}", manhattan_dist2);
}
