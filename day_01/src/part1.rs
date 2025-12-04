use day_01::{parse_lists, Direction};
use std::str;

pub fn process(data: &str) -> usize {
    let (_, parsed) = parse_lists(data).unwrap();
    println!("{:?}", parsed);
    let mut cur: isize = 50;
    let mut result: usize = 0;
    for (dir, val) in parsed {
        let delta = match dir {
            Direction::Left => -val,
            Direction::Right => val,
        };
        cur += delta;
        if cur % 100 == 0 {
            result += 1;
        }
    }
    result
}

#[cfg(test)]
mod test {
    use crate::part1::process;

    #[test]
    fn test_process() {
        let data = include_str!("sample_data_1.txt");
        assert_eq!(process(data), 3);
    }
}
