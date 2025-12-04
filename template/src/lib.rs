use nom::character::complete::{digit1, newline, space1};
use nom::combinator::{map_res, recognize};
use nom::multi::separated_list1;
use nom::IResult;
use std::str;

pub fn parse_input(data: &str) -> IResult<&str, Vec<Vec<u8>>> {
    separated_list1(
        newline,
        separated_list1(space1, map_res(recognize(digit1), str::parse)),
    )(data)
}
