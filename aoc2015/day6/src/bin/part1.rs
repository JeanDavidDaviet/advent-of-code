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

fn update_grid<'a>(grid: &'a mut [i32], total_size: i32, start: &'a Point, end: &'a Point) -> &'a [i32] {
    let start_index = index_from_cell(start.x, start.y);
    let end_index = index_from_cell(end.x, end.y);
    dbg!(start, end, start_index, end_index);
    for i in 0..total_size {
        if i % 1000 >= start.x && i % 1000 <= end.x && i / 1000 >= start.y && i / 1000 <= end.y {
            let i = i as usize;
            grid[i] = 1;
        }
    }
    grid
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let line = String::from("turn on 222,12 through 856,241");
    let positions = get_positions_from_line(line);
    let start = Point { x: positions.0, y: positions.1 };
    let end = Point { x: positions.2, y: positions.3 };

    let mut grid: [i32; 1_000_000] = [0; 1_000_000];
    update_grid(&mut grid, 1_000_000, &start, &end);    

    dbg!(grid.iter().filter(|i| i.is_positive()).sum::<i32>());
}
