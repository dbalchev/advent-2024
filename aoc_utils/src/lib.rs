use std::error::Error;

mod day_solution;
mod format_struct;
mod parsable;
mod regex_utils;

pub use crate::day_solution::{make_day_solution, DaySolution};
pub use crate::parsable::{Parsable, ParseBuffer, SeparatorParsable};
pub use crate::regex_utils::make_regex;

pub type MyResult<T> = Result<T, Box<dyn Error>>;
