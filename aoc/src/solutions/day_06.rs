use std::{collections::HashSet, fmt::Debug};

use aoc_utils::{formatted_struct, DaySolution, MyResult, Parsable};
use std::error::Error;
formatted_struct! {
    #[derive(Debug)]
    pub struct RawInputFormat {
        #[separated_by="\n"]
        grid: Vec<String>,
    }
}
#[derive(Debug)]
pub struct ProcessedInputFormat {
    grid: Vec<Vec<u8>>,
    starting_pos: (usize, usize),
}

impl Parsable for ProcessedInputFormat {
    fn parse(text: &str) -> MyResult<Self> {
        let raw = RawInputFormat::parse(text)?;
        let mut grid = raw
            .grid
            .into_iter()
            .map(String::into_bytes)
            .collect::<Vec<_>>();
        let starting_pos = grid
            .iter()
            .enumerate()
            .filter_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(j, &byte)| if byte == b'^' { Some((i, j)) } else { None })
                    .next()
            })
            .next()
            .ok_or_else(|| -> Box<dyn Error> { From::from("No starting position found") })?;
        grid[starting_pos.0][starting_pos.1] = b'.';
        Ok(ProcessedInputFormat { grid, starting_pos })
    }
}

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

pub struct Solution;

impl DaySolution for Solution {
    type InputFormat = ProcessedInputFormat;
    fn solve_1(input: &ProcessedInputFormat) -> MyResult<impl Debug + 'static> {
        let ProcessedInputFormat {
            grid,
            starting_pos: (si, sj),
        } = input;
        let mut current_pos = (*si as isize, *sj as isize);
        let mut current_dir = 0;
        let mut position_history = HashSet::new();
        position_history.insert(current_pos);
        loop {
            let (i, j) = current_pos;
            let (di, dj) = DIRECTIONS[current_dir];
            let ni = i + di;
            let nj = j + dj;
            if !(0..grid.len() as isize).contains(&ni)
                || !(0..grid[ni as usize].len() as isize).contains(&nj)
            {
                break;
            }
            if grid[ni as usize][nj as usize] != b'.' {
                current_dir = (current_dir + 1) % 4;
                continue;
            }
            current_pos = (ni, nj);
            position_history.insert(current_pos);
        }
        Ok(position_history.len())
    }
}
