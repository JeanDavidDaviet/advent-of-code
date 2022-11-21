// use std::{fs, error::Error, num::{ParseIntError, ParseFloatError}};
use std::{fs};

fn get_positions_from_line(line: String) -> (i32, i32, i32, i32) {
    let mut filtered_line = line.clone();
    filtered_line.retain(|character| character.to_digit(10).is_some() || character == ',' || character == ' ');
    let trimmed_filtered_line = filtered_line.trim().split(" ").filter(|f| ! f.is_empty()).collect::<Vec<&str>>();
    let mut result = (0,0,0,0);
    for (i, l) in trimmed_filtered_line.iter().enumerate() {
        let t : Vec<i32> = l.split(",").filter_map(|x| x.parse::<i32>().ok()).collect();
        if i == 0 {
            result.0 = t[0];
            result.1 = t[1];
        } else {
            result.2 = t[0];
            result.3 = t[1];
        }
    }
    result
}

fn main() {
    let line = String::from("turn on 222,12 through 856,241");
    dbg!(get_positions_from_line(line));
}
