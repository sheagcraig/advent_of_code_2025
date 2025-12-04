use day_04::{parse_input, Map, SEARCH_DIRS};
use glam::IVec2;
use std::collections::HashMap;

pub fn process(data: &str) -> usize {
    let mut map = parse_input(data);
    // println!("{map:?}");
    let mut total_removed = 0;
    loop {
        let removed_count = remove_paper(&mut map);
        if removed_count == 0 {
            break;
        }
        total_removed += removed_count;
    }
    total_removed
}

fn remove_paper(map: &mut HashMap<IVec2, Map>) -> usize {
    let removed = map
        .iter()
        .filter(|(pos, v)| {
            *v == &Map::Paper
                && SEARCH_DIRS
                    .iter()
                    .filter(|&dir| map.get(&(*pos + dir)).is_some_and(|v| v == &Map::Paper))
                    .count()
                    < 4
        })
        .map(|(pos, _v)| *pos)
        .collect::<Vec<IVec2>>();
    let removed_count = removed.len();
    for removal in removed {
        map.entry(removal).insert_entry(Map::Floor);
    }
    removed_count
}

#[cfg(test)]
mod test {
    use crate::part2::process;

    #[test]
    fn test_process() {
        let data = include_str!("sample_data_1.txt");
        assert_eq!(process(data), 43);
    }
}
