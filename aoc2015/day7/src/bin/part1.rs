use std::fs;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha0, alpha1, char, multispace0, multispace1};
use nom::character::complete::{digit0, digit1};
use nom::combinator::map;
use nom::sequence::{pair, preceded, separated_pair, terminated, tuple};
use nom::IResult;

#[derive(Clone, Debug)]
struct Line(String, String);

fn parse_left_side(line: &str) -> IResult<&str, (&str, &str)> {
    pair(
        terminated(
            alt((  
                alpha1,
                digit1,
            )),
            alt((
                tag(" AND "),
                tag(" OR "),
                tag(" LSHIFT "),
                tag(" RSHIFT "),
            ))
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
    let input: String = fs::read_to_string("./input.txt").unwrap_or_default();
    let entries: Vec<&str> = input.lines().into_iter().map(|s| s.trim()).collect();
    for entry in entries {
        dbg!(parse_line(entry));
    }
}

#[cfg(test)]
mod tests {
    use crate::parse_line;

    #[test]
    fn test_parse_line_complete() {
        assert_eq!(
            Ok(("", (("a", "bn",), "v",),)),
            parse_line("a RSHIFT bn -> v")
        );
    }

    #[test]
    fn test_parse_line_not() {
        assert_eq!(
            Ok(("", (("EMPTY", "ge",), "z",),)),
            parse_line("NOT ge -> z")
        );
    }
}
