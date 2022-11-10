use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let mut acc: i32 = 0;
    for a in input.chars() {
        acc += match a {
            '(' => 1,
            ')' => -1,
            _ => 0
        };
         1;
    }   
    dbg!(acc);
}
