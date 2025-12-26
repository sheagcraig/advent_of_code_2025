use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::{char, digit1, line_ending, space0};
use nom::combinator::map;
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, tuple};
use nom::IResult;
use std::str;

pub fn parse_input(
    data: &str,
) -> IResult<&str, (Vec<Vec<bool>>, Vec<Vec<Vec<u8>>>, Vec<Vec<u16>>)> {
    // Parse the lights section [.##.]
    let lights = delimited(
        tag("["),
        map(take_while1(|c| c == '.' || c == '#'), |s: &str| {
            s.chars().map(|c| c == '#').collect()
        }),
        tag("]"),
    );

    // Parse a number group like (1,2,3) or (5)
    let number_group = delimited(
        tag("("),
        separated_list1(char(','), map(digit1, |s: &str| s.parse::<u8>().unwrap())),
        tag(")"),
    );

    // Parse the final brace group {3,5,4,7}
    let brace_group = delimited(
        tag("{"),
        separated_list1(char(','), map(digit1, |s: &str| s.parse::<u16>().unwrap())),
        tag("}"),
    );

    // Parse one line: lights, multiple (), and one {}
    let line_parser = map(
        tuple((
            lights,
            many1(delimited(space0, number_group, space0)),
            delimited(space0, brace_group, space0),
        )),
        |(lights, paren_groups, brace_nums)| (lights, paren_groups, brace_nums),
    );

    // Parse multiple lines and collect into separate vectors
    map(separated_list1(line_ending, line_parser), |lines| {
        let mut all_lights = Vec::new();
        let mut all_paren_groups = Vec::new();
        let mut all_brace_groups = Vec::new();

        for (lights, paren_groups, brace_nums) in lines {
            all_lights.push(lights);
            all_paren_groups.push(paren_groups);
            all_brace_groups.push(brace_nums);
        }

        (all_lights, all_paren_groups, all_brace_groups)
    })(data)
}
