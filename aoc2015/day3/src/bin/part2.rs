use std::{fs, collections::BTreeSet};

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap_or_default();
    let mut unique_houses: BTreeSet<(i32, i32)> = BTreeSet::new();
    unique_houses.insert((0,0));

    let mut x_santa = 0;
    let mut y_santa = 0;
    let mut x_robot = 0;
    let mut y_robot = 0;

    for (index, direction) in input.chars().enumerate() {
        if index % 2 == 0 {
            match direction {
                '^' => y_santa -= 1,
                'v' => y_santa += 1,
                '<' => x_santa -= 1,
                '>' => x_santa += 1,
                _ => (),
            }
            unique_houses.insert((x_santa, y_santa));

        } else {
            match direction {
                '^' => y_robot -= 1,
                'v' => y_robot += 1,
                '<' => x_robot -= 1,
                '>' => x_robot += 1,
                _ => (),
            }
            unique_houses.insert((x_robot, y_robot));
        } 
    }
    dbg!(unique_houses.len());
}
