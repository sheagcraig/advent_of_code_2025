use day_11::parse_input;
use std::collections::HashMap;

pub fn process(data: &str) -> usize {
    let devices = parse_input(data)
        .unwrap()
        .1
        .into_iter()
        .collect::<HashMap<_, _>>();
    // dbg!(&devices);
    let mut cache: HashMap<String, usize> = HashMap::new();

    fn dfs(
        devices: &HashMap<&str, Vec<&str>>,
        start: &str,
        end: &str,
        seen: &mut HashMap<String, usize>,
    ) -> usize {
        devices.get(start).unwrap().iter().fold(0, |acc, output| {
            if *output == end {
                return 1;
            }
            let paths = dfs(devices, output, end, seen);
            seen.insert(output.to_string(), paths.clone());
            acc + paths
        })
    }

    dfs(&devices, "you", "out", &mut cache)
}

#[cfg(test)]
mod test {
    use crate::part1::process;

    #[test]
    fn test_process() {
        let data = include_str!("sample_data_1.txt");
        assert_eq!(process(data), 5);
    }
}
