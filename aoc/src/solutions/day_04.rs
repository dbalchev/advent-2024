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
    for (l, &byte) in b"XMAS".iter().enumerate() {
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
    true
}

const DELTAS: [(i32, i32); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

fn is_cross_mass(grid: &[String], (i, j): (i32, i32), start_angle: i32) -> bool {
    let poke = |i: i32, j: i32| grid[i as usize].as_bytes()[j as usize];
    if poke(i, j) != b'A' {
        return false;
    }

    let poke_angle_offset = |angle_offset: i32| {
        let (di, dj) = DELTAS[((start_angle + angle_offset) % 8) as usize];
        poke(i + di, j + dj)
    };
    if poke_angle_offset(0) != b'M' || poke_angle_offset(4) != b'S' {
        return false;
    }
    if poke_angle_offset(2) != b'M' || poke_angle_offset(6) != b'S' {
        return false;
    }
    true
}

pub struct Solution;

impl DaySolution for Solution {
    type InputFormat = InputFormat;
    fn solve_1(input: &InputFormat) -> MyResult<impl Debug + 'static> {
        let grid = &input.lines;
        let mut num_xmas = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                for direction in &DELTAS {
                    num_xmas += is_xmas(grid, (i as i32, j as i32), direction) as i32;
                }
            }
        }
        Ok(num_xmas)
    }
    fn solve_2(input: &InputFormat) -> MyResult<impl Debug + 'static> {
        let grid = &input.lines;
        let mut num_xmas = 0;
        for i in 1..(grid.len() - 1) {
            for j in 1..(grid[i].len() - 1) {
                // let mut current_xmas = 0;
                for start_angle in [1, 3, 5, 7] {
                    // current_xmas += is_cross_mass(grid, (i as i32, j as i32), start_angle) as i32;
                    num_xmas += is_cross_mass(grid, (i as i32, j as i32), start_angle) as i32;
                }
                // if current_xmas == 1 {
                //     num_xmas += 1;
                //     println!("{}", &grid[i - 1][j - 1..j + 2]);
                //     println!("{}", &grid[i][j - 1..j + 2]);
                //     println!("{}", &grid[i + 1][j - 1..j + 2]);
                //     println!();
                // }
            }
        }
        Ok(num_xmas)
    }
    fn preferred_sample_input() -> i32 {
        1
    }
}
