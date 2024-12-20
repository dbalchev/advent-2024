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
}

impl DaySolution for Solution {
    type InputFormat = InputFormat;
    fn solve_1(input: &InputFormat) -> MyResult<impl Debug + 'static> {
        let (graph, start_pos, end_pos) = input.read_graph();
        let distance_from_start = graph.shortest_paths(start_pos, |_| false);
        let distance_from_end = graph.shortest_paths(end_pos, |_| false);
        let mut distance_2_deltas = HashSet::new();
        let threshold = match input.rows.len() {
            15 => 1,
            141 => 100,
            _ => panic!("{}", input.rows.len()),
        };
        let no_cheating_distance = distance_from_start(end_pos).unwrap();
        for d1 in DELTAS {
            for d2 in DELTAS {
                let new_entry = (d1.0 + d2.0, d1.1 + d2.1);
                if new_entry == (0, 0) {
                    continue;
                }
                distance_2_deltas.insert(new_entry);
            }
        }
        let mut cheat_count = HashMap::new();
        // let mut c2 = vec![];
        let mut add_cheat_distance = |cheat_start, cheat_end| {
            let cheat_distance = distance_from_start(cheat_start)? + distance_from_end(cheat_end)?;
            let saving = no_cheating_distance - cheat_distance - 2;
            if saving < threshold {
                return None;
            }
            // if saving == 2 {
            //     c2.push((cheat_start, cheat_end));
            // }
            *cheat_count.entry(saving).or_insert(0) += 1;
            Some(())
        };
        for (i, j) in graph.vertices() {
            for delta in &distance_2_deltas {
                let di = i + delta.0;
                let dj = j + delta.1;
                add_cheat_distance((i, j), (di, dj));
            }
        }
        // c2.sort();
        // println!("{:?}", c2);
        let mut cheat_count = cheat_count.into_iter().collect::<Vec<_>>();
        cheat_count.sort();
        Ok((cheat_count.iter().map(|(_, a)| a).sum::<i32>(), cheat_count))
    }
}
