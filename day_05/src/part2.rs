use day_05::parse_input;
use std::ops::RangeInclusive;

pub fn process(data: &str) -> usize {
    let (mut ingredient_ids, _available_ingredients) = parse_input(data).unwrap().1;
    let mut found_overlaps = true;
    loop {
        let mut temp_fresh_ingredient_ranges: Vec<RangeInclusive<usize>> = Vec::new();
        for range in ingredient_ids {
            if let Some(found) = temp_fresh_ingredient_ranges
                .iter()
                .position(|expanded_range| {
                    expanded_range.contains(range.start())
                        || expanded_range.contains(range.end())
                        || range.contains(expanded_range.start())
                        || range.contains(expanded_range.end())
                })
            {
                // Found an overlap; merge the two
                temp_fresh_ingredient_ranges[found] = (*temp_fresh_ingredient_ranges[found]
                    .start()
                    .min(range.start()))
                    ..=(*temp_fresh_ingredient_ranges[found].end().max(range.end()));
                found_overlaps = true;
            } else {
                temp_fresh_ingredient_ranges.push(range)
            };
        }
        ingredient_ids = temp_fresh_ingredient_ranges;
        if !found_overlaps {
            break;
        }
        found_overlaps = false;
    }
    ingredient_ids.iter().map(|r| r.end() - r.start() + 1).sum()
}

#[cfg(test)]
mod test {
    use crate::part2::process;

    #[test]
    fn test_process() {
        let data = include_str!("sample_data_1.txt");
        assert_eq!(process(data), 14);
    }
}
