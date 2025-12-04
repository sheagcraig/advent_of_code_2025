use nom::branch::alt;
use nom::character::complete::{char, digit1, newline};
use nom::combinator::{map_res, recognize, value};
use nom::multi::separated_list0;
use nom::sequence::pair;
use nom::IResult;

#[derive(Clone, Debug)]
pub enum Direction {
    Left,
    Right,
}

pub fn parse_lists(data: &str) -> IResult<&str, Vec<(Direction, isize)>> {
    separated_list0(
        newline,
        pair(
            alt((
                value(Direction::Left, char('L')),
                value(Direction::Right, char('R')),
            )),
            map_res(recognize(digit1), str::parse),
        ),
    )(data)
}
