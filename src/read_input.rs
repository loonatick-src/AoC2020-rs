use std::fs;
use std::path::{Path};

pub fn read_input(day: i32) -> String {
    let file_name = format!("../../data/{}", format!("{}.txt", day));
    let file_path = Path::new(&file_name);
    let contents = fs::read_to_string(file_path)
        .expect("File's not there, mate. Consider downloading the input and saving as <day>.txt");
    contents.strip_suffix("\n").unwrap().to_string()
}

