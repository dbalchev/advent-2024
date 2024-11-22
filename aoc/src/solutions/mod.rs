#![allow(clippy::vec_init_then_push)]
use aoc_utils::{make_day_solution, DaySolution};

macro_rules! declare_days {
    ($($i:ident),*) => {
        $(mod $i;)*
    };
}
macro_rules! make_day_solution_items {
    ($v: ident, $($i:ident),*) => {
        $($v.push(make_day_solution(concat!(stringify!($i), ".rs"), $i::solve_1, $i::solve_2)));*
    };
}
macro_rules! register_days {
    ($($is:ident),*) => {
        declare_days! {$($is),*}
        pub fn make_day_solutions() -> Vec<DaySolution> {
            let mut result = Vec::new();
            make_day_solution_items!(result, $($is),*);
            result
        }
    };
}
// register_days! {day_00}
include! {concat!(env!("OUT_DIR"), "/days.fragment")}
