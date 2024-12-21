use std::fmt::Debug;

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

impl DaySolution for Solution {
    type InputFormat = InputFormat;
    fn solve_1(input: &InputFormat) -> MyResult<impl Debug + 'static> {
        let mut stones = input.stones.clone();
        for _ in 0..25 {
            stones = blink(&stones);
        }
        Ok(stones.len())
    }
    fn preferred_sample_input() -> i32 {
        99
    }
}
