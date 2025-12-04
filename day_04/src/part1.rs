use day_04::{parse_input, Map, SEARCH_DIRS};

pub fn process(data: &str) -> usize {
    let map = parse_input(data);
    // println!("{map:?}");
    map.iter()
        .filter(|(pos, v)| {
            *v == &Map::Paper
                && SEARCH_DIRS
                    .iter()
                    .filter(|&dir| map.get(&(*pos + dir)).is_some_and(|v| v == &Map::Paper))
                    .count()
                    < 4
        })
        .count()
}

#[cfg(test)]
mod test {
    use crate::part1::process;

    #[test]
    fn test_process() {
        let data = include_str!("sample_data_1.txt");
        assert_eq!(process(data), 13);
    }
}
