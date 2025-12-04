use glam::IVec2;
use std::collections::HashMap;
use std::str;

#[derive(Debug, PartialEq)]
pub enum Map {
    Floor,
    Paper,
}

pub static SEARCH_DIRS: &[IVec2] = &[
    IVec2::NEG_X,
    IVec2::NEG_Y,
    IVec2::X,
    IVec2::Y,
    IVec2::ONE,
    IVec2::NEG_ONE,
    IVec2::new(-1, 1),
    IVec2::new(1, -1),
];

pub fn parse_input(data: &str) -> HashMap<IVec2, Map> {
    data.lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                (
                    IVec2::new(x as i32, y as i32),
                    match c {
                        '.' => Map::Floor,
                        '@' => Map::Paper,
                        _ => panic!("Unknown map character: {}", c),
                    },
                )
            })
        })
        .collect()
}
