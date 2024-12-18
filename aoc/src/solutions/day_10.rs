use std::{collections::HashSet, fmt::Debug};

use aoc_utils::{formatted_struct, Chars, DaySolution, MyResult};

formatted_struct! {
    #[derive(Debug)]
    pub struct InputFormat {
        #[separated_by="\n"]
        rows: Vec<Chars>,
    }
}

pub struct Solution;

impl DaySolution for Solution {
    type InputFormat = InputFormat;
    fn solve_1(input: &InputFormat) -> MyResult<impl Debug + 'static> {
        let mut reachable_ends = input
            .rows
            .iter()
            .map(|Chars(row)| row.iter().map(|_| HashSet::new()).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        for (i, row) in reachable_ends.iter_mut().enumerate() {
            for (j, c) in row.iter_mut().enumerate() {
                if input.rows[i].0[j] == '9' {
                    c.insert((i, j));
                }
            }
        }
        for current in ('0'..='8').rev() {
            let mut new_reachable_ends = input
                .rows
                .iter()
                .map(|Chars(row)| row.iter().map(|_| HashSet::new()).collect::<Vec<_>>())
                .collect::<Vec<_>>();
            for (i, row) in new_reachable_ends.iter_mut().enumerate() {
                for (j, c) in row.iter_mut().enumerate() {
                    if input.rows[i].0[j] == current {
                        for (di, dj) in [(0, 1), (0, -1), (-1, 0), (1, 0)] {
                            let ci = i as i32 + di;
                            let cj = j as i32 + dj;
                            let in_bounds = (0..(reachable_ends.len() as i32)).contains(&ci)
                                && (0..(reachable_ends[ci as usize].len() as i32)).contains(&cj);
                            if in_bounds {
                                c.extend(reachable_ends[ci as usize][cj as usize].iter());
                            }
                        }
                    }
                }
            }
            reachable_ends = new_reachable_ends;
        }

        // println!("{:?}", reachable_ends);
        Ok(reachable_ends
            .iter()
            .map(|r| r.iter().map(HashSet::len).sum::<usize>())
            .sum::<usize>())
    }

    fn solve_2(input: &InputFormat) -> MyResult<impl Debug + 'static> {
        let mut reachable_ends = input
            .rows
            .iter()
            .map(|Chars(row)| row.iter().map(|_| 0).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        for (i, row) in reachable_ends.iter_mut().enumerate() {
            for (j, c) in row.iter_mut().enumerate() {
                if input.rows[i].0[j] == '9' {
                    *c = 1;
                }
            }
        }
        for current in ('0'..='8').rev() {
            let mut new_reachable_ends = input
                .rows
                .iter()
                .map(|Chars(row)| row.iter().map(|_| 0).collect::<Vec<_>>())
                .collect::<Vec<_>>();
            for (i, row) in new_reachable_ends.iter_mut().enumerate() {
                for (j, c) in row.iter_mut().enumerate() {
                    if input.rows[i].0[j] == current {
                        for (di, dj) in [(0, 1), (0, -1), (-1, 0), (1, 0)] {
                            let ci = i as i32 + di;
                            let cj = j as i32 + dj;
                            let in_bounds = (0..(reachable_ends.len() as i32)).contains(&ci)
                                && (0..(reachable_ends[ci as usize].len() as i32)).contains(&cj);
                            if in_bounds {
                                *c += reachable_ends[ci as usize][cj as usize];
                            }
                        }
                    }
                }
            }
            reachable_ends = new_reachable_ends;
        }

        // println!("{:?}", reachable_ends);
        Ok(reachable_ends
            .iter()
            .map(|r| r.iter().sum::<usize>())
            .sum::<usize>())
    }
    fn preferred_sample_input() -> i32 {
        4
    }
}
