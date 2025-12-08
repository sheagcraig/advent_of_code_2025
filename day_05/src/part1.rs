use day_05::parse_input;

pub fn process(data: &str) -> usize {
    let (ingredient_ids, available_ingredients) = parse_input(data).unwrap().1;
    // dbg!(&ingredient_ids);
    // dbg!(&available_ingredients);
    available_ingredients
        .iter()
        .filter(|i| ingredient_ids.iter().any(|r| r.contains(i)))
        .count()
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
