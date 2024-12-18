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

fn size_n_corruptions(input: &InputFormat) -> ((i32, i32), usize) {
    match input.locations.len() {
        25 => ((7, 7), 12),
        3450 => ((71, 71), 1024),
        x => panic!("unknown n_locations = {}", x),
    }
}

impl Graph {
    fn new(input: &InputFormat, n_corruptions: usize) -> Graph {
        let (size, _) = size_n_corruptions(input);
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
    fn go_to_target(&self) -> Option<i32> {
        let mut state = AStarState::new(self.size);
        let target = (self.size.0 - 1, self.size.1 - 1);
        state.push((0, 0), 0);

        while let Some((location, n_steps)) = state.pop() {
            for adj_location in self.adj_locations(location) {
                if adj_location == target {
                    return Some(n_steps + 1);
                }
                state.push(adj_location, n_steps + 1);
            }
        }
        None
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
    fn pop(&mut self) -> Option<((i32, i32), i32)> {
        while let Some((_, negative_n_steps, location)) = self.state_queue.pop() {
            let n_steps = -negative_n_steps;
            let min_steps = self.min_steps_per_state.get(&location).unwrap();
            if n_steps > *min_steps {
                continue;
            }
            return Some((location, n_steps));
        }
        None
    }
}

impl DaySolution for Solution {
    type InputFormat = InputFormat;
    fn solve_1(input: &InputFormat) -> MyResult<impl Debug + 'static> {
        let (_, n_corruptions) = size_n_corruptions(input);
        let graph = Graph::new(input, n_corruptions);
        Ok(graph.go_to_target().unwrap())
    }
    fn solve_2(input: &InputFormat) -> MyResult<impl Debug + 'static> {
        let mut max_solvable = 0;
        let mut min_unsolvable = input.locations.len() - 1;
        while max_solvable + 1 < min_unsolvable {
            let mid = (max_solvable + min_unsolvable) / 2;
            assert_ne!(mid, max_solvable);
            assert_ne!(mid, min_unsolvable);
            if Graph::new(input, mid + 1).go_to_target().is_some() {
                max_solvable = mid;
            } else {
                min_unsolvable = mid;
            }
        }
        let breaking_byte = &input.locations[min_unsolvable];
        Ok((breaking_byte.x, breaking_byte.y))
    }
}
