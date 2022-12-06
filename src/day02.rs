use crate::read_input;

fn parse_lims(sl: &str) -> Vec<i32> {
    sl.split("-")
	.map(|s| {
	    s.parse::<i32>().unwrap()
	})
	.collect()
}

fn count_char(s: &str, c: char, lo: i32, hi: i32) -> bool {
    let mut count = 0;
    for cs in s.chars() {
	if cs == c {
	    count += 1;
	}
	if count > hi {
	    return false;
	}
    }
    return count >= lo;
}

pub fn solve_1() -> i32 {
    let input_string = read_input::read_input(2_i32);
    let input_lines = input_string.split("\n");
    let split_input_lines = input_lines.map(|s| { s.split(" ")});
    let mut count = 0;
    for mut sl in split_input_lines {
	let lo_hi = parse_lims(sl.next().unwrap());
	let lo = lo_hi[0];
	let hi = lo_hi[1];
	let c = sl.next().unwrap()
	    .chars().next().unwrap();
	let pwd = sl.next().unwrap();
	if count_char(pwd, c, lo, hi) {
	    count += 1;
	}
    }
    count
}

fn verify_pwd(pwd_str: &str, c: char, idx1: i32, idx2: i32) -> bool {
    let pwd = pwd_str.to_string();
    let lo: usize = idx1 as usize;
    let hi: usize = idx2 as usize; 
    let mut pwd_iter = pwd.chars();
    let mut pwd_iter_2 = pwd.chars().clone();
    let c1 = pwd_iter.nth(lo-1).unwrap();
    if c1 != c {
	return false
    }
    let c2 = pwd_iter_2.nth(hi-1).unwrap();
    return c2 != c;
}

pub fn solve_2() -> i32 {
    let input_string = read_input::read_input(2_i32);
    let input_lines = input_string.split("\n");
    let split_input_lines = input_lines.map(|s| { s.split(" ")});
    let mut count = 0;
    for mut sl in split_input_lines {
	let lo_hi = parse_lims(sl.next().unwrap());
	let lo = lo_hi[0];
	let hi = lo_hi[1];
	let c = sl.next().unwrap()
	    .chars().next().unwrap();
	let pwd = sl.next().unwrap();
	if verify_pwd(pwd, c, lo, hi) {
	    count += 1;
	}
    }
    count
}


#[test]
fn test_1() {
    let lo = 1;
    let hi = 3;
    let c: char = 'a';
    let pwd = "abcde";

    assert!(verify_pwd(pwd, c, lo, hi));
}

#[test]
fn test_2() {
    let lo = 1;
    let hi = 3;
    let c: char = 'b';
    let pwd = "cdefg";

    assert!(!verify_pwd(pwd, c, lo, hi));
}

#[test]
fn test_3() {
    let lo = 2;
    let hi = 9;
    let c: char = 'c';
    let pwd = "ccccccccc";

    assert!(!verify_pwd(pwd, c, lo, hi));
}
