use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

use aoc_utils::{formatted_struct, graph::Graph, Chars, DaySolution, MyResult};

formatted_struct! {
    #[derive(Debug)]
    pub struct InputFormat {
        #[separated_by="\n"]
        rows: Vec<Chars>,
    }
}

pub struct Solution;

const DELTAS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

impl InputFormat {
    fn read_graph(&self) -> (Graph<(i32, i32), ()>, (i32, i32), (i32, i32)) {
        let mut edges = HashSet::new();
        let mut start_pos = None;
        let mut end_pos = None;
        for i in 1..(self.rows.len() - 1) {
            for j in (1..self.rows[i].0.len() - 1) {
                let current_char = self.rows[i].0[j];
                let current_pos = Some((i as i32, j as i32));
                match current_char {
                    'S' => {
                        assert!(start_pos.is_none());
                        start_pos = current_pos;
                    }
                    'E' => {
                        assert!(end_pos.is_none());
                        end_pos = current_pos;
                    }
                    '#' => continue,
                    _ => {}
                };
                for (di, dj) in DELTAS {
                    let ci = di + i as i32;
                    let cj = dj + j as i32;
                    if self.rows[ci as usize].0[cj as usize] != '.' {
                        continue;
                    }
                    edges.extend([
                        ((i as i32, j as i32), (ci, cj)),
                        ((ci, cj), (i as i32, j as i32)),
                    ]);
                }
            }
        }
        (
            Graph::from_edges(edges),
            start_pos.unwrap(),
            end_pos.unwrap(),
        )
    }
    fn count_threshold(&self) -> [i32; 2] {
        match self.rows.len() {
            15 => [1, 50],
            141 => [100, 100],
            _ => panic!("{}", self.rows.len()),
        }
    }
    fn generic_solve(&self, cheat_threshold: i32, count_threshold: i32) -> (i32, Vec<(i32, i32)>) {
        let (graph, start_pos, end_pos) = self.read_graph();
        let distance_from_start = graph.shortest_paths(start_pos, |_| false);
        let distance_from_end = graph.shortest_paths(end_pos, |_| false);
        let no_cheating_distance = distance_from_start(end_pos).unwrap();

        let mut cheat_count = HashMap::new();
        let mut add_cheat_distance = |cheat_start: (i32, i32), cheat_end: (i32, i32)| {
            let cheat_duration =
                (cheat_start.0.abs_diff(cheat_end.0) + cheat_start.1.abs_diff(cheat_end.1)) as i32;
            if cheat_duration > cheat_threshold {
                return None;
            }
            let cheat_distance = distance_from_start(cheat_start)? + distance_from_end(cheat_end)?;
            let saving = no_cheating_distance - cheat_distance - cheat_duration;
            if saving < count_threshold {
                return None;
            }
            // if saving == 2 {
            //     c2.push((cheat_start, cheat_end));
            // }
            *cheat_count.entry(saving).or_insert(0) += 1;
            Some(())
        };
        let vertices = graph.vertices();
        for &v1 in &vertices {
            for &v2 in &vertices {
                add_cheat_distance(v1, v2);
            }
        }
        let mut cheat_count = cheat_count.into_iter().collect::<Vec<_>>();
        cheat_count.sort();
        let cheat_sum = cheat_count.iter().map(|(_, a)| a).sum::<i32>();
        cheat_count.truncate(20);
        (cheat_sum, cheat_count)
    }
}

impl DaySolution for Solution {
    type InputFormat = InputFormat;
    fn solve_1(input: &InputFormat) -> MyResult<impl Debug + 'static> {
        Ok(input.generic_solve(2, input.count_threshold()[0]))
    }
    fn solve_2(input: &InputFormat) -> MyResult<impl Debug + 'static> {
        Ok(input.generic_solve(20, input.count_threshold()[1]))
    }
}
