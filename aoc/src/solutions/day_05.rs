use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

use aoc_utils::{formatted_struct, DaySolution, MyResult};

formatted_struct! {
    #[derive(Debug)]
    pub struct OrderingRule {
        page_before: i32,
        "\\|",
        page_after: i32,
    }
}

formatted_struct! {
    #[derive(Debug)]
    pub struct Update {
        #[separated_by=","]
        page_numbers: Vec<i32>,
    }
}

formatted_struct! {
    #[derive(Debug)]
    pub struct InputFormat {
        #[separated_by="\n"]
        ordering_rules: Vec<OrderingRule>,
        "\n\n",
        #[separated_by="\n"]
        updates : Vec<Update>,
    }
}

pub struct Solution;

impl DaySolution for Solution {
    type InputFormat = InputFormat;
    fn solve_1(input: &InputFormat) -> MyResult<impl Debug + 'static> {
        let mut page_to_pages_after = HashMap::new();
        for &OrderingRule {
            page_before,
            page_after,
        } in &input.ordering_rules
        {
            page_to_pages_after
                .entry(page_before)
                .or_insert_with(|| HashSet::new())
                .insert(page_after);
        }
        let check_updates = |&Update { ref page_numbers }| {
            let mut past_pages = HashSet::new();
            for &page in page_numbers {
                if let Some(pages_after) = page_to_pages_after.get(&page) {
                    if !past_pages.is_disjoint(pages_after) {
                        return false;
                    }
                }
                past_pages.insert(page);
            }
            return true;
        };
        let result = input
            .updates
            .iter()
            .filter(|u| check_updates(u))
            .map(|&Update { ref page_numbers }| page_numbers[page_numbers.len() / 2])
            .sum::<i32>();

        Ok(result)
    }
}
