use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};

use aoc_utils::{formatted_struct, DaySolution, MyResult};

formatted_struct! {
    #[derive(Debug)]
    pub struct InputFormat {
        #[separated_by="\n"]
        instructions: Vec<String>,
    }
}

pub struct Solution;

const NUMPAD: [[char; 3]; 4] = [
    ['7', '8', '9'],
    ['4', '5', '6'],
    ['1', '2', '3'],
    [' ', '0', 'A'],
];

const DPAD: [[char; 3]; 2] = [[' ', '^', 'A'], ['<', 'v', '>']];

fn permutations<T: Clone + Eq + Hash>(items: &[T]) -> Vec<Vec<T>> {
    let mut beam = vec![(items, Vec::new())];
    let mut result = HashSet::new();

    while let Some((remaining, local_result)) = beam.pop() {
        let (first, next) = match remaining.split_first() {
            Some((first, next)) => (first, next),
            None => {
                result.insert(local_result);
                continue;
            }
        };
        for insertion in 0..=(local_result.len()) {
            let mut local_result = local_result.clone();
            local_result.insert(insertion, first.clone());
            beam.push((next, local_result));
        }
    }

    result.into_iter().collect::<Vec<_>>()
}

fn generate_keyboard_paths<const N: usize, const M: usize>(
    keyboard: &[[char; M]; N],
) -> HashMap<(char, char), Vec<String>> {
    let mut char_to_coordinates = HashMap::with_capacity(N * M);
    for i in 0..N {
        for j in 0..M {
            char_to_coordinates.insert(keyboard[i][j], (i, j));
        }
    }
    let char_to_coordinates = char_to_coordinates;
    let chars = keyboard
        .iter()
        .flatten()
        .cloned()
        .filter(|&c| c != ' ')
        .collect::<Vec<_>>();
    // let mut direct_connections = HashMap::new();

    let mut result = HashMap::with_capacity(N * M);
    for i in &chars {
        for j in &chars {
            let dy = char_to_coordinates[i].0 as i32 - char_to_coordinates[j].0 as i32;
            let dx = char_to_coordinates[i].1 as i32 - char_to_coordinates[j].1 as i32;

            let mut path = Vec::new();
            if dx < 0 {
                for _ in 0..(-dx) {
                    path.push('>');
                }
            } else if dx > 0 {
                for _ in 0..(dx) {
                    path.push('<');
                }
            }

            if dy < 0 {
                for _ in 0..(-dy) {
                    path.push('v');
                }
            } else {
                for _ in 0..dy {
                    path.push('^');
                }
            }

            result.insert(
                (*i, *j),
                permutations(&path)
                    .into_iter()
                    .map(|v| String::from_iter(&v))
                    .collect::<Vec<_>>(),
            );
        }
    }
    result
}

fn translate_path(
    paths: &[String],
    paths_to_buttons: &HashMap<(char, char), Vec<String>>,
) -> Vec<String> {
    let mut beam = paths
        .iter()
        .map(|path| ('A', path.as_str(), String::new()))
        .collect::<Vec<_>>();
    let mut result = Vec::new();
    while let Some((current_char, remaining_path, current_result)) = beam.pop() {
        if remaining_path.is_empty() {
            result.push(current_result);
            continue;
        }
        let (current_str, remaining_path) = remaining_path.split_at(1);
        let next_button = current_str.chars().next().unwrap();
        for continuation in paths_to_buttons.get(&(current_char, next_button)).unwrap() {
            let mut current_result = current_result.clone();
            current_result.extend(continuation.chars());
            current_result.push('A');
            beam.push((next_button, remaining_path, current_result))
        }
    }
    result
}

fn trim(result: Vec<String>) -> Vec<String> {
    let min = result.iter().map(String::len).min().unwrap();
    let threshold = 0;
    // let initial_size = result.len();
    let result = result
        .into_iter()
        .filter(|s| s.len() <= min + threshold)
        .collect::<Vec<_>>();
    // println!("trim {} to {}", initial_size, result.len());
    result
}

impl DaySolution for Solution {
    type InputFormat = InputFormat;
    fn solve_1(input: &InputFormat) -> MyResult<impl Debug + 'static> {
        let numpad_paths = generate_keyboard_paths(&NUMPAD);
        let dpad_paths = generate_keyboard_paths(&DPAD);
        let mut sum = 0;
        for code in &input.instructions {
            let numpad_path = trim(translate_path(&[code.clone()], &numpad_paths));
            // println!(
            //     "numpad_path {} {}",
            //     numpad_path.len(),
            //     numpad_path.iter().map(String::len).min().unwrap()
            // );
            let dpad_path_1 = trim(translate_path(&numpad_path, &dpad_paths));
            // println!(
            //     "dpad_path_1 {} {}",
            //     dpad_path_1.len(),
            //     dpad_path_1.iter().map(String::len).min().unwrap()
            // );
            let dpad_path_2 = trim(translate_path(&dpad_path_1, &dpad_paths));
            // println!(
            //     "dpad_path_2 {} {}",
            //     dpad_path_2.len(),
            //     dpad_path_2.iter().map(String::len).min().unwrap()
            // );
            // let computed_paths = [numpad_path, dpad_path_1, dpad_path_2];
            let min_path = dpad_path_2.iter().map(String::len).min().unwrap();
            let num_code = code.trim_end_matches('A').parse::<i32>()?;
            println!("{}", min_path);
            sum += min_path as i32 * num_code;
        }
        Ok(sum)
    }
    fn preferred_sample_input() -> i32 {
        3
    }
}
