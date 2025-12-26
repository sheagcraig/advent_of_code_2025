use day_11::parse_input;
use std::collections::{HashMap, HashSet};

pub fn process(data: &str) -> usize {
    let devices = parse_input(data)
        .unwrap()
        .1
        .into_iter()
        .collect::<HashMap<_, _>>();

    // Cache key: (current_node, has_seen_fft, has_seen_dac)
    // Cache value: number of paths from this state to "out"
    let mut cache: HashMap<(String, bool, bool), usize> = HashMap::new();
    
    fn dfs(
        devices: &HashMap<&str, Vec<&str>>,
        start: &str,
        end: &str,
        has_fft: bool,
        has_dac: bool,
        visited: &mut HashSet<String>,
        cache: &mut HashMap<(String, bool, bool), usize>,
    ) -> usize {
        // Update flags if we're at fft or dac
        let has_fft = has_fft || start == "fft";
        let has_dac = has_dac || start == "dac";
        
        // Base case: reached the end
        if start == end {
            // Only count if we've seen both fft and dac
            return if has_fft && has_dac { 1 } else { 0 };
        }
        
        // Check cache
        let cache_key = (start.to_string(), has_fft, has_dac);
        if let Some(&cached_count) = cache.get(&cache_key) {
            return cached_count;
        }
        
        // Prevent cycles
        if visited.contains(start) {
            return 0;
        }
        visited.insert(start.to_string());
        
        let mut total_paths = 0;
        
        // Explore all neighbors
        if let Some(outputs) = devices.get(start) {
            for output in outputs {
                total_paths += dfs(devices, output, end, has_fft, has_dac, visited, cache);
            }
        }
        
        // Remove from visited to allow other paths to use this node
        visited.remove(start);
        
        // Cache the result
        cache.insert(cache_key, total_paths);
        
        total_paths
    }

    let mut visited = HashSet::new();
    dfs(&devices, "svr", "out", false, false, &mut visited, &mut cache)
}
#[cfg(test)]
mod test {
    use crate::part2::process;

    #[test]
    fn test_process() {
        let data = include_str!("sample_data_2.txt");
        assert_eq!(process(data), 2);
    }
}
