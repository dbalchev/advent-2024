use std::{collections::HashMap, fmt::Debug, sync::atomic::AtomicUsize, thread};

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
    fn simulate(&self) -> Option<Vec<(isize, isize)>> {
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
            if current_pos_history.iter().any(|&x| x == current_dir) {
                return None;
            }
            current_pos_history.push(current_dir);
        }
        Some(position_history.keys().cloned().collect::<Vec<_>>())
    }
}

impl DaySolution for Solution {
    type InputFormat = ProcessedInputFormat;
    fn solve_1(input: &ProcessedInputFormat) -> MyResult<impl Debug + 'static> {
        Ok(input
            .simulate()
            .ok_or_else(|| -> Box<dyn Error> { From::from("shouldn't shuck in a loop") })?
            .len())
    }
    fn solve_2(input: &ProcessedInputFormat) -> MyResult<impl Debug + 'static> {
        let initial_path = input
            .simulate()
            .ok_or_else(|| -> Box<dyn Error> { From::from("shouldn't shuck in a loop") })?;
        let work_to_be_done = Vec::from(initial_path);
        let work_index = AtomicUsize::new(0);
        let result = thread::scope(|score| {
            let threads = (0..8)
                .map(|_| {
                    score.spawn(|| {
                        let mut loops = 0;
                        let mut input = input.clone();
                        loop {
                            let (i, j) = {
                                let my_index =
                                    work_index.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                                if my_index < work_to_be_done.len() {
                                    work_to_be_done[my_index]
                                } else {
                                    break;
                                }
                            };

                            let i = i as usize;
                            let j = j as usize;
                            if (i, j) == input.starting_pos || input.grid[i][j] != b'.' {
                                continue;
                            }
                            input.grid[i][j] = b'o';
                            loops += input.simulate().is_none() as i32;
                            input.grid[i][j] = b'.';
                        }

                        return loops;
                    })
                })
                .collect::<Vec<_>>();
            threads
                .into_iter()
                .map(|thread| thread.join().unwrap())
                .sum::<i32>()
        });
        Ok(result)
    }
}
