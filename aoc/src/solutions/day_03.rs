use std::fmt::Debug;

use aoc_utils::{DaySolution, MyResult, Parsable};
use regex::Regex;

pub struct Solution;

impl DaySolution for Solution {
    type InputFormat = String;
    fn solve_1(input: &String) -> MyResult<impl Debug + 'static> {
        let instruction_matcher = Regex::new("mul\\((?<lh>\\d{1,3}),(?<rh>\\d{1,3})\\)")?;
        let matches = instruction_matcher
            .captures_iter(input)
            .map(|m| {
                let extract = |name| -> MyResult<_> {
                    let text = m.name(name).ok_or(format!("{} is missing", name))?.as_str();

                    Ok(i32::parse(text)?)
                };
                Ok(extract("lh")? * extract("rh")?)
            })
            .collect::<MyResult<Vec<_>>>()?;
        Ok(matches.into_iter().sum::<i32>())
    }
}
