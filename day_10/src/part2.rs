use day_10::parse_input;
use std::collections::VecDeque;

pub fn process(data: &str) -> usize {
    let (_, (_, all_buttons, all_joltages)) = parse_input(data).unwrap();
    // println!("{&lights:?}");
    // println!("{&buttons:?}");
    let mut presses = 0;
    for (joltages, buttons) in all_joltages.into_iter().zip(all_buttons.into_iter()) {
        presses += bfs(joltages, buttons);
        println!("Completed one entry.");
    }
    presses
}

fn bfs(joltages: Vec<u16>, buttons: Vec<Vec<u8>>) -> usize {
    let mut queue = VecDeque::new();

    for (button_idx, _button) in buttons.iter().enumerate() {
        queue.push_front((button_idx, vec![0; joltages.len()], 0));
    }
    while !queue.is_empty() {
        let (head_idx, mut joltage_state, mut path) = queue.pop_back().unwrap();
        dbg!(&joltage_state);
        let head = &buttons[head_idx];
        head.iter().for_each(|&idx| {
            joltage_state[idx as usize] += 1;
        });
        path += 1;
        if joltage_state == joltages {
            return path;
        }
        // No need to continue on this path if joltages have exceeded
        // our desired state.
        if joltage_state
            .iter()
            .enumerate()
            .any(|(i, v)| v > &joltages[i])
        {
            continue;
        }

        for (next_button_idx, _next_button) in buttons.iter().enumerate() {
            queue.push_front((next_button_idx, joltage_state.clone(), path.clone()));
        }
    }
    0
}

#[cfg(test)]
mod test {
    use crate::part2::process;

    #[test]
    fn test_process() {
        let data = include_str!("sample_data_1.txt");
        assert_eq!(process(data), 33);
    }
}
