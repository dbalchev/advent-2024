use std::error::Error;

mod aoc_main;
mod day_solution;
mod format_struct;
mod input_paths;
mod memo;
mod parsable;
mod register_days;

pub use crate::aoc_main::aoc_main;
pub use crate::day_solution::{make_day_solution, DaySolution, ExistentialDaySolution};
pub use crate::input_paths::{make_real_path, make_sample_path};
pub use crate::memo::make_recursive_fn;
pub use crate::parsable::{Parsable, ParseBuffer, SeparatorParsable};
pub use regex::Regex;
pub type MyResult<T> = Result<T, Box<dyn Error>>;
