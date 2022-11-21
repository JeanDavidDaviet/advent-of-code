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

fn index_from_cell(x: i32, y: i32) -> i32 {
    y * 1000 + x
}

fn cell_from_index(i: i32) -> (i32, i32) {
    (i % 1000, i / 1000)
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let line = String::from("turn on 222,12 through 856,241");
    let positions = get_positions_from_line(line);
    let start = Point { x: positions.0, y: positions.1 };
    let end = Point { x: positions.2, y: positions.3 };

    let total_size = 1_000_000;
    let size = 1_000;
    let grid: [i32; 100_000] = [0; 100_000];
    
    dbg!(2001 % 1000, 2001 / 1000);
    for i in 0..total_size {
        dbg!(i, cell_from_index(i));
    } 


}
