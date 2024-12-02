use std::{cmp::Ordering, fmt::Debug};

use aoc_utils::{formatted_struct, DaySolution, MyResult};

formatted_struct! {
    #[derive(Debug)]
    pub struct Report {
        #[separated_by=" +"]
        levels: Vec<i32>,
    }
}

formatted_struct! {
    #[derive(Debug)]
    pub struct InputFormat {
        #[separated_by="\n"]
        reports: Vec<Report>,
    }
}

impl Report {
    fn is_safe(&self) -> bool {
        let mut desired_ord = None;
        for (l, r) in self.levels.iter().zip(self.levels.iter().skip(1)) {
            let current_ord = l.cmp(r);
            if current_ord == Ordering::Equal {
                return false;
            }
            desired_ord = desired_ord.or(Some(current_ord));
            if desired_ord != Some(current_ord) {
                return false;
            }
            if l.abs_diff(*r) > 3 {
                return false;
            }
        }
        true
    }
}

pub struct Solution;

impl DaySolution for Solution {
    type InputFormat = InputFormat;
    fn solve_1(input: &InputFormat) -> MyResult<impl Debug + 'static> {
        let safe_count = input.reports.iter().filter(|x| x.is_safe()).count();
        Ok(safe_count)
    }
}
