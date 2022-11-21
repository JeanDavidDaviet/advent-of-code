use std::{fs};

#[derive(Debug, Clone, Copy)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug, Clone, Copy)]
enum Action {
    Toggle,
    TurnOn,
    TurnOff,
}

fn get_positions_from_line(line: &str) -> (u32, u32, u32, u32) {
    let mut filtered_line = line.to_string();
    filtered_line.retain(|character| character.is_ascii_digit() || character == ',' || character == ' ');
    let trimmed_filtered_line = filtered_line.trim().split(' ').filter(|f| ! f.is_empty()).collect::<Vec<&str>>();
    let mut result = (0,0,0,0);
    for (i, l) in trimmed_filtered_line.iter().enumerate() {
        let t : Vec<u32> = l.split(',').filter_map(|x| x.parse::<u32>().ok()).collect();
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

fn update_grid<'a>(grid: &'a mut [u32], total_size: u32, start: &'a Point, end: &'a Point, action: Action) -> &'a [u32] {
    for i in 0..total_size {
        if i % 1000 >= start.x && i % 1000 <= end.x && i / 1000 >= start.y && i / 1000 <= end.y {
            let i = i as usize;
            match action {
                Action::TurnOn => grid[i] += 1,
                Action::TurnOff => grid[i] = grid[i].checked_sub(1).unwrap_or(0),
                Action::Toggle => grid[i] += 2,
            }
        }
    }
    grid
}

fn process_line(line: &str, grid: &mut [u32]) {
    let action = match &line[0..7] {
        "turn on" => Some(Action::TurnOn),
        "turn of" => Some(Action::TurnOff),
        "toggle " => Some(Action::Toggle),
        _ => None
    };
    if action.is_some() {
        let action = action.unwrap();
        let positions = get_positions_from_line(line);
        let start = Point { x: positions.0, y: positions.1 };
        let end = Point { x: positions.2, y: positions.3 };
        update_grid(grid, 1_000_000, &start, &end, action);
    }
}

fn main() {
    let mut grid: [u32; 1_000_000] = [0; 1_000_000];
    let input = fs::read_to_string("./input.txt").unwrap_or_default();
    // let input = String::from("toggle 0,0 through 0,0");
    for line in input.lines() {
        process_line(line, &mut grid);
    }
    
    dbg!(grid.iter().sum::<u32>());
}
