use aoc_utils::{formatted_struct, MyResult};
use regex::Regex;

// pub type InputFormat = String;

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
            "-"
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
pub fn solve_1(input: &InputFormat) -> MyResult<String> {
    // let f = Foo {
    //     foo: "x".to_string(),
    //     bar: 3,
    // };
    Ok(format!("Hello {:?}", *input))
}

pub fn solve_2(_input: &InputFormat) -> MyResult<String> {
    Err("not implemented")?
}
