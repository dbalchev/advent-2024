use std::{fmt::Debug, sync::LazyLock};

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

fn expectation_after_concatenation(expectation: i64, operand: i64) -> Option<i64> {
    static POWERS_10: LazyLock<Vec<i64>> = LazyLock::new(|| {
        let mut result: Vec<i64> = vec![1];
        loop {
            let &last = result.last().expect("it's never empty");
            if let Some(next) = last.checked_mul(10) {
                result.push(next);
            } else {
                break;
            }
        }
        result
    });
    if expectation <= operand {
        return None;
    }
    if operand == 0 && expectation % 10 == 0 {
        return Some(expectation / 10);
    }
    let operand_power = match POWERS_10.binary_search(&operand) {
        Ok(power) => POWERS_10[power + 1],
        Err(next_power) => POWERS_10[next_power],
    };
    if expectation % operand_power == operand {
        Some(expectation / operand_power)
    } else {
        None
    }
}

fn is_solvable(expectation: i64, operands: &[i64], allow_concat: bool) -> bool {
    if let &[last_operand] = operands {
        return last_operand == expectation;
    }
    let (&last, rest_operands) = operands.split_last().expect("operands shouldn't be empty");
    if allow_concat {
        if let Some(new_expectation) = expectation_after_concatenation(expectation, last) {
            if is_solvable(new_expectation, rest_operands, true) {
                return true;
            }
        }
    }
    if expectation % last == 0 && is_solvable(expectation / last, rest_operands, allow_concat) {
        return true;
    }
    is_solvable(expectation - last, rest_operands, allow_concat)
}

impl InputFormat {
    fn solve(&self, allow_concat: bool) -> i64 {
        self.operators
            .iter()
            .filter_map(
                |Operator {
                     expectation,
                     operands,
                 }| {
                    if is_solvable(*expectation, operands, allow_concat) {
                        Some(*expectation)
                    } else {
                        None
                    }
                },
            )
            .sum::<i64>()
    }
}
pub struct Solution;

impl DaySolution for Solution {
    type InputFormat = InputFormat;
    fn solve_1(input: &InputFormat) -> MyResult<impl Debug + 'static> {
        Ok(input.solve(false))
    }
    fn solve_2(input: &InputFormat) -> MyResult<impl Debug + 'static> {
        Ok(input.solve(true))
    }
}

#[cfg(test)]
mod test {
    use crate::solutions::day_07::expectation_after_concatenation;

    #[test]
    fn test_concat_non_power() {
        assert_eq!(expectation_after_concatenation(12345, 345), Some(12));
    }

    #[test]
    fn test_concat_power() {
        assert_eq!(expectation_after_concatenation(123100, 100), Some(123));
    }

    #[test]
    fn test_concat_single() {
        assert_eq!(expectation_after_concatenation(1234, 4), Some(123));
    }
}
