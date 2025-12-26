use day_09::parse_input;
use itertools::Itertools;

pub fn process(data: &str) -> usize {
    let (_, input) = parse_input(data).unwrap();
    dbg!(&input);
    input
        .iter()
        .combinations(2)
        .map(|pts| ((pts[0].x - pts[1].x).abs() + 1) * ((pts[0].y - pts[1].y).abs() + 1))
        .max()
        .unwrap() as usize
}

#[cfg(test)]
mod test {
    use crate::part1::process;

    #[test]
    fn test_process() {
        let data = include_str!("sample_data_1.txt");
        assert_eq!(process(data), 50);
    }
}
