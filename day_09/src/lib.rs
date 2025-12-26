use glam::I64Vec2;
use nom::character::complete::{char, digit1, newline};
use nom::combinator::{map, map_res, recognize};
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;
use std::str;

pub fn parse_input(data: &str) -> IResult<&str, Vec<I64Vec2>> {
    separated_list1(
        newline,
        map(
            tuple((
                map_res(recognize(digit1), str::parse),
                char(','),
                map_res(recognize(digit1), str::parse),
            )),
            |(x, _, y)| I64Vec2::new(x, y),
        ), // separated_list1(char(','), map_res(recognize(digit1), str::parse)),
    )(data)
}
