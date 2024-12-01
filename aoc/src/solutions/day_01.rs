use std::fmt::Debug;

use aoc_utils::{formatted_struct, DaySolution, MyResult};

// pub type InputFormat = String;

formatted_struct! {
    #[derive(Debug)]
    pub struct InputLine {
        first:i32,
        " +",
        second:i32,
    }
}

formatted_struct! {
    #[derive(Debug)]
    pub struct InputFormat {
        #[separated_by="\n"]
        lines: Vec<InputLine>,
    }
}

pub struct Solution;

impl DaySolution for Solution {
    type InputFormat = InputFormat;
    fn solve_1(input: &InputFormat) -> MyResult<impl Debug + 'static> {
        let mut firsts = input.lines.iter().map(|l| l.first).collect::<Vec<_>>();
        let mut seconds = input.lines.iter().map(|l| l.second).collect::<Vec<_>>();
        firsts.sort();
        seconds.sort();
        let delta_sum = firsts
            .into_iter()
            .zip(seconds.into_iter())
            .map(|(first, second)| i32::abs(first - second))
            .sum::<i32>();
        Ok(delta_sum)
    }
}
