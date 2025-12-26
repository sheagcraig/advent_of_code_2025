use std::collections::VecDeque;

use day_10::parse_input;

pub fn process(data: &str) -> usize {
    let (_, (all_lights, all_buttons, _)) = parse_input(data).unwrap();
    // println!("{&lights:?}");
    // println!("{&buttons:?}");
    let mut presses = 0;
    for (lights, buttons) in all_lights.into_iter().zip(all_buttons.into_iter()) {
        presses += bfs(lights, buttons);
    }
    presses
}

fn bfs(lights: Vec<bool>, buttons: Vec<Vec<u8>>) -> usize {
    let mut queue = VecDeque::new();

    for button in buttons.iter() {
        queue.push_front((button, vec![false; lights.len()], vec![]));
    }
    // Maybe we don't _need_ cycle detection because we know that
    // every machine does have a shortest way to start up.
    while !queue.is_empty() {
        let (head, mut light_state, mut path) = queue.pop_back().unwrap();
        head.iter().for_each(|&idx| {
            light_state[idx as usize] ^= true;
        });
        path.push(head);
        if light_state == lights {
            return path.len();
        }

        for next_button in buttons.iter() {
            if !(head == next_button) {
                queue.push_front((next_button, light_state.clone(), path.clone()));
            }
        }
    }
    0
}

#[cfg(test)]
mod test {
    use crate::part1::process;

    #[test]
    fn test_process() {
        let data = include_str!("sample_data_1.txt");
        assert_eq!(process(data), 7);
    }
}
