use std::fs;

const _RAW_INP1: &str = r"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";


fn puzzle(r_slope: usize, d_slope: usize) -> usize {
    let mut nr_of_trees: usize = 0;
    let mut x = 0;
    let mut y = d_slope;
    let contents = fs::read_to_string("src/input.txt").expect("Something went wrong reading the file");
    let cur_inp = contents;
    for (n, s) in cur_inp.split('\n').enumerate() {
        if n == y {
            x += r_slope;
            y += d_slope;
            x %= s.len();
            if s.as_bytes()[x] == b'#' {
                nr_of_trees += 1;
            }
        }
    }
    println!("{}", nr_of_trees);
    nr_of_trees
}

fn main() {
    let mult = puzzle(3, 1)
        * puzzle(1, 1)
        * puzzle(5, 1)
        * puzzle(7, 1)
        * puzzle(1, 2);
    println!("final answer is {}", mult);
}
