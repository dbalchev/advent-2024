use std::{collections::HashMap, fmt::Debug};

use aoc_utils::{formatted_struct, DaySolution, MyResult};

// pub type InputFormat = String;

formatted_struct! {
    #[derive(Debug)]
    pub struct InputLine {
        first:i64,
        " +",
        second:i64,
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
            .map(|(first, second)| i64::abs(first - second))
            .sum::<i64>();
        Ok(delta_sum)
    }
    fn solve_2(input: &InputFormat) -> MyResult<impl Debug + 'static> {
        let mut second_counts = HashMap::<i64, i64>::new();
        let firsts = input.lines.iter().map(|l| l.first).collect::<Vec<_>>();
        for second in input.lines.iter().map(|l| l.second) {
            *second_counts.entry(second).or_insert(0) += 1;
        }
        let similarity_sum = firsts
            .into_iter()
            .map(|first| first * second_counts.get(&first).unwrap_or(&0))
            .sum::<i64>();
        Ok(similarity_sum)
    }
}
