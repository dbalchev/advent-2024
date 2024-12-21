use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};

pub struct Graph<T, W> {
    start_to_end_to_weight: HashMap<T, HashMap<T, W>>,
}

pub struct Edge<T, W> {
    pub from: T,
    pub to: T,
    pub weight: W,
}

impl<T> From<(T, T)> for Edge<T, ()> {
    fn from(value: (T, T)) -> Self {
        let (from, to) = value;
        Edge {
            from,
            to,
            weight: (),
        }
    }
}

impl<T, W> From<(T, T, W)> for Edge<T, W> {
    fn from(value: (T, T, W)) -> Self {
        let (from, to, weight) = value;
        Edge { from, to, weight }
    }
}

pub trait EdgeWeight {
    type PathWeight;

    fn identity() -> <Self as EdgeWeight>::PathWeight;
    fn madd(&self, acc: &<Self as EdgeWeight>::PathWeight) -> <Self as EdgeWeight>::PathWeight;
}

impl EdgeWeight for () {
    type PathWeight = i32;
    fn identity() -> <Self as EdgeWeight>::PathWeight {
        0
    }
    fn madd(&self, acc: &<Self as EdgeWeight>::PathWeight) -> <Self as EdgeWeight>::PathWeight {
        acc + 1
    }
}

impl EdgeWeight for i32 {
    type PathWeight = i32;
    fn identity() -> Self {
        0
    }

    fn madd(&self, other: &i32) -> i32 {
        *self + *other
    }
}

struct ShortestPathState<T, P> {
    queue: BinaryHeap<(Reverse<P>, T)>,
    min_path_weight: HashMap<T, P>,
}

impl<T: Hash + Eq + Ord + Clone, PathWeight: Ord + Clone> ShortestPathState<T, PathWeight> {
    fn new() -> ShortestPathState<T, PathWeight> {
        ShortestPathState {
            queue: BinaryHeap::new(),
            min_path_weight: HashMap::new(),
        }
    }
    fn push(&mut self, current: T, path_weight: PathWeight) {
        if let Some(old_path_weight) = self.min_path_weight.get(&current) {
            if *old_path_weight < path_weight {
                return;
            }
        }
        self.min_path_weight
            .insert(current.clone(), path_weight.clone());
        self.queue.push((Reverse(path_weight), current));
    }
    fn pop(&mut self) -> Option<(T, PathWeight)> {
        while let Some((Reverse(path_weight), current)) = self.queue.pop() {
            let current_min_path_weight = self.min_path_weight.get(&current).unwrap();
            if path_weight > *current_min_path_weight {
                continue;
            }
            return Some((current, path_weight));
        }
        None
    }
}

impl<
        T: Hash + Eq + Ord + Clone + Debug,
        PathWeight: Ord + Clone + Debug,
        W: EdgeWeight<PathWeight = PathWeight> + Debug + Clone,
    > Graph<T, W>
{
    pub fn from_edges<ToEdge: Into<Edge<T, W>> + Sized>(
        edge_iterator: impl IntoIterator<Item = ToEdge>,
    ) -> Graph<T, W> {
        let mut start_to_end_to_weight = HashMap::new();
        for edge in edge_iterator {
            let Edge { from, to, weight } = edge.into();
            let old_value = start_to_end_to_weight
                .entry(from)
                .or_insert_with(HashMap::new)
                .insert(to, weight);
            assert!(old_value.is_none());
        }
        Graph {
            start_to_end_to_weight,
        }
    }
    pub fn shortest_paths(
        &self,
        from: T,
        to_predicate: impl Fn(&T) -> bool,
    ) -> impl Fn(T) -> Option<PathWeight> {
        let mut state = ShortestPathState::new();
        state.push(from, W::identity());
        while let Some((current, current_path_weight)) = state.pop() {
            if to_predicate(&current) {
                break;
            }
            let current_adj = match self.start_to_end_to_weight.get(&current) {
                None => continue,
                Some(x) => x,
            };
            for (next, edge_weight) in current_adj {
                state.push(next.clone(), edge_weight.madd(&current_path_weight));
            }
        }
        move |x| state.min_path_weight.get(&x).cloned()
    }
    pub fn vertices(&self) -> HashSet<T> {
        let mut vertices = self
            .start_to_end_to_weight
            .keys()
            .cloned()
            .collect::<HashSet<_>>();
        for tos in self.start_to_end_to_weight.values() {
            vertices.extend(tos.keys().cloned());
        }
        vertices
    }
    pub fn edges_of<'a>(&'a self, v: &'a T) -> impl IntoIterator<Item = Edge<T, W>> + 'a {
        self.start_to_end_to_weight
            .get(v)
            .unwrap()
            .iter()
            .map(|(u, w)| Edge {
                from: v.clone(),
                to: u.clone(),
                weight: w.clone(),
            })
    }
}
