use day_03::parse_input;

pub fn process(data: &str) -> usize {
    let input = parse_input(data);
    dbg!(&input);
    // Now we need to turn on 12 instead of 2.
    // I think the algorithm is basically the same.
    // You can't start later than len - 12 because then you can't turn 12 on.
    // Then for the second digit you can't start later than len - 11, etc.
    // So I think I can generalize it to not do "first, second" but intead to
    // use a range and iterate.
    // 123,456,789,012 is hundred billions.
    // 1,000,000,000,000 = 10 ** 12
    // 100,000,000,000 = 10 ** 11. So 12 - i if we're doing 1..=12
    input.iter().fold(0, |acc, v| {
        (1..=12)
            .fold((0, 0), |(start_idx, inner_acc), i| {
                // dbg!(&v[start_idx..v.len() - (12 - i)]);
                let largest = v[start_idx..v.len() - (12 - i)].iter().max().unwrap();
                let idx = v[start_idx..].iter().position(|&i| i == *largest).unwrap();
                // let dbg_acc = (*largest as usize * usize::pow(10, 12 - i as u32)) + inner_acc;
                // dbg!((i, start_idx, idx, largest, dbg_acc));
                (
                    idx + start_idx + 1,
                    (*largest as usize * usize::pow(10, 12 - i as u32)) + inner_acc,
                )
            })
            .1
            + acc
    })
}
#[cfg(test)]
mod test {
    use crate::part2::process;

    #[test]
    fn test_process() {
        let data = include_str!("sample_data_1.txt");
        assert_eq!(process(data), 3121910778619);
    }
}
