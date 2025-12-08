use std::str;

#[derive(PartialEq, Debug)]
pub enum Map {
    Start,
    Splitter,
    Empty,
}

pub fn parse_input(data: &str) -> Vec<Vec<Map>> {
    data.lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'S' => Map::Start,
                    '^' => Map::Splitter,
                    '.' => Map::Empty,
                    something_else => panic!("Unexpected char in input {something_else:?}"),
                })
                .collect()
        })
        .collect()
}
