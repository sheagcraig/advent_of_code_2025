use day_07::{parse_input, Map};
use std::collections::HashMap;

pub fn process(data: &str) -> usize {
    let map = parse_input(data);
    // dbg!(&map);

    let start = map[0].iter().position(|m| m == &Map::Start).unwrap();
    fn dfs(
        row: usize,
        beam_index: usize,
        width: usize,
        map: &Vec<Vec<Map>>,
        cache: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        if row >= width {
            return 1;
        }
        if let Some(cached) = cache.get(&(row, beam_index)) {
            return *cached;
        }
        let mut paths = 0;
        match map[row][beam_index] {
            Map::Splitter => {
                paths += dfs(row + 1, beam_index - 1, width, map, cache);
                paths += dfs(row + 1, beam_index + 1, width, map, cache);
            }
            _ => paths += dfs(row + 1, beam_index, width, map, cache),
        }
        cache.insert((row, beam_index), paths);
        paths
    }

    let width = map.len();
    let mut cache = HashMap::new();

    dfs(0, start, width, &map, &mut cache)
}

#[cfg(test)]
mod test {
    use crate::part2::process;

    #[test]
    fn test_process() {
        let data = include_str!("sample_data_1.txt");
        assert_eq!(process(data), 40);
    }

    #[test]
    fn test_process_2splitters() {
        let data = r"
.......S.......
...............
.......^.......
...............
......^........
...............";
        assert_eq!(process(&data[1..]), 3);
    }

    #[test]
    fn test_process_3splitters() {
        let data = r"
.......S.......
...............
.......^.......
...............
......^.^......
...............";
        assert_eq!(process(&data[1..]), 4);
    }

    #[test]
    fn test_process_4splitters() {
        let data = r"
.......S.......
...............
.......^.......
...............
......^.^......
.....^.........";
        assert_eq!(process(&data[1..]), 5);
    }

    #[test]
    fn test_process_4splitters_shared() {
        let data = r"
.......S.......
...............
.......^.......
...............
......^.^......
.......^.......";
        assert_eq!(process(&data[1..]), 6);
    }
}
