use std::{fs, collections::BTreeSet};

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let mut unique_houses: BTreeSet<(i32, i32)> = BTreeSet::new();
    let mut x = 0;
    let mut y = 0;
    for direction in input.chars() {
        match direction {
            '^' => y -= 1,
            'v' => y += 1,
            '<' => x -= 1,
            '>' => x += 1,
            _ => (),
        }
        unique_houses.insert((x, y));
    }
    dbg!(unique_houses.len() + 1);
}
