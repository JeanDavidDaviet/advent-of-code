use std::{fs, error::Error, num::{ParseIntError, ParseFloatError}};

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap_or_default();
    
        let test = input
            .lines()
            .map(|line|
                line
                    .split(" ")
                    .collect::<Vec<&str>>()
            )
            ;
            for i in test {
                dbg!(i.iter().filter_map(|x| x.replace(",", ".").parse::<f32>().ok()).collect::<Vec<f32>>());
                // i.iter().map(|x| x.parse::<i32>().unwrap()).collect::<i32>();
            }
}
