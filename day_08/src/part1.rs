use glam::IVec3;
use std::collections::HashSet;

use day_08::parse_input;

pub fn process(data: &str, connections: usize) -> usize {
    let junction_boxes = parse_input(data).unwrap().1;
    // dbg!(&junction_boxes);
    let mut circuits: Vec<HashSet<IVec3>> = junction_boxes
        .iter()
        .map(|bx| HashSet::from([*bx]))
        .collect();

    // Pre-compute all pairs sorted by distance
    let mut all_pairs: Vec<(i64, IVec3, IVec3)> = Vec::new();
    for i in 0..junction_boxes.len() {
        for j in (i + 1)..junction_boxes.len() {
            let box1 = junction_boxes[i];
            let box2 = junction_boxes[j];
            // Calculate distance squared using i64 to avoid overflow
            let dx = (box1.x - box2.x) as i64;
            let dy = (box1.y - box2.y) as i64;
            let dz = (box1.z - box2.z) as i64;
            let dist = dx * dx + dy * dy + dz * dz;
            all_pairs.push((dist, box1, box2));
        }
    }
    all_pairs.sort_by_key(|&(dist, _, _)| dist);

    // Process the N closest pairs (even if already connected)
    for (pairs_processed, (_dist, bx1, bx2)) in all_pairs.into_iter().enumerate() {
        if pairs_processed >= connections {
            break;
        }

        // dbg!(((dist as f32).sqrt(), bx1, bx2));

        // Check if already connected
        let bx1_circuit_idx = circuits
            .iter()
            .position(|boxes| boxes.contains(&bx1))
            .unwrap();
        let bx2_circuit_idx = circuits
            .iter()
            .position(|boxes| boxes.contains(&bx2))
            .unwrap();

        if bx1_circuit_idx != bx2_circuit_idx {
            // Merge circuits
            if bx1_circuit_idx < bx2_circuit_idx {
                let bx2_circuit = circuits.remove(bx2_circuit_idx);
                circuits[bx1_circuit_idx].extend(bx2_circuit);
            } else {
                let bx1_circuit = circuits.remove(bx1_circuit_idx);
                circuits[bx2_circuit_idx].extend(bx1_circuit);
            }
        }
    }

    let mut circuit_sizes = circuits.iter().map(|c| c.len()).collect::<Vec<usize>>();
    circuit_sizes.sort();

    // dbg!(&circuit_sizes);
    circuit_sizes[circuit_sizes.len() - 3..].iter().product()
}

#[cfg(test)]
mod test {
    use crate::part1::process;

    #[test]
    fn test_process() {
        let data = include_str!("sample_data_1.txt");
        assert_eq!(process(data, 10), 40);
    }
}
