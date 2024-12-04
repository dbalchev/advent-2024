use std::fmt::Debug;

use aoc_utils::{formatted_struct, DaySolution, MyResult};

formatted_struct! {
    #[derive(Debug)]
    pub struct InputFormat {
        #[separated_by="\n"]
        lines: Vec<String>,
    }
}

fn is_xmas(grid: &[String], (i, j): (i32, i32), (di, dj): &(i32, i32)) -> bool {
    for (l, &byte) in b"XMAS".into_iter().enumerate() {
        let l = l as i32;
        let ci = i + di * l;
        let cj = j + dj * l;
        if !(0..(grid.len() as i32)).contains(&ci) {
            return false;
        }
        let row = grid[ci as usize].as_bytes();
        if !(0..(row.len() as i32)).contains(&cj) {
            return false;
        }
        if row[cj as usize] != byte {
            return false;
        }
    }
    return true;
}

fn make_directions() -> Vec<(i32, i32)> {
    let mut result = Vec::with_capacity(8);
    for i in [-1, 0, 1] {
        for j in [-1, 0, 1] {
            if i == 0 && j == 0 {
                continue;
            }
            result.push((i, j));
        }
    }
    result
}

pub struct Solution;

impl DaySolution for Solution {
    type InputFormat = InputFormat;
    fn solve_1(input: &InputFormat) -> MyResult<impl Debug + 'static> {
        let grid = &input.lines;
        let directions = make_directions();
        let mut num_xmas = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                for direction in &directions {
                    num_xmas += is_xmas(grid, (i as i32, j as i32), direction) as i32;
                }
            }
        }
        Ok(num_xmas)
    }
    fn preferred_sample_input() -> i32 {
        1
    }
}
