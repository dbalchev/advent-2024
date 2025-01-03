use std::fmt::Debug;

use aoc_utils::{DaySolution, MyResult};

pub struct Solution;

fn to_disk_map(input: &str) -> Vec<Option<i32>> {
    const ZERO: i32 = '0' as i32;
    let mut char_iterator = input.chars();
    let mut result = Vec::new();
    let mut next_id = 0;
    while let Some(file_blocks) = char_iterator.next() {
        let file_blocks = file_blocks as i32 - ZERO;
        for _ in 0..file_blocks {
            result.push(Some(next_id));
        }
        let Some(empty_blocks) = char_iterator.next() else {
            break;
        };
        let empty_blocks = empty_blocks as i32 - ZERO;

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

#[derive(Debug)]
enum DiskChunk {
    File { id: i32, size: i32 },
    Empty { size: i32 },
}

fn to_disk_map_2(input: &str) -> Vec<DiskChunk> {
    const ZERO: i32 = '0' as i32;
    let mut char_iterator = input.chars();
    let mut result = Vec::new();
    let mut next_id = 0;
    while let Some(file_blocks) = char_iterator.next() {
        let file_blocks = file_blocks as i32 - ZERO;
        result.push(DiskChunk::File {
            id: next_id,
            size: file_blocks,
        });
        let Some(empty_blocks) = char_iterator.next() else {
            break;
        };
        let empty_blocks = empty_blocks as i32 - ZERO;

        result.push(DiskChunk::Empty { size: empty_blocks });
        next_id += 1;
    }
    result
}

fn compact_2(mut disk_map: Vec<DiskChunk>) -> Vec<DiskChunk> {
    for current_block_index in (0..disk_map.len()).rev() {
        let DiskChunk::File {
            id: current_block_id,
            size: current_block_size,
        } = disk_map[current_block_index]
        else {
            continue;
        };
        let Some((space_index, space_size)) = disk_map[0..current_block_index]
            .iter()
            .enumerate()
            .filter_map(|(i, candidate_block)| {
                if let DiskChunk::Empty { size } = candidate_block {
                    Some((i, *size))
                } else {
                    None
                }
            })
            .find(|(_, empty_size)| *empty_size >= current_block_size)
        else {
            continue;
        };
        if space_size == current_block_size {
            disk_map.swap(current_block_index, space_index);
            continue;
        }
        disk_map[current_block_index] = DiskChunk::Empty {
            size: current_block_size,
        };
        // no need to compcat disk_map after current_block_index
        let DiskChunk::Empty { size } = &mut disk_map[space_index] else {
            panic!("should be empty");
        };
        *size -= current_block_size;
        disk_map.insert(
            space_index,
            DiskChunk::File {
                id: current_block_id,
                size: current_block_size,
            },
        );
    }
    disk_map
}

fn compute_checksum_2(disk_map: &[DiskChunk]) -> i64 {
    let mut current_index = 0;
    let mut result = 0;
    for chunk in disk_map {
        match chunk {
            DiskChunk::Empty { size } => current_index += *size,
            &DiskChunk::File { id, size } => {
                for _ in 0..size {
                    result += id as i64 * current_index as i64;
                    current_index += 1;
                }
            }
        }
    }
    result
}

impl DaySolution for Solution {
    type InputFormat = String;
    fn solve_1(input: &String) -> MyResult<impl Debug + 'static> {
        let disk_map = to_disk_map(input);
        let compacted = compact(disk_map);
        let checksum = compute_checksum(&compacted);
        Ok(checksum)
    }
    fn solve_2(input: &String) -> MyResult<impl Debug + 'static> {
        let disk_map = to_disk_map_2(input);
        let compacted = compact_2(disk_map);
        // return Ok(compacted);
        let checksum = compute_checksum_2(&compacted);
        Ok(checksum)
    }
}
