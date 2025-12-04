use day_02::parse_lists;

pub fn process(data: &str) -> usize {
    let ranges = parse_lists(data).unwrap();
    dbg!(&ranges);

    // let mut invalid_total = 0;

    // for product_id_range in ranges.1 {
    //     for id in (product_id_range.0..=product_id_range.1) {
    //         let str_id = format!("{id}");
    //         if str_id.len() % 2 != 0 {
    //             continue;
    //         }
    //         let (first, second) = str_id.split_at(str_id.len() / 2);
    //         if first == second {
    //             invalid_total += id;
    //         }
    //     }
    // }
    // invalid_total
    ranges
        .1
        .iter()
        .map(|product_id_range| {
            (product_id_range.0..=product_id_range.1)
                .filter(|id| {
                    let id_str = format!("{id}");
                    if id_str.len() % 2 == 0 {
                        let (first, second) = id_str.split_at(id_str.len() / 2);
                        return first == second;
                    }
                    false
                })
                .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use crate::part1::process;

    #[test]
    fn test_process() {
        let data = include_str!("sample_data_1.txt");
        dbg!(data);
        assert_eq!(process(data), 1227775554);
    }
}
