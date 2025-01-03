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

fn bad_level_index(levels: &[i32]) -> Option<usize> {
    let mut desired_ord = None;
    for (i, (l, r)) in levels.iter().zip(levels.iter().skip(1)).enumerate() {
        let current_ord = l.cmp(r);
        if current_ord == Ordering::Equal {
            return Some(i);
        }
        desired_ord = desired_ord.or(Some(current_ord));
        if desired_ord != Some(current_ord) {
            return Some(i);
        }
        if l.abs_diff(*r) > 3 {
            return Some(i);
        }
    }
    None
}

impl Report {
    fn is_safe(&self) -> bool {
        bad_level_index(&self.levels).is_none()
    }
    fn is_tolerably_safe(&self) -> bool {
        let Some(bad_index) = bad_level_index(&self.levels) else {
            return true;
        };
        let removal_candidates = [0, bad_index, bad_index + 1];
        for i in removal_candidates {
            let mut new_report_levels = self.levels.clone();
            new_report_levels.remove(i);
            if bad_level_index(&new_report_levels).is_none() {
                return true;
            }
        }
        false
    }
}

pub struct Solution;

impl DaySolution for Solution {
    type InputFormat = InputFormat;
    fn solve_1(input: &InputFormat) -> MyResult<impl Debug + 'static> {
        let safe_count = input.reports.iter().filter(|x| x.is_safe()).count();
        Ok(safe_count)
    }
    fn solve_2(input: &InputFormat) -> MyResult<impl Debug + 'static> {
        let safe_count = input
            .reports
            .iter()
            .filter(|x| x.is_tolerably_safe())
            .count();
        Ok(safe_count)
    }
}
