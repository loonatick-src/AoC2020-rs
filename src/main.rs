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
    // unreachable
    // TODO: consider using `Result<i32, Err>`
    return -1_i32;
}

pub fn solve1_2() -> i32 {
    let input_string = read_input(1_i32);
    let input_lines = input_string.split("\n");
    // forgive me lord for what I am about to do
    let numbers: Vec<i32> = input_lines.map(|x| x.parse::<i32>().unwrap()).collect();
    let n = numbers.len();
    for i in 0..n {
        let n1 = numbers[i];
        for j in i+1..n {
            let n2 = numbers[j];
            for k in j+1..n {
                let n3 = numbers[k];
                if (n1 + n2 + n3) == 2020_i32 {
                    return n1 * n2 * n3;
                }
            }
        }
    }

    // unreachable for nice input
    return -1;
}

fn main() {
    println!("{}", solve1_2());
}
