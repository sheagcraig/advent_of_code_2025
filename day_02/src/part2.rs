use day_02::parse_lists;

pub fn process(data: &str) -> usize {
    let ranges = parse_lists(data).unwrap();
    dbg!(&ranges);

    let mut invalid_total = 0;

    // Dynamic programming approach
    // 1188511880
    // 1 matches 1? Y
    // 1 matches 8? N
    // 11 m 88? N
    // 118 m 851 N
    // 11885 m 11880 N
    // but that's all easy to just ==
    //
    // 1188511885
    // 1 m 1 Y
    // 1 m 8 N
    // 11 m 88 N
    // 118 m 851 N
    // 1188 m 5118 N
    // 11885 m 11885 Y
    //
    // 1188511885                           /// len 10
    // S == s[0] * s.len() N                /// always possible
    // S = s[0..=1] * s.len() / sub_len       /// sub len 2, so 5 repeats
    // S = s[0..=2] * s.len() / sub_len N    /// sub len 3, len doesn't div by 3, so skip
    // S = s[0..=3] * s.len() / sub_len N    /// sub len 4, len doesn't div by 4, so skip
    // S = s[0..=4] * s.len() / sub_len N   /// sub_len 5, len DOES div by 5
    //
    // So algo is
    // 0. Prep a cache that, for a given len, tells you the largest divisor. E.g. len 9 is 3, 16 is 8, 27 is 9. This is the max that we will iterate.
    // 0. OR take square root of len. Iterate from 1 to that only for the len of the substring.
    //    If the total len doesn't divide by the substring len evenly, move to the next one.
    // 1. Get len once
    // 2. for sub_len in sqrt(len)
    //
    // Also, check: is the digit at least 2 char? Because there's no point; it can't repeat if it's only 1!

    for product_id_range in ranges.1 {
        for id in product_id_range.0..=product_id_range.1 {
            let str_id = format!("{id}");
            let len = str_id.len();
            // for factor in 1..=len.isqrt() {
            for factor in 1..=len / 2 {
                if len % factor != 0 {
                    continue;
                }
                let substr = str_id.get(0..factor).unwrap();
                if substr.repeat(len / factor) == str_id {
                    invalid_total += id;
                    dbg!(id);
                    break;
                }
            }
            // let (first, second) = str_id.split_at(str_id.len() / 2);
            // if first == second {
            //     invalid_total += id;
            // }
        }
    }
    invalid_total
}
#[cfg(test)]
mod test {
    use crate::part2::process;

    #[test]
    fn test_process() {
        let data = include_str!("sample_data_1.txt");
        assert_eq!(process(data), 4174379265);
    }
}
