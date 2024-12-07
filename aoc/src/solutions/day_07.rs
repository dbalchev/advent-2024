use std::fmt::Debug;

use aoc_utils::{formatted_struct, DaySolution, MyResult};

formatted_struct! {
    #[derive(Debug)]
    pub struct Operator {
        expectation: i64,
        ": ",
        #[separated_by=" "]
        operands: Vec<i64>,
    }
}

formatted_struct! {
    #[derive(Debug)]
    pub struct InputFormat {
        #[separated_by="\n"]
        operators: Vec<Operator>,
    }
}

fn is_solvable(expectation: i64, operands: &[i64]) -> bool {
    if let &[last_operand] = operands {
        return last_operand == expectation;
    }
    let (&last, rest_operands) = operands.split_last().expect("operands shouldn't be empty");
    if expectation % last == 0 && is_solvable(expectation / last, rest_operands) {
        return true;
    }
    is_solvable(expectation - last, rest_operands)
}

pub struct Solution;

impl DaySolution for Solution {
    type InputFormat = InputFormat;
    fn solve_1(input: &InputFormat) -> MyResult<impl Debug + 'static> {
        let result = input
            .operators
            .iter()
            .filter_map(
                |Operator {
                     expectation,
                     operands,
                 }| {
                    if is_solvable(*expectation, operands) {
                        Some(*expectation)
                    } else {
                        None
                    }
                },
            )
            .sum::<i64>();
        Ok(result)
    }
}
