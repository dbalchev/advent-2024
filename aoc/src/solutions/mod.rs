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
    () => {};
    ($i: ident) => {
        make_day_solution_item!($i)
    };
    ($i: ident, $($is:ident),*) => {
        make_day_solution_item!($i), make_day_solution_items!($($is),*)
    };
}
macro_rules! register_days {
    ($($is:ident),*) => {
        declare_days! {$($is),*}
        pub fn make_day_solutions() -> Vec<DaySolution> {
            vec![make_day_solution_items!($($is),*)]
        }
    };
}
register_days! {day_00}
