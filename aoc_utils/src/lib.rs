use std::error::Error;

mod day_solution;
mod format_struct;
mod input_paths;
mod parsable;

pub use crate::day_solution::{make_day_solution, DaySolution, ExistentialDaySolution};
pub use crate::input_paths::{make_real_path, make_sample_path};
pub use crate::parsable::{Parsable, ParseBuffer, SeparatorParsable};
pub use regex::Regex;
pub type MyResult<T> = Result<T, Box<dyn Error>>;
