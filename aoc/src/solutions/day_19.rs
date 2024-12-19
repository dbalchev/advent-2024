use std::fmt::Debug;

use aoc_utils::{formatted_struct, DaySolution, MyResult};

formatted_struct! {
    #[derive(Debug)]
    pub struct InputFormat {
        #[separated_by=", "]
        towels: Vec<String>,
        "\n\n",
        #[separated_by="\n"]
        designs: Vec<String>,
    }
}

pub struct Solution;

fn count_possibilities(design: &str, towels: &[String]) -> i64 {
    let mut prefix_possible = Vec::with_capacity(design.len() + 1);
    prefix_possible.push(1);
    for prefix_len in 1..=design.len() {
        let prefix = &design[..prefix_len];
        prefix_possible.push(
            towels
                .iter()
                .map(|towel| {
                    if prefix.ends_with(towel) {
                        prefix_possible[prefix_len - towel.len()]
                    } else {
                        0
                    }
                })
                .sum::<i64>(),
        );
    }
    // println!("{:?}", prefix_possible);
    *prefix_possible.last().unwrap()
}

impl DaySolution for Solution {
    type InputFormat = InputFormat;
    fn solve_1(input: &InputFormat) -> MyResult<impl Debug + 'static> {
        Ok(input
            .designs
            .iter()
            .map(|d| (count_possibilities(d, &input.towels) > 0) as i32)
            .sum::<i32>())
    }
    fn solve_2(input: &InputFormat) -> MyResult<impl Debug + 'static> {
        Ok(input
            .designs
            .iter()
            .map(|d| count_possibilities(d, &input.towels))
            .sum::<i64>())
    }
}
