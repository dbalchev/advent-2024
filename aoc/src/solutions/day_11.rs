use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

use aoc_utils::{formatted_struct, DaySolution, MyResult};

formatted_struct! {
    #[derive(Debug)]
    pub struct InputFormat {
        #[separated_by=" "]
        stones: Vec<String>,
    }
}

pub struct Solution;

fn blink(old_stones: &[String]) -> Vec<String> {
    let mut result = Vec::new();
    for stone in old_stones {
        let new_stones: &[&str] = match stone.as_str() {
            "0" => &["1"],
            x if x.len() % 2 == 0 => {
                let half = x.len() / 2;
                &[&x[..half], &x[half..]]
            }
            _ => {
                let x = stone.parse::<i64>().unwrap();
                &[&format!("{}", x * 2024)]
            }
        };
        result.extend(new_stones.iter().map(|&x| {
            let mut x = x.trim_start_matches('0');
            if x.is_empty() {
                x = "0";
            }
            x.to_string()
        }));
    }
    result
}

fn generate_all_stone_transitions(
    initial_stones: &[String],
) -> HashMap<String, Vec<(String, i64)>> {
    let mut stones_left = initial_stones.to_vec();
    let mut stones_met: HashSet<String> = HashSet::from_iter(initial_stones.iter().cloned());
    let mut result = HashMap::new();

    while let Some(stone) = stones_left.pop() {
        let new_stones = blink(&[stone.clone()]);
        let mut next_stones = HashMap::new();
        for new_stone in new_stones {
            *next_stones.entry(new_stone.clone()).or_insert(0) += 1i64;
            if stones_met.contains(&new_stone) {
                continue;
            }
            stones_met.insert(new_stone.clone());
            stones_left.push(new_stone);
        }
        let old = result.insert(stone, next_stones.into_iter().collect::<Vec<_>>());
        assert!(old.is_none());
    }
    result
}

fn solve(initial_stones: &[String], n_blinks: i32) -> i64 {
    let transitions = generate_all_stone_transitions(initial_stones);
    let mut current_stones = HashMap::new();

    for stone in initial_stones {
        *current_stones.entry(stone.clone()).or_insert(0) += 1i64;
    }
    for _ in 0..n_blinks {
        let mut new_stones = HashMap::new();
        for (old_stone, old_count) in current_stones {
            for (new_stone, new_count) in transitions.get(&old_stone).unwrap() {
                *new_stones.entry(new_stone.clone()).or_insert(0) += old_count * new_count;
            }
        }
        current_stones = new_stones;
    }
    current_stones.values().sum::<i64>()
}

impl DaySolution for Solution {
    type InputFormat = InputFormat;
    fn solve_1(input: &InputFormat) -> MyResult<impl Debug + 'static> {
        Ok(solve(&input.stones, 25))
    }
    fn solve_2(input: &InputFormat) -> MyResult<impl Debug + 'static> {
        Ok(solve(&input.stones, 75))
    }
    fn preferred_sample_input() -> i32 {
        99
    }
}
