use std::fs;

fn get_square_feet(l: u32, w: u32, h: u32) -> u32 {
    let sizes: Vec<u32> = vec![l * w, w * h, h * l];
    let wrapping: u32 = (2 * sizes[0]) + (2 * sizes[1]) + (2 * sizes[2]);
    let slack: &u32 = sizes.iter().min().unwrap_or(&0);
    wrapping + slack
}

fn get_square_feet_from_line(line: &str) -> u32 {
    let values_as_string: Vec<&str> = line.split('x').collect();
    
    get_square_feet(
        values_as_string[0].parse::<u32>().unwrap_or(0),
        values_as_string[1].parse::<u32>().unwrap_or(0),
        values_as_string[2].parse::<u32>().unwrap_or(0),
    )
}

fn main() {
    let input: String = fs::read_to_string("./input.txt").unwrap_or_default();
    let mut total: u32 = 0;
    for line in input.lines() {
        total += get_square_feet_from_line(&String::from(line));
    }
    dbg!(total);
}
