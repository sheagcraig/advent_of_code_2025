use day_06::{parse_input, Op};

pub fn process(data: &str) -> usize {
    let (_, (digits, ops)) = parse_input(data).unwrap();
    // dbg!(&digits);
    // dbg!(&ops);
    ops.iter()
        .enumerate()
        .map(|(i, op)| match op {
            Op::Add => digits.iter().map(|r| r[i]).sum::<usize>(),
            Op::Mul => digits.iter().map(|r| r[i]).product(),
        })
        .sum()
}

#[cfg(test)]
mod test {
    use crate::part1::process;

    #[test]
    fn test_process() {
        let data = include_str!("sample_data_1.txt");
        assert_eq!(process(data), 4277556);
    }
}
