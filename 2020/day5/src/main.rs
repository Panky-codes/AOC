use std::fs;

const _RAW_INP1: &str = r"BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL";

fn parse_value(s: &str, left: usize, right: usize) -> usize {
    let c = s.chars().nth(0);
    if s.len() == 1 {
        if c == Some('F') || c == Some('L') {
            left
        } else {
            right
        }
    } else {
        let mut new_left: usize = left;
        let mut new_right: usize = right;
        let mid_point = (left + right) / 2;
        if c == Some('F') || c == Some('L') {
            new_right = mid_point;
        } else {
            new_left = mid_point + 1;
        }
        let _string = s.split_at(1).1;

        parse_value(s.split_at(1).1, new_left, new_right)
    }
}

fn parse_row_col(s: &str) -> usize {
    let (row_str, col_str) = s.split_at(7);
    let row_value = parse_value(row_str, 0, 127);
    let col_value = parse_value(col_str, 0, 7);
    8 * row_value + col_value
}
fn main() {
    let contents =
        fs::read_to_string("src/input.txt").expect("Something went wrong reading the file");
    let max_sum = contents.split("\n").map(parse_row_col).max();
    println!("puzzle one: {}", max_sum.unwrap_or(0));

    let parsed_values: Vec<usize> = contents.split("\n").map(parse_row_col).collect();

    // excluding the first and the last
    for x in 63..1016 {
        if !parsed_values.contains(&x)
            && (parsed_values.contains(&(x - 1)) && parsed_values.contains(&(x + 1)))
        {
            println!("Puzzle two {}", x);
        }
    }
}
