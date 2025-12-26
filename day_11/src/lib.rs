use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, newline, space1};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;
use std::str;

pub fn parse_line(data: &str) -> IResult<&str, (&str, Vec<&str>)> {
    separated_pair(alpha1, tag(": "), separated_list1(space1, alpha1))(data)
}

pub fn parse_input(data: &str) -> IResult<&str, Vec<(&str, Vec<&str>)>> {
    separated_list1(newline, parse_line)(data)
}
