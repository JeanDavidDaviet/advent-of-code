use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let mut acc: i32 = 0;
    let mut basement: usize = 1;
    for (index, a) in input.chars().enumerate() {
        acc += match a {
            '(' => 1,
            ')' => -1,
            _ => 0
        };
        if acc == -1 && basement == 1 {
            basement = index + 1;
        }
    }
    dbg!(basement);
}
