use std::path::{Path};
use std::fs;
use std::collections::HashSet;

pub fn read_input(day: i32) -> String {
    let file_name = format!("../../data/{}", format!("{}.txt", day));
    let file_path = Path::new(&file_name);
    let contents = fs::read_to_string(file_path)
        .expect("File's not there, mate. Consider downloading the input and saving as <day>.txt");
    contents.strip_suffix("\n").unwrap().to_string()
}

pub fn solve1_1() -> i32 {
    let input_string = read_input(1_i32);
    let input_lines = input_string.split("\n");
    let mut required = HashSet::new();
    for line in input_lines {
        let n: i32 = line.parse().unwrap();
        let diff = 2020_i32 - n;
        if required.contains(&diff) {
            return n * diff;
        } else {
            required.insert(n);
        }
    }
    return -1_i32;
}

fn main() {
    println!("{}", solve1_1());
}
