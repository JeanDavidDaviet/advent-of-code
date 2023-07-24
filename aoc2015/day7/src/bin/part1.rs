#[allow(unused_imports)]
use std::fs;

use nom::IResult;
use nom::branch::alt;
use nom::character::complete::{alpha1, alpha0, multispace1, char, multispace0};
use nom::character::complete::{digit0, digit1};
use nom::combinator::map;
use nom::sequence::{separated_pair, tuple, preceded, terminated, pair};
use nom::bytes::complete::tag;

#[derive(Clone, Debug)]
struct Line(String, String);

fn parse_left_side(line: &str) -> IResult<&str, (&str, &str)> {
    pair(
        terminated(
            alpha1,
            tag(" RSHIFT ")
        ),
        alt((
            digit1,
            alpha1
        )),
    )(line)
}

fn parse_not(line: &str) -> IResult<&str, (&str, &str)> {
    map(
        preceded(
            tag("NOT "),
            alpha1,
        ),
        |found| ("EMPTY", found)
    )(line)
}

#[allow(dead_code)]
fn parse_line(line: &str) -> IResult<&str, ((&str, &str), &str)> {
    separated_pair(
        alt((
            parse_left_side,
            parse_not
        )),
        tag(" -> "),
        alt((
            digit1,
            alpha1
        )),
    )(line)
}

fn main() {
    // let input: String = fs::read_to_string("./input.txt").unwrap_or_default();
    // let entries: Vec<&str> = input.lines().into_iter().map(|s| s.trim()).collect();
    // for entry in entries {
    //     dbg!(parse_line(entry));
    // }
    // let test = parse_nested("1 -> x");
    // dbg!("{}", test);

    dbg!(parse_line("a RSHIFT bn -> v"));
    dbg!(parse_line("NOT ge -> z"));
}

#[cfg(test)]
mod tests {
    use crate::{parse_line};

    #[test]
    fn test_parse_line() {
        let expected = Ok(
            (
                "",
                (
                    "123",
                    "ab",
                ),
            ),
        );
        let actual = parse_line("123 -> ab");
        assert_eq!(expected, actual);
    }
}