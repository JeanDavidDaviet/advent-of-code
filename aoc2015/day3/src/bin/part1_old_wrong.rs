use std::{collections::HashSet, fs};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, Hash)]
struct House {
    point: Point,
    visited: u32,
}

impl Eq for House {}

impl PartialEq for House {
    fn eq(&self, other: &Self) -> bool {
        self.point.x == other.point.x && self.point.y == other.point.y
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let mut houses: HashSet<House> = HashSet::new();
    let mut location: Point = Point { x: 0, y: 0 };
    for direction in input.chars() {
        match direction {
            '^' => location.y -= 1,
            'v' => location.y += 1,
            '<' => location.x -= 1,
            '>' => location.x += 1,
            _ => (),
        }

        let new_house = House {
            point: Point {
                x: location.x,
                y: location.y,
            },
            visited: 1,
        };

        houses.insert(new_house);
        let house_to_update = houses.get(&new_house);
        if house_to_update.is_some() {
            let house: &mut &House = &mut house_to_update.unwrap();
            house.visited += 1;
            dbg!(house);

        }
    }
    dbg!(houses.get(&House {
        point: Point {
            x: 8,
            y: -55,
        },
        visited: 1,
    }));
}
