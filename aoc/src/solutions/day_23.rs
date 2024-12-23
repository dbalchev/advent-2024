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
    fn compute_adj(&self) -> HashMap<String, HashSet<String>> {
        let mut adj = HashMap::new();

        for (u, v) in self.all_edges() {
            adj.entry(u.to_string())
                .or_insert_with(HashSet::new)
                .insert(v.to_string());
        }
        adj
    }
    fn clique_3<'a>(&'a self, adj: &'a HashMap<String, HashSet<String>>) -> HashSet<Vec<&'a str>> {
        self.all_edges()
            .flat_map(move |(u, v)| {
                adj[u].intersection(&adj[v]).map(move |t| {
                    let mut r = HashSet::with_capacity(3);
                    r.insert(u);
                    r.insert(v);
                    r.insert(t.as_str());
                    r
                })
            })
            .filter(|clique| clique.len() == 3)
            .map(|clique| {
                let mut clique = Vec::from_iter(clique);
                clique.sort();
                clique
            })
            .collect::<HashSet<_>>()
    }
}

impl DaySolution for Solution {
    type InputFormat = InputFormat;
    fn solve_1(input: &InputFormat) -> MyResult<impl Debug + 'static> {
        let adj = input.compute_adj();
        Ok(input
            .clique_3(&adj)
            .iter()
            .filter(|&triplet| triplet.iter().any(|&u| u.starts_with('t')))
            .count())
    }
    fn solve_2(input: &InputFormat) -> MyResult<impl Debug + 'static> {
        let adj = input.compute_adj();
        let cliques = Vec::from_iter(input.clique_3(&adj));
        let adj = adj
            .iter()
            .map(|(u, vs)| {
                (
                    u.as_str(),
                    vs.iter().map(String::as_str).collect::<HashSet<_>>(),
                )
            })
            .collect::<HashMap<_, _>>();
        let mut cliques = Vec::from_iter(cliques);
        loop {
            let mut new_cliques = HashSet::new();

            for clique in &cliques {
                for &v in adj.keys() {
                    if v <= clique.last().unwrap() {
                        continue;
                    }
                    let adj_v = adj.get(v).unwrap();
                    if clique.iter().any(|&u| !adj_v.contains(u)) {
                        continue;
                    }
                    let mut new_clique = clique.clone();
                    new_clique.push(v);
                    new_cliques.insert(new_clique);
                }
            }
            if new_cliques.is_empty() {
                break;
            }
            cliques = Vec::from_iter(new_cliques);
        }

        Ok(cliques[0].join(","))
    }
}
