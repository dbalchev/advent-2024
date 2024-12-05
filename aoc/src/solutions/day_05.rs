use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

use aoc_utils::{formatted_struct, DaySolution, MyResult, Parsable};

formatted_struct! {
    #[derive(Debug)]
    pub struct OrderingRule {
        page_before: i32,
        "\\|",
        page_after: i32,
    }
}

formatted_struct! {
    #[derive(Debug, Clone)]
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

pub struct ProcessedInput {
    page_to_pages_after: HashMap<i32, HashSet<i32>>,
    updates: Vec<Update>,
}

impl Parsable for ProcessedInput {
    fn parse(text: &str) -> MyResult<Self> {
        let InputFormat {
            updates,
            ordering_rules,
        } = InputFormat::parse(text)?;
        let mut page_to_pages_after = HashMap::new();

        for &OrderingRule {
            page_before,
            page_after,
        } in &ordering_rules
        {
            page_to_pages_after
                .entry(page_before)
                .or_insert_with(HashSet::new)
                .insert(page_after);
        }
        Ok(ProcessedInput {
            page_to_pages_after,
            updates,
        })
    }
}

impl ProcessedInput {
    fn check_update(&self, update: &Update) -> bool {
        let mut past_pages = HashSet::new();
        for &page in &update.page_numbers {
            if let Some(pages_after) = self.page_to_pages_after.get(&page) {
                if !past_pages.is_disjoint(pages_after) {
                    return false;
                }
            }
            past_pages.insert(page);
        }
        true
    }
    fn sort_update(&self, update: Update) -> Update {
        let Update { page_numbers } = update;
        let mut sorted_page_numbers = Vec::new();

        for current_page in page_numbers {
            let insert_index = sorted_page_numbers
                .iter()
                .enumerate()
                .filter_map(|(index, past_page)| {
                    let pages_after = self.page_to_pages_after.get(&current_page)?;
                    if pages_after.contains(past_page) {
                        Some(index)
                    } else {
                        None
                    }
                })
                .min()
                .unwrap_or(sorted_page_numbers.len());
            sorted_page_numbers.insert(insert_index, current_page);
        }
        Update {
            page_numbers: sorted_page_numbers,
        }
    }
}

pub struct Solution;

impl DaySolution for Solution {
    type InputFormat = ProcessedInput;
    fn solve_1(input: &ProcessedInput) -> MyResult<impl Debug + 'static> {
        let result = input
            .updates
            .iter()
            .filter(|u| input.check_update(u))
            .map(|Update { page_numbers }| page_numbers[page_numbers.len() / 2])
            .sum::<i32>();

        Ok(result)
    }
    fn solve_2(input: &ProcessedInput) -> MyResult<impl Debug + 'static> {
        let result = input
            .updates
            .iter()
            .filter(|u| !input.check_update(u))
            .cloned()
            .map(|u| input.sort_update(u))
            .map(|Update { page_numbers }| page_numbers[page_numbers.len() / 2])
            // .collect::<Vec<_>>();
            .sum::<i32>();

        Ok(result)
    }
}
