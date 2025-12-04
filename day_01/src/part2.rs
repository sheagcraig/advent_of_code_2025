use day_01::{parse_lists, Direction};
use std::str;

pub fn process(data: &str) -> usize {
    let (_, parsed) = parse_lists(data).unwrap();
    let mut cur: isize = 50;
    let mut result: usize = 0;
    for (dir, val) in parsed {
        // Handle multiples of 100, as they will _always_ trigger a trip past
        // zero.
        result += (val / 100) as usize;

        // Now handle the sub-100 remainder
        let remainder = val % 100;
        match dir {
            Direction::Left => {
                if remainder > cur {
                    // This will cross zero, one time.
                    if cur != 0 {
                        result += 1;
                    }
                    let updated_val = remainder - cur;
                    cur = 100 - updated_val;
                } else if remainder == cur {
                    result += 1;
                    cur = 0;
                } else {
                    cur -= remainder;
                    if cur == 0 {
                        result += 1;
                    }
                }
            }
            Direction::Right => {
                let next = remainder + cur;
                result += (next / 100) as usize;
                cur = next % 100;
            }
        }
    }
    result
}

#[cfg(test)]
mod test {
    use crate::part2::process;

    #[test]
    fn test_process() {
        let data = include_str!("sample_data_1.txt");
        assert_eq!(process(data), 6);
    }
}
