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
    fn solve_2(input: &String) -> MyResult<impl Debug + 'static> {
        let instruction_matcher = Regex::new(
            "(?<do>do\\(\\))|(?<dont>don't\\(\\))|mul\\((?<lh>\\d{1,3}),(?<rh>\\d{1,3})\\)",
        )?;
        let mut enabled = true;
        let matches = instruction_matcher
            .captures_iter(input)
            .map(|m| {
                if m.name("do").is_some() {
                    enabled = true;
                    return Ok(0);
                }
                if m.name("dont").is_some() {
                    enabled = false;
                    return Ok(0);
                }
                if !enabled {
                    return Ok(0);
                }
                let extract = |name| -> MyResult<_> {
                    let text = m.name(name).ok_or(format!("{} is missing", name))?.as_str();

                    Ok(i32::parse(text)?)
                };
                Ok(extract("lh")? * extract("rh")?)
            })
            .collect::<MyResult<Vec<_>>>()?;
        // Ok(format!("{:?}", matches))
        Ok(matches.into_iter().sum::<i32>())
    }
    fn preferred_sample_input() -> i32 {
        1
    }
}
