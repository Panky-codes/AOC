use std::{collections::HashSet, fs, usize};
const _RAW_INP1: &str = r"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

fn find_loop(data: &Vec<(&str, i32)>) -> (i32, bool) {
    let mut pc: i32 = 0; // program counter
    let mut is_inifinite_loop = false;
    let mut acc = 0;
    let mut pc_set: HashSet<u32> = HashSet::new();

    while !is_inifinite_loop && (pc as usize) < (data.len()) {
        let instr = data[pc as usize];
        match instr.0 {
            "acc" => {
                acc += instr.1;
                pc += 1;
            }
            "jmp" => {
                pc += instr.1;
            }
            "nop" => {
                pc += 1;
            }
            _ => {}
        }
        if !pc_set.insert(pc as u32) {
            is_inifinite_loop = true;
        }
    }
    (acc, is_inifinite_loop)
}

fn main() {
    let contents =
        fs::read_to_string("src/input.txt").expect("Something went wrong reading the file");
    let cur_inp = contents;

    let data_struct: Vec<(&str, i32)> = cur_inp
        .split("\n")
        .map(|s| {
            let mut x = s.split_whitespace();
            let y: &str = x.next().unwrap();
            let x: i32 = x.next().unwrap_or("0").parse::<i32>().unwrap();
            (y, x)
        })
        .collect();
    // Later will be used for the second puzzle
    let mut temp_data = data_struct.clone();

    let (acc, _) = find_loop(&data_struct);
    println!("{:?}", acc);

    // Second puzzle
    for (index, instr_set) in (&data_struct).iter().enumerate() {
        let instr = instr_set.0;
        temp_data[index].0 = if instr == "jmp" { "nop" } else { instr_set.0 };
        let ret = find_loop(&temp_data);
        if ret.1 == false {
            println!("Acc value is: {}", ret.0);
            break;
        }
        // Restore the data
        temp_data[index] = data_struct[index];
    }
}
