pub fn process(data: &str) -> usize {
    let mut rows = data
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let ops = rows.pop().unwrap();
    let mut cur_op: Option<char> = None;
    let mut op_total = 0;
    let mut total = 0;
    for col in 0..rows.iter().map(|r| r.len()).max().unwrap() {
        let number_string: String = rows
            .iter()
            .map(|row| row.get(col).unwrap_or(&' '))
            .collect();
        let number = number_string.trim().parse::<usize>();
        match ops.get(col).unwrap_or(&' ') {
            '*' => {
                cur_op = Some('*');
                total += op_total;
                op_total = number.unwrap_or(1);
            }
            '+' => {
                cur_op = Some('+');
                total += op_total;
                op_total = number.unwrap_or(0);
            }
            _ => {
                op_total = match cur_op {
                    Some('*') => op_total * number.unwrap_or(1),
                    Some('+') => op_total + number.unwrap_or(0),
                    _ => panic!("Invalid operator"),
                }
            }
        }
    }
    total + op_total
}
#[cfg(test)]
mod test {
    use crate::part2::process;

    #[test]
    fn test_process() {
        let data = include_str!("sample_data_1.txt");
        assert_eq!(process(data), 3263827);
    }
}
