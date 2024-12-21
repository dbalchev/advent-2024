use std::fmt::Debug;

use aoc_utils::{formatted_struct, Chars, DaySolution, MyResult};

formatted_struct! {
    #[derive(Debug)]
    pub struct InputFormat {
        #[separated_by="\n"]
        rows: Vec<Chars>,
    }
}

pub struct Solution;

fn adj_indices((i, j): (usize, usize), (mi, mj): (usize, usize)) -> Vec<(usize, usize)> {
    let mut result = vec![];
    let mi = mi as isize;
    let mj = mj as isize;
    let i = i as isize;
    let j = j as isize;
    for (di, dj) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
        let ci = i + di;
        let cj = j + dj;
        if (0..mi).contains(&ci) && (0..mj).contains(&cj) {
            result.push((ci as usize, cj as usize));
        }
    }
    result
}

fn label_regions(rows: &[Chars]) -> (Vec<Vec<i32>>, i32) {
    let mi = rows.len();
    let mj = rows[0].0.len();
    let mut result = vec![vec![-1; mj]; mi];
    let mut label_no = 0;
    for i in 0..result.len() {
        for j in 0..result[i].len() {
            if result[i][j] != -1 {
                continue;
            }
            let original_label = rows[i].0[j];
            let mut stack = vec![(i, j)];
            result[i][j] = label_no;
            while let Some((ci, cj)) = stack.pop() {
                for (ai, aj) in adj_indices((ci, cj), (mi, mj)) {
                    if result[ai][aj] != -1 || rows[ai].0[aj] != original_label {
                        continue;
                    }
                    result[ai][aj] = label_no;
                    stack.push((ai, aj));
                }
            }
            label_no += 1;
        }
    }

    (result, label_no)
}

fn label_score(labels: &[Vec<i32>], target_label: i32) -> i32 {
    let mi = labels.len();
    let mj = labels[0].len();
    let mut perimeter = 0;
    let mut area = 0;

    for i in 0..mi {
        for j in 0..mj {
            if labels[i][j] != target_label {
                continue;
            }
            area += 1;
            for (di, dj) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
                let ai = i as isize + di;
                let aj = j as isize + dj;
                if (0..mi as isize).contains(&ai) && (0..mj as isize).contains(&aj) {
                    perimeter += (labels[ai as usize][aj as usize] != target_label) as i32;
                } else {
                    perimeter += 1;
                }
            }
        }
    }
    area * perimeter
}

impl DaySolution for Solution {
    type InputFormat = InputFormat;
    fn solve_1(input: &InputFormat) -> MyResult<impl Debug + 'static> {
        let (regions, n_labels) = label_regions(&input.rows);
        let result = (0..n_labels).map(|i| label_score(&regions, i)).sum::<i32>();
        // .collect::<Vec<_>>();
        Ok(result)
    }
}
