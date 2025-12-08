use nom::bytes::complete::tag;
use nom::character::complete::{digit1, newline};
use nom::combinator::{map, map_res, recognize};
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair};
use nom::IResult;
use std::ops::RangeInclusive;
use std::str;

fn parse_digit(data: &str) -> IResult<&str, usize> {
    map_res(recognize(digit1), str::parse)(data)
}

pub fn parse_input(data: &str) -> IResult<&str, (Vec<RangeInclusive<usize>>, Vec<usize>)> {
    let (rem, ingredient_ids) = separated_list1(
        newline,
        map(
            separated_pair(parse_digit, tag("-"), parse_digit),
            |(start, end)| start..=end,
        ),
    )(data)
    .unwrap();

    let (rem, available_ingredients) =
        preceded(tag("\n\n"), separated_list1(newline, parse_digit))(rem).unwrap();

    Ok((rem, (ingredient_ids, available_ingredients)))
}
