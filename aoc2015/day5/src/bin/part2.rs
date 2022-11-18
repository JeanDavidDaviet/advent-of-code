use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap_or_default();
    dbg!(input
        .lines()
        .filter(|line| line
            .char_indices()
            .collect::<Vec<(usize, char)>>()
            .windows(2)
            .any(|chars| {
                let two_chars = String::from_iter([chars[0].1, chars[1].1]);
                *&line[(chars[0].0 + 2)..].contains(&two_chars)
            })
        )
        .filter(|line| line
            .chars()
            .collect::<Vec<char>>()
            .windows(3)
            .any(|chars| chars[0] == chars[2]))
        .fold(0, |total, _| total + 1)
    );
}
