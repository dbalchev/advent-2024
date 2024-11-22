#![allow(clippy::vec_init_then_push)]
use aoc_utils::{make_day_solution, DaySolution};

macro_rules! declare_days {
    () => {};
    ($i: ident) => {
        mod $i;
    };
    ($i: ident, $($is:ident),*) => {
        mod $i;
        declare_days!{$($is),*}
    };
}
macro_rules! make_day_solution_item {
    ($i: ident) => {
        make_day_solution(stringify!($i), $i::solve_1, $i::solve_2)
    };
}
macro_rules! make_day_solution_items {
    ($v: ident) => {};
    ($v: ident, $i: ident) => {
        $v.push(make_day_solution_item!($i));
    };
    ($v: ident, $i: ident, $($is:ident),*) => {
        $v.push(make_day_solution_item!($i));
        make_day_solution_items!($v, $($is),*)
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
register_days! {day_00}
