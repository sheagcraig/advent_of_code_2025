use nom::branch::alt;
use nom::character::complete::{char, digit1, newline, space0, space1};
use nom::combinator::{map_res, recognize, value};
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::IResult;
use std::str;

#[derive(Clone, Debug)]
pub enum Op {
    Mul,
    Add,
}

// impl From<char> for Op {
//     fn from(c: char) -> Self {
//         match c {
//             '*' => Op::Mul,
//             '+' => Op::Add,
//             _ => panic!("Invalid operator"),
//         }
//     }
// }

pub fn parse_input(data: &str) -> IResult<&str, (Vec<Vec<usize>>, Vec<Op>)> {
    let (rem, digits) = separated_list1(
        newline,
        separated_list1(
            space1,
            preceded(space0, map_res(recognize(digit1), str::parse)),
        ),
    )(data)?;
    let (rem, _) = newline(rem)?;
    let (_, ops) = separated_list1(
        space1,
        alt((value(Op::Mul, char('*')), value(Op::Add, char('+')))),
    )(rem)?;
    Ok((rem, (digits, ops)))
}
