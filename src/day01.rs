use std::collections::HashSet;

use crate::read_input;

pub fn solve_1() -> i32 {
    let input_string = read_input::read_input(1_i32);
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

pub fn solve_2() -> i32 {
    let input_string = read_input::read_input(1_i32);
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
