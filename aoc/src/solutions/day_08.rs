use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

use aoc_utils::{formatted_struct, DaySolution, MyResult, Parsable};

formatted_struct! {
    #[derive(Debug)]
    pub struct RawInputFormat {
        #[separated_by="\n"]
        rows: Vec<String>,
    }
}

#[derive(Debug)]
pub struct ParsedInputFormat {
    antenna_locations: HashMap<char, Vec<(i32, i32)>>,
    n_rows: i32,
    n_cols: i32,
}

impl Parsable for ParsedInputFormat {
    fn parse(text: &str) -> MyResult<Self> {
        let raw = RawInputFormat::parse(text)?;
        let n_rows = raw.rows.len() as i32;
        let n_cols = raw.rows[0].len() as i32;
        let mut antenna_locations = HashMap::new();
        for (i, row) in raw.rows.into_iter().enumerate() {
            for (j, char) in row.chars().enumerate() {
                if char == '.' {
                    continue;
                }
                antenna_locations
                    .entry(char)
                    .or_insert_with(Vec::new)
                    .push((i as i32, j as i32));
            }
        }
        Ok(ParsedInputFormat {
            antenna_locations,
            n_rows,
            n_cols,
        })
    }
}

pub struct Solution;

impl DaySolution for Solution {
    type InputFormat = ParsedInputFormat;
    fn solve_1(input: &ParsedInputFormat) -> MyResult<impl Debug + 'static> {
        let mut antinodes = HashSet::new();
        for antenna_locaations in input.antenna_locations.values() {
            for d_1_location in antenna_locaations {
                for d_2_location in antenna_locaations {
                    if d_1_location == d_2_location {
                        continue;
                    }
                    let di = d_2_location.0 - d_1_location.0;
                    let dj = d_2_location.1 - d_1_location.1;
                    antinodes.insert((d_1_location.0 - di, d_1_location.1 - dj));
                }
            }
        }
        let antinodes_on_map = antinodes
            .into_iter()
            .filter(|(i, j)| (0..input.n_rows).contains(i) && (0..input.n_cols).contains(j))
            .count();
        Ok(antinodes_on_map)
    }
    fn solve_2(input: &ParsedInputFormat) -> MyResult<impl Debug + 'static> {
        let mut antinodes = HashSet::new();
        for (&_antena_name, antenna_locaations) in &input.antenna_locations {
            for &d_1_location in antenna_locaations {
                for &d_2_location in antenna_locaations {
                    if d_1_location == d_2_location {
                        continue;
                    }
                    let di = d_2_location.0 - d_1_location.0;
                    let dj = d_2_location.1 - d_1_location.1;
                    for i in 0..input.n_rows {
                        let di_1 = i - d_1_location.0;
                        if di_1 % di != 0 {
                            continue;
                        }
                        let t = di_1 / di;
                        let j = d_1_location.1 + dj * t;
                        if !(0..input.n_cols).contains(&j) {
                            continue;
                        }

                        antinodes.insert((i, j));
                    }
                }
            }
        }
        // let mut antinodes = Vec::from_iter(antinodes);
        // antinodes.sort();
        Ok(antinodes.len())
    }
}
