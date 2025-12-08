use day_07::{parse_input, Map};

pub fn process(data: &str) -> usize {
    let map = parse_input(data);
    // dbg!(&map);

    let mut splits = 0;
    let width = map[0].len();
    let mut beams = vec![false; width];
    for row in map.iter() {
        for (idx, item) in row.iter().enumerate() {
            match item {
                Map::Start => beams[idx] = true,
                Map::Splitter => {
                    if beams[idx] {
                        splits += 1;
                        beams[idx] = false;
                    }
                    if idx > 0 {
                        beams[idx - 1] = true;
                    }
                    if idx < width {
                        beams[idx + 1] = true;
                    }
                }
                _ => {}
            }
        }
    }

    splits
}

#[cfg(test)]
mod test {
    use crate::part1::process;

    #[test]
    fn test_process() {
        let data = include_str!("sample_data_1.txt");
        assert_eq!(process(data), 21);
    }
}
