use nom::character::complete::alpha1;
#[allow(unused_imports)]
use nom::character::complete::{digit0, digit1};
use nom::multi::many0;
use nom::sequence::separated_pair;
use nom::bytes::complete::tag;

#[derive(Clone, Debug)]
struct Line(String, String);

#[allow(dead_code)]
fn parse_many(line: &str) -> Result<(&str, Vec<(&str, &str)>), nom::Err<nom::error::Error<&str>>> {
    many0(separated_pair(
        digit1, 
        tag(" -> "),
        alpha1,
    ))(line)
}

#[allow(dead_code)]
fn parse_one(line: &str) -> Result<(&str, (&str, &str)), nom::Err<nom::error::Error<&str>>> {
    separated_pair(
        digit1,
        tag(" -> "),
        alpha1,
    )(line)
}

fn main() {
    let line = parse_many("123 -> ab456 -> y");
    dbg!("{}", line.unwrap());
}
