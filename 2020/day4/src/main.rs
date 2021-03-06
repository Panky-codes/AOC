use regex::Regex;
use std::collections::HashMap;
use std::fs;

const _RAW_INP1: &str = r"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024 ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

const _RAW_INP2: &str = r"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

const _RAW_INP3: &str = r"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

fn main() {
    // Data structures
    let id_map: HashMap<&str, i32> = [
        ("byr", 1),
        ("iyr", 1),
        ("eyr", 1),
        ("hgt", 1),
        ("hcl", 1),
        ("ecl", 1),
        ("pid", 1),
    ]
    .iter()
    .cloned()
    .collect();

    let mut id_with_constraints_map = HashMap::new();
    id_with_constraints_map.insert("byr", Regex::new(r"19[2-9][0-9]|200[0-2]"));
    id_with_constraints_map.insert("iyr", Regex::new(r"201[0-9]|2020"));
    id_with_constraints_map.insert("eyr", Regex::new(r"202[0-9]|2030"));
    id_with_constraints_map.insert(
        "hgt",
        Regex::new(r"1[5-8][0-9]cm|19[0-3]cm|59in|6[0-9]in|7[0-6]in"),
    );
    id_with_constraints_map.insert("hcl", Regex::new(r"#[0-9a-f]{6}"));
    id_with_constraints_map.insert("ecl", Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$"));
    id_with_constraints_map.insert("pid", Regex::new(r"^[0-9]{9}$"));

    let contents =
        fs::read_to_string("src/input.txt").expect("Something went wrong reading the file");
    
    // Algorithm
    let puzzle_one = |s: &str| {
        let mut sum = 0;
        for id in s.split_whitespace() {
            let val: Vec<&str> = id.split(":").collect();
            if id_map.contains_key(val[0]) {
                sum += 1;
            }
        }
        if sum == id_map.len() {
            1
        } else {
            0
        }
    };

    let puzzle_two = |s: &str| {
        let mut sum = 0;
        for id in s.split_whitespace() {
            let val: Vec<&str> = id.split(":").collect();
            if id_with_constraints_map.contains_key(val[0])
                && id_with_constraints_map[val[0]].as_ref().unwrap().is_match(val[1])
            {
                sum += 1
            };
        }
        if sum == id_map.len() {
            1
        } else {
            0
        }
    };

    let puzzle_one_sum: usize = contents
        .split("\n\n")
        .filter(|x| *x != "")
        .map(puzzle_one)
        .sum();
    
    let puzzle_two_sum: usize = contents
        .split("\n\n")
        .filter(|x| *x != "")
        .map(puzzle_two)
        .sum();
    
    println!("{}", puzzle_one_sum);
    println!("{}", puzzle_two_sum);
}
