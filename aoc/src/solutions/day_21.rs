use std::{
    cmp::Ordering,
    collections::{hash_map::Entry, HashMap, HashSet},
    fmt::Debug,
    usize,
};

use aoc_utils::{formatted_struct, Chars, DaySolution, MyResult};

formatted_struct! {
    #[derive(Debug)]
    pub enum Instruction {
        Set {
            name:String,
            "=",
            value: i32,
        },
        Dash {
            name: String,
            "-",
        },
    }
}

formatted_struct! {
    #[derive(Debug)]
    pub struct InputFormat {
        #[separated_by="\n"]
        instructions: Vec<Chars>,
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

macro_rules! hashset {
    ($v:expr) => {{
        {
            let mut result = HashSet::new();
            result.insert($v);
            result
        }
    }};
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

            let connection = match (dx, dy) {
                (0, 0) => hashset!["".to_string()],
                (-1, 0) => hashset!['>'.to_string()],
                (1, 0) => hashset!['<'.to_string()],
                (0, -1) => hashset!['v'.to_string()],
                (0, 1) => hashset!['^'.to_string()],
                _ => continue,
            };
            result.insert((*i, *j), connection);
        }
    }
    let mut update_shortest_path = |i, j, k| {
        let ik = result.get(&(i, k))?.clone();
        let kj = result.get(&(k, j))?.clone();

        let add_ikj = |ij: &mut HashSet<String>| {
            for a in &ik {
                for b in &kj {
                    let mut r = String::with_capacity(a.len() + b.len());
                    r.extend(a.chars());
                    r.extend(b.chars());
                    ij.insert(r);
                }
            }
        };
        match result.entry((i, j)) {
            Entry::Occupied(mut occupied) => {
                let original = occupied.get_mut();
                match original
                    .iter()
                    .next()
                    .unwrap()
                    .len()
                    .cmp(&(ik.iter().next().unwrap().len() + kj.iter().next().unwrap().len()))
                {
                    Ordering::Less => return None,
                    Ordering::Greater => {
                        original.clear();
                    }
                    Ordering::Equal => (),
                }
                add_ikj(original);
            }
            Entry::Vacant(vacant) => add_ikj(vacant.insert(HashSet::new())),
        };

        Some(())
    };
    for &i in &chars {
        for &j in &chars {
            for &k in &chars {
                update_shortest_path(i, j, k);
            }
        }
    }
    result
        .into_iter()
        .map(|(k, v)| {
            let mut v = v.into_iter().collect::<Vec<_>>();
            v.sort();
            (k, v)
        })
        .collect::<HashMap<_, Vec<_>>>()
}

fn generate_collection_path_lengths(
    lower_path_lengths: &HashMap<(Vec<char>, Vec<char>), usize>,
    higher_paths: &HashMap<(char, char), Vec<String>>,
    all_higher_chars: &[char],
) -> HashMap<(Vec<char>, Vec<char>), usize> {
    let mut result = HashMap::new();
    let all_lower_paths = &lower_path_lengths
        .keys()
        .flat_map(|(a, b)| [a, b])
        .cloned()
        .collect::<HashSet<_>>();
    let mut lower_paths_ending_in = HashMap::with_capacity(all_higher_chars.len());
    for lower_path in all_lower_paths {
        lower_paths_ending_in
            .entry(*lower_path.last().unwrap())
            .or_insert_with(Vec::new)
            .push(lower_path);
    }
    for lower_path_start in all_lower_paths {
        for lower_path_end in all_lower_paths {
            for &higher_char_start in all_higher_chars {
                for &higher_char_end in all_higher_chars {
                    let mut best_path_length = usize::MAX;
                    for higher_path in higher_paths
                        .get(&(higher_char_start, higher_char_end))
                        .unwrap()
                    {
                        let mut current_state_lengths = HashMap::new();
                        current_state_lengths.insert(lower_path_start.clone(), 0);
                    }
                }
            }
        }
    }
    result
}

impl DaySolution for Solution {
    type InputFormat = InputFormat;
    fn solve_1(input: &InputFormat) -> MyResult<impl Debug + 'static> {
        let numpad_paths = generate_keyboard_paths(&NUMPAD);
        let dpad_paths = generate_keyboard_paths(&DPAD);
        let order_1_path_lengths = ();
        Ok(format!(
            "Hello {:?}",
            [numpad_paths.get(&('A', '7')), numpad_paths.get(&('7', 'A'))]
        ))
    }
    fn preferred_sample_input() -> i32 {
        3
    }
}
