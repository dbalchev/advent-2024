use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Debug,
};

use aoc_utils::{formatted_struct, DaySolution, MyResult};

formatted_struct! {
    #[derive(Debug)]
    pub struct MemoryLocation {
        x:i32,
        ",",
        y:i32,
    }
}

formatted_struct! {
    #[derive(Debug)]
    pub struct InputFormat {
        #[separated_by="\n"]
        locations: Vec<MemoryLocation>,
    }
}

pub struct Solution;

#[derive(Debug)]
struct Graph {
    size: (i32, i32),
    corruptions: HashSet<(i32, i32)>,
}

const DELTAS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

impl Graph {
    fn new(input: &InputFormat) -> Graph {
        let (size, n_corruptions) = match input.locations.len() {
            25 => ((7, 7), 12),
            3450 => ((71, 71), 1024),
            x => panic!("unknown n_locations = {}", x),
        };
        let corruptions = input.locations[..n_corruptions]
            .iter()
            .map(|MemoryLocation { x, y }| (*x, *y))
            .collect::<HashSet<_>>();
        Graph { size, corruptions }
    }
    fn adj_locations(&self, location: (i32, i32)) -> Vec<(i32, i32)> {
        let mut result = vec![];
        let (n, m) = self.size;
        for &(di, dj) in &DELTAS {
            let i = location.0 + di;
            let j = location.1 + dj;
            if !(0..n).contains(&i) || !(0..m).contains(&j) || self.corruptions.contains(&(i, j)) {
                continue;
            }
            result.push((i, j));
        }
        result
    }
}

struct AStarState {
    size: (i32, i32),
    state_queue: BinaryHeap<(i32, i32, (i32, i32))>,
    min_steps_per_state: HashMap<(i32, i32), i32>,
}

impl AStarState {
    fn new(size: (i32, i32)) -> AStarState {
        AStarState {
            size,
            state_queue: BinaryHeap::new(),
            min_steps_per_state: HashMap::new(),
        }
    }
    fn optimistic_remaining(&self, location: (i32, i32), n_steps: i32) -> i32 {
        self.size.0 - location.0 + self.size.1 - location.1 + n_steps
    }
    fn push(&mut self, location: (i32, i32), n_steps: i32) {
        if let Some(&current_distance) = self.min_steps_per_state.get(&location) {
            if current_distance <= n_steps {
                return;
            }
        }
        let remaining = self.optimistic_remaining(location, n_steps);
        self.state_queue.push((-remaining, -n_steps, location));
        self.min_steps_per_state.insert(location, n_steps);
    }
    fn pop(&mut self) -> ((i32, i32), i32) {
        let (_, negative_n_steps, location) = self.state_queue.pop().unwrap();
        (location, -negative_n_steps)
    }
}

impl DaySolution for Solution {
    type InputFormat = InputFormat;
    fn solve_1(input: &InputFormat) -> MyResult<impl Debug + 'static> {
        let graph = Graph::new(input);
        let mut state = AStarState::new(graph.size);
        let target = (graph.size.0 - 1, graph.size.1 - 1);
        state.push((0, 0), 0);

        loop {
            let (location, n_steps) = state.pop();
            for adj_location in graph.adj_locations(location) {
                if adj_location == target {
                    return Ok(n_steps + 1);
                }
                state.push(adj_location, n_steps + 1);
            }
        }

        panic!("foo")
    }
}
