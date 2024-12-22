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
    for (i, row) in keyboard.iter().enumerate() {
        for (j, button) in row.iter().enumerate() {
            char_to_coordinates.insert(*button, (i, j));
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

            let mut path = vec![if dx < 0 { '>' } else { '<' }; dx.unsigned_abs() as usize];
            path.resize(
                path.len() + dy.unsigned_abs() as usize,
                if dy < 0 { 'v' } else { '^' },
            );
            let ensure_no_gaps = |path| {
                let (mut y, mut x) = char_to_coordinates[i];
                let space_coords = char_to_coordinates[&' '];
                for &c in &path {
                    match c {
                        '>' => x += 1,
                        '<' => x -= 1,
                        '^' => y -= 1,
                        'v' => y += 1,
                        _ => panic!("{}", c),
                    };
                    if (y, x) == space_coords {
                        return None;
                    }
                }
                Some(String::from_iter(path))
            };

            result.insert(
                (*i, *j),
                permutations(&path)
                    .into_iter()
                    .filter_map(ensure_no_gaps)
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
            current_result.push_str(continuation);
            current_result.push('A');
            beam.push((next_button, remaining_path, current_result))
        }
    }
    result
}

fn cached_translate_path(
    paths: &[HashMap<String, i64>],
    paths_to_buttons: &HashMap<(char, char), Vec<String>>,
) -> Vec<HashMap<String, i64>> {
    paths
        .iter()
        .map(|segment_counts| {
            let mut result = HashMap::new();
            for (segment, count) in segment_counts {
                let translated = translate_path(&[segment.clone()], paths_to_buttons);
                assert_eq!(translated.len(), 1);
                for new_segment in translated[0].split_inclusive("A") {
                    *result.entry(new_segment.to_string()).or_insert(0) += count;
                }
            }
            result
        })
        .collect::<Vec<_>>()
}

fn prune_paths(
    paths_per_char_pair: &HashMap<(char, char), Vec<String>>,
    lower_paths: &HashMap<(char, char), Vec<String>>,
    prune_steps: i32,
) -> HashMap<(char, char), Vec<String>> {
    paths_per_char_pair
        .iter()
        .map(|(char_pair, paths)| {
            let best_paths = paths
                .iter()
                .cloned()
                .map(|path| {
                    let mut variants = translate_path(&[path.clone()], lower_paths);
                    for _ in 0..prune_steps {
                        variants = trim(variants, false);
                        variants = translate_path(&variants, lower_paths);
                    }
                    (path, variants.iter().map(String::len).min().unwrap())
                })
                .collect::<Vec<_>>();
            let optimum_length = best_paths.iter().map(|(_, l)| *l).min().unwrap();
            (
                *char_pair,
                best_paths
                    .into_iter()
                    .filter_map(|(path, len)| {
                        if len <= optimum_length {
                            Some(path)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<HashMap<_, _>>()
}

fn trim(result: Vec<String>, should_print: bool) -> Vec<String> {
    let min = result.iter().map(String::len).min().unwrap();
    let threshold = 5;
    let initial_size = result.len();
    let result = result
        .into_iter()
        .filter(|s| s.len() <= min + threshold)
        .collect::<Vec<_>>();
    if should_print {
        println!("trim {} to {}", initial_size, result.len());
    }
    result
}

fn general_solve(input: &InputFormat, n_indirections: i32) -> i64 {
    let numpad_paths = generate_keyboard_paths(&NUMPAD);
    let dpad_paths = generate_keyboard_paths(&DPAD);
    let dpad_paths = prune_paths(&dpad_paths, &dpad_paths, 1);
    // let dpad_paths = prune_paths(&dpad_paths, &dpad_paths, 2);
    let dpad_paths = prune_paths(&dpad_paths, &dpad_paths, 3);
    // let numpad_paths = prune_paths(&numpad_paths, &dpad_paths, 1);
    // let numpad_paths = prune_paths(&numpad_paths, &dpad_paths, 1);
    let mut sum = 0;
    for code in &input.instructions {
        let numpad_path = trim(translate_path(&[code.clone()], &numpad_paths), true);
        let resulting_paths = numpad_path;
        let mut resulting_paths = resulting_paths
            .into_iter()
            .map(|p| {
                let mut r = HashMap::new();
                r.insert(p, 1);
                r
            })
            .collect::<Vec<_>>();
        for _ in 0..n_indirections {
            resulting_paths = cached_translate_path(&resulting_paths, &dpad_paths);
        }
        let min_path = resulting_paths
            .into_iter()
            .map(|tp| tp.into_iter().map(|(s, c)| s.len() as i64 * c).sum::<i64>())
            .min()
            .unwrap();
        let num_code = code.trim_end_matches('A').parse::<i64>().unwrap();
        println!("{}", min_path);
        sum += min_path * num_code;
    }
    sum
}

impl DaySolution for Solution {
    type InputFormat = InputFormat;
    fn solve_1(input: &InputFormat) -> MyResult<impl Debug + 'static> {
        Ok(general_solve(input, 2))
    }
    fn solve_2(input: &InputFormat) -> MyResult<impl Debug + 'static> {
        Ok(general_solve(input, 25))
    }
    fn preferred_sample_input() -> i32 {
        3
    }
}
