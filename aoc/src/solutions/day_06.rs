use std::{collections::HashMap, fmt::Debug};

use aoc_utils::{formatted_struct, DaySolution, MyResult, Parsable};
use std::error::Error;
formatted_struct! {
    #[derive(Debug)]
    pub struct RawInputFormat {
        #[separated_by="\n"]
        grid: Vec<String>,
    }
}
#[derive(Debug, Clone)]
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

impl ProcessedInputFormat {
    fn simulate(&self) -> Option<usize> {
        let ProcessedInputFormat {
            grid,
            starting_pos: (si, sj),
        } = self;
        let mut current_pos = (*si as isize, *sj as isize);
        let mut current_dir = 0;
        let mut position_history = HashMap::new();
        position_history.insert(current_pos, vec![current_dir]);
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
            let current_pos_history = position_history.entry(current_pos).or_insert(Vec::new());
            if current_pos_history
                .iter()
                .find(|&&x| x == current_dir)
                .is_some()
            {
                return None;
            }
            current_pos_history.push(current_dir);
        }
        Some(position_history.len())
    }
}

impl DaySolution for Solution {
    type InputFormat = ProcessedInputFormat;
    fn solve_1(input: &ProcessedInputFormat) -> MyResult<impl Debug + 'static> {
        input
            .simulate()
            .ok_or_else(|| From::from("shouldn't shuck in a loop"))
    }
    fn solve_2(input: &ProcessedInputFormat) -> MyResult<impl Debug + 'static> {
        let mut input = input.clone();
        let mut loops = 0;
        for i in 0..input.grid.len() {
            for j in 0..input.grid[i].len() {
                if (i, j) == input.starting_pos || input.grid[i][j] != b'.' {
                    continue;
                }
                input.grid[i][j] = b'o';
                loops += input.simulate().is_none() as i32;
                input.grid[i][j] = b'.';
            }
        }
        Ok(loops)
    }
}
