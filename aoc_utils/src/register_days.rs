#[macro_export]
macro_rules! register_days {
    ($($i:ident),*, $(,)?) => {
        $(mod $i;)*
        pub fn make_day_solutions() -> Vec<$crate::ExistentialDaySolution> {
            vec![
                $(
                    $crate::make_day_solution::<$i::Solution>(concat!(stringify!($i), ".rs")),
                )*
            ]
        }
    };
}
