use std::fmt::Debug;

use aoc_utils::{formatted_struct, DaySolution, MyResult};

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
        #[separated_by=","]
        instructions: Vec<Instruction>,
    }
}

pub struct Solution;

impl DaySolution for Solution {
    type InputFormat = InputFormat;
    fn solve_1(input: &InputFormat) -> MyResult<impl Debug + 'static> {
        Ok(format!("Hello {:?}", *input))
    }
}
