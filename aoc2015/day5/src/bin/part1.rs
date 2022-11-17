use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap_or_default();
    dbg!(input
        .lines()
        .filter(|line| line
            .chars()
            .filter(|&character| character == 'a'
                || character == 'e'
                || character == 'i'
                || character == 'o'
                || character == 'u')
            .count()
            >= 3)
        .filter(|line| line
            .chars()
            .collect::<Vec<char>>()
            .windows(2)
            .any(|chars| chars[0] == chars[1]))
        .filter(|line| !(line.contains("ab")
            || line.contains("cd")
            || line.contains("pq")
            || line.contains("xy")))
        .fold(0, |total, _| total + 1));
}
