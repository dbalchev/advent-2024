use std::fmt::Debug;

use aoc_utils::{formatted_struct, DaySolution, MyResult};

formatted_struct! {
    #[derive(Debug)]
    pub struct InputFormat {
        #[separated_by="\n"]
        initial_secrets: Vec<i64>,
    }
}

pub struct Solution;

const MODULO: i64 = 16_777_216;

fn evolve(mut x: i64) -> i64 {
    x = (x ^ (x * 64)) % MODULO;
    x = (x ^ (x / 32)) % MODULO;
    x = (x ^ (x * 2048)) % MODULO;
    x
}

impl DaySolution for Solution {
    type InputFormat = InputFormat;
    fn solve_1(input: &InputFormat) -> MyResult<impl Debug + 'static> {
        let mut sum = 0;
        for &initial_secret in &input.initial_secrets {
            let mut x = initial_secret;
            for _ in 0..2000 {
                x = evolve(x);
            }
            sum += x;
        }
        Ok(sum)
    }
    fn preferred_sample_input() -> i32 {
        1
    }
}
