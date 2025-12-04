use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map_res, recognize};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;
use std::str;

pub fn parse_lists(data: &str) -> IResult<&str, Vec<(usize, usize)>> {
    separated_list1(
        tag(","),
        separated_pair(
            map_res(recognize(digit1), str::parse),
            tag("-"),
            map_res(recognize(digit1), str::parse),
        ),
    )(data)
}
