use std::fmt::Debug;

use aoc_utils::{formatted_struct, graph::Graph, Chars, DaySolution, MyResult};

formatted_struct! {
    #[derive(Debug)]
    pub struct InputFormat {
        #[separated_by="\n"]
        rows: Vec<Chars>,
    }
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
enum Orientation {
    Vertical,
    Horizontal,
}

use Orientation::*;
impl InputFormat {
    #[allow(clippy::type_complexity)]
    fn read_graph(
        &self,
    ) -> (
        Graph<((usize, usize), Orientation), i32>,
        (usize, usize),
        (usize, usize),
    ) {
        let mut edges = Vec::new();
        let mut start_pos = None;
        let mut end_pos = None;
        for i in 1..(self.rows.len() - 1) {
            for j in 1..self.rows[i].0.len() - 1 {
                let current_char = self.rows[i].0[j];
                let current_pos = Some((i as usize, j as usize));
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
                edges.extend([
                    (((i, j), Vertical), ((i, j), Horizontal), 1000),
                    (((i, j), Horizontal), ((i, j), Vertical), 1000),
                ]);
                if self.rows[i].0[j + 1] != '#' {
                    edges.extend([
                        (((i, j), Horizontal), ((i, j + 1), Horizontal), 1),
                        (((i, j + 1), Horizontal), ((i, j), Horizontal), 1),
                    ]);
                }
                if self.rows[i + 1].0[j] != '#' {
                    edges.extend([
                        (((i, j), Vertical), ((i + 1, j), Vertical), 1),
                        (((i + 1, j), Vertical), ((i, j), Vertical), 1),
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
pub struct Solution;

impl DaySolution for Solution {
    type InputFormat = InputFormat;
    fn solve_1(input: &InputFormat) -> MyResult<impl Debug + 'static> {
        let (graph, start, end) = input.read_graph();
        let distance_function = graph.shortest_paths((start, Horizontal), |_| false);
        let results = [Horizontal, Vertical]
            .into_iter()
            .flat_map(|o| distance_function((end, o)))
            .min()
            .unwrap();
        Ok(results)
    }
}
