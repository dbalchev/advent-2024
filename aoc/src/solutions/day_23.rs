use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

use aoc_utils::{formatted_struct, DaySolution, MyResult};

formatted_struct! {
    #[derive(Debug)]
    pub struct Connection {
        from: String,
        "-",
        to: String,
    }
}

formatted_struct! {
    #[derive(Debug)]
    pub struct InputFormat {
        #[separated_by="\n"]
        connections: Vec<Connection>,
    }
}

pub struct Solution;

impl InputFormat {
    fn all_edges(&self) -> impl Iterator<Item = (&str, &str)> {
        self.connections.iter().flat_map(|c| {
            [
                (c.from.as_str(), c.to.as_str()),
                (c.to.as_str(), c.from.as_str()),
            ]
        })
    }
}

impl DaySolution for Solution {
    type InputFormat = InputFormat;
    fn solve_1(input: &InputFormat) -> MyResult<impl Debug + 'static> {
        let mut adj = HashMap::new();

        for (u, v) in input.all_edges() {
            adj.entry(u.to_string())
                .or_insert_with(HashSet::new)
                .insert(v.to_string());
        }
        let triplets = input
            .all_edges()
            .flat_map(|(u, v)| {
                adj[u].intersection(&adj[v]).map(move |t| {
                    let mut r = HashSet::with_capacity(3);
                    r.insert(u);
                    r.insert(v);
                    r.insert(t.as_str());
                    r
                })
            })
            .filter(|clique| clique.len() == 3 && clique.iter().any(|&u| u.starts_with('t')))
            .map(|clique| {
                let mut clique = Vec::from_iter(clique);
                clique.sort();
                clique
            })
            .collect::<HashSet<_>>();

        Ok(triplets.len())
    }
}
