use std::fmt::Debug;

use aoc_utils::{DaySolution, MyResult};

pub struct Solution;

fn to_disk_map(input: &str) -> Vec<Option<i32>> {
    const ZERO: i32 = '0' as i32;
    let mut char_iterator = input.chars();
    let mut result = Vec::new();
    let mut next_id = 0;
    loop {
        let file_blocks = if let Some(file_blocks) = char_iterator.next() {
            file_blocks as i32 - ZERO
        } else {
            break;
        };
        for _ in 0..file_blocks {
            result.push(Some(next_id));
        }
        let empty_blocks = if let Some(empty_blocks) = char_iterator.next() {
            empty_blocks as i32 - ZERO
        } else {
            break;
        };

        for _ in 0..empty_blocks {
            result.push(None);
        }
        next_id += 1;
    }
    result
}

fn compute_checksum(compacted: &[i32]) -> i64 {
    compacted
        .iter()
        .enumerate()
        .map(|(i, &id)| i as i64 * id as i64)
        .sum()
}

fn compact(mut disk_map: Vec<Option<i32>>) -> Vec<i32> {
    let mut first_possibly_empty = 0;
    let mut last_possibly_file = disk_map.len() - 1;
    while first_possibly_empty < last_possibly_file {
        if disk_map[first_possibly_empty].is_some() {
            first_possibly_empty += 1;
            continue;
        }
        if disk_map[last_possibly_file].is_none() {
            last_possibly_file -= 1;
            continue;
        }
        disk_map.swap(first_possibly_empty, last_possibly_file);
        first_possibly_empty += 1;
        last_possibly_file -= 1;
    }
    let first_empty = disk_map
        .iter()
        .enumerate()
        .find(|(_, value)| value.is_none())
        .map(|(i, _)| i)
        .unwrap_or(disk_map.len());
    assert!(disk_map[0..first_empty].iter().all(Option::is_some));
    assert!(disk_map[first_empty..].iter().all(Option::is_none));
    disk_map
        .into_iter()
        .take(first_empty)
        .collect::<Option<Vec<_>>>()
        .expect("should've skipped the empty")
}

impl DaySolution for Solution {
    type InputFormat = String;
    fn solve_1(input: &String) -> MyResult<impl Debug + 'static> {
        let disk_map = to_disk_map(&input);
        let compacted = compact(disk_map);
        let checksum = compute_checksum(&compacted);
        Ok(checksum)
    }
}
