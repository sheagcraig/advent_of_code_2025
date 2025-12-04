use std::str;

pub fn parse_input(data: &str) -> Vec<Vec<u8>> {
    data.lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}
