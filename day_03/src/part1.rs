use day_03::parse_input;

pub fn process(data: &str) -> usize {
    let input = parse_input(data);
    dbg!(&input);
    input.iter().fold(0, |acc, v| {
        let first = v[..v.len() - 1].iter().max().unwrap();
        let first_idx = v.iter().position(|&i| i == *first).unwrap();
        let second = v[(first_idx + 1)..].iter().max().unwrap();
        *first as usize * 10 + *second as usize + acc
    })
}

#[cfg(test)]
mod test {
    use crate::part1::process;

    #[test]
    fn test_process() {
        let data = include_str!("sample_data_1.txt");
        assert_eq!(process(data), 357);
    }
}
