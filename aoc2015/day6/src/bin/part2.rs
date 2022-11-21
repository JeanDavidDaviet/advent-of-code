use std::{fs};

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
enum Action {
    Toggle,
    TurnOn,
    TurnOff,
}

fn get_positions_from_line(line: &str) -> (i32, i32, i32, i32) {
    let mut filtered_line = line.to_string();
    filtered_line.retain(|character| character.is_ascii_digit() || character == ',' || character == ' ');
    let trimmed_filtered_line = filtered_line.trim().split(' ').filter(|f| ! f.is_empty()).collect::<Vec<&str>>();
    let mut result = (0,0,0,0);
    for (i, l) in trimmed_filtered_line.iter().enumerate() {
        let t : Vec<i32> = l.split(',').filter_map(|x| x.parse::<i32>().ok()).collect();
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

fn update_grid<'a>(grid: &'a mut [i32], total_size: i32, start: &'a Point, end: &'a Point, action: Action) -> &'a [i32] {
    for i in 0..total_size {
        if i % 1000 >= start.x && i % 1000 <= end.x && i / 1000 >= start.y && i / 1000 <= end.y {
            let i = i as usize;
            match action {
                Action::TurnOn => grid[i] = 1,
                Action::TurnOff => grid[i] = 0,
                Action::Toggle => {
                    if grid[i] == 1 {
                        grid[i] = 0;
                    } else {
                        grid[i] = 1;
                    }
                }
            }
        }
    }
    grid
}

fn process_line(line: &str, grid: &mut [i32]) {
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
    let mut grid: [i32; 1_000_000] = [0; 1_000_000];
    let input = fs::read_to_string("./input.txt").unwrap_or_default();
    for line in input.lines() {
        process_line(line, &mut grid);
    }
    
    dbg!(grid.iter().filter(|i| i.is_positive()).sum::<i32>());
}
