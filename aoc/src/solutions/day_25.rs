use std::fmt::Debug;

use aoc_utils::{formatted_struct, Chars, DaySolution, MyResult};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

formatted_struct! {
    #[derive(Debug)]
    pub struct Schematic {
        #[separated_by="\n"]
        rows: Vec<Chars>,
    }
}

formatted_struct! {
    #[derive(Debug)]
    pub struct InputFormat {
        #[separated_by="\n\n"]
        schematics: Vec<Schematic>,
    }
}

pub struct Solution;

impl Schematic {
    fn fits(&self, other: &Self) -> bool {
        assert_eq!(self.rows.len(), other.rows.len());
        assert_eq!(self.rows[0].0.len(), other.rows[0].0.len());

        self.rows.iter().zip(&other.rows).all(|(lh, rh)| {
            lh.0.iter()
                .zip(&rh.0)
                .all(|(&lc, &rc)| lc != '#' || rc != '#')
        })
    }
}

fn pairs<T>(l: &[T]) -> Vec<(&T, &T)> {
    let Some((x, xs)) = l.split_first() else {
        return vec![];
    };
    let mut rest = pairs(xs);
    rest.extend(xs.iter().map(|y| (x, y)));
    rest
}

impl DaySolution for Solution {
    type InputFormat = InputFormat;
    fn solve_1(input: &InputFormat) -> MyResult<impl Debug + 'static> {
        let r = pairs(&input.schematics)
            .into_par_iter()
            .filter(|(a, b)| a.fits(b))
            .count();

        Ok(r)
    }
}
