use day_09::parse_input;
use geo::{Contains, LineString, Polygon};
use itertools::Itertools;

pub fn process(data: &str) -> usize {
    let (_, input) = parse_input(data).unwrap();
    dbg!(&input);
    let polygon = Polygon::new(
        LineString::from(
            input
                .iter()
                .map(|pt| (pt.x as f64, pt.y as f64))
                .collect::<Vec<(f64, f64)>>(),
        ),
        vec![],
    );
    let rects: Vec<i64> = input
        .iter()
        .combinations(2)
        .filter_map(|pts| {
            let min_x = pts.iter().map(|p| p.x).min().unwrap();
            let max_x = pts.iter().map(|p| p.x).max().unwrap();
            let min_y = pts.iter().map(|p| p.y).min().unwrap();
            let max_y = pts.iter().map(|p| p.y).max().unwrap();
            let poly = Polygon::new(
                LineString::from(vec![
                    (min_x as f64, min_y as f64),
                    (max_x as f64, min_y as f64),
                    (max_x as f64, max_y as f64),
                    (min_x as f64, max_y as f64),
                    (min_x as f64, min_y as f64),
                ]),
                vec![],
            );
            let area = ((max_x - min_x).abs() + 1) * (max_y - min_y.abs() + 1);
            if polygon.contains(&poly) {
                Some(area)
            } else {
                None
            }
        })
        .sorted()
        .rev()
        .collect();
    *rects.get(0).unwrap() as usize
}
#[cfg(test)]
mod test {
    use crate::part2::process;

    #[test]
    fn test_process() {
        let data = include_str!("sample_data_1.txt");
        assert_eq!(process(data), 24);
    }
}
