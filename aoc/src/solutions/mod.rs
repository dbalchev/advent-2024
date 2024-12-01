#![allow(clippy::vec_init_then_push)]
use aoc_utils::{make_day_solution, ExistentialDaySolution};

macro_rules! register_days {
    ($($i:ident),*) => {
        $(mod $i;)*
        pub fn make_day_solutions() -> Vec<ExistentialDaySolution> {
            vec![
                $(
                    make_day_solution::<$i::Solution>(concat!(stringify!($i), ".rs")),
                )*
            ]
        }
    };
}
register_days! {day_00, day_01}
