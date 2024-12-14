use std::fmt::Debug;

use aoc_utils::{formatted_struct, DaySolution, MyResult};

formatted_struct! {
    #[derive(Debug)]
    pub struct Robot {
        "p=",
        p_x: i64,
        ",",
        p_y: i64,
        " v=",
        v_x: i64,
        ",",
        v_y:i64,
    }
}

formatted_struct! {
    #[derive(Debug)]
    pub struct Bathroom {
        #[separated_by="\n"]
        robots: Vec<Robot>,
    }
}

fn extrapolate(p:i64, v:i64, t:i64, size:i64) -> i64 {
    let mut r = (p + v * t) % size;
    if r < 0 { 
        r += size;
    }
    r
}

impl Robot {
    fn predict(&self, t: i64, bathroom_size: (i64, i64)) -> (i64, i64) {
        let (sx, sy) = bathroom_size;
        (
            extrapolate(self.p_x, self.v_x, t, sx), extrapolate(self.p_y,self.v_y,t,sy)
        )
    }
}

fn single_dim_sector(p: i64, s:i64) -> Option<usize> {
    assert!(s % 2 == 1);
    let boundary = s / 2;
    if p == boundary {
        None
    } else {Some(if p < boundary {
        0
    } else {
        1
    })}
}

impl Bathroom {
    fn size(&self) -> (i64, i64) {
        match self.robots.len() {
            12 => (11, 7),
            500 => (101, 103),
            _ => panic!("Don't know the bathroom size for {} robots", self.robots.len())
        }
    }
    fn location_quadrant(&self, p: (i64, i64)) -> Option<usize> {
        let (sx, sy) = self.size();
        let qx = single_dim_sector(p.0, sx)?;
        let qy = single_dim_sector(p.1, sy)?;
        Some(qx * 2 + qy)
    }
}

pub struct Solution;

impl DaySolution for Solution {
    type InputFormat = Bathroom;
    fn solve_1(input: &Bathroom) -> MyResult<impl Debug + 'static> {
        let bathroom_size = input.size();
        let new_locations = input.robots.iter().map(|r| r.predict(100, bathroom_size)).collect::<Vec<_>>();
        let mut quadrant_counts = [0;4];
        for &location in &new_locations {
            if let Some(q) = input.location_quadrant(location) {
                quadrant_counts[q] += 1;
            }
        }
        Ok(quadrant_counts.into_iter().product::<i64>())
    }
    fn solve_2(input: &Bathroom) -> MyResult<impl Debug + 'static> {
        let bathroom_size = input.size();

        for i in 0..10_000 {
            let line = vec![' '; bathroom_size.0 as usize];
            let mut grid = vec![line ;bathroom_size.1 as usize];
            for new_location in  input.robots.iter().map(|r| r.predict(i, bathroom_size)).collect::<Vec<_>>() {
                grid[new_location.1 as usize][new_location.0 as usize] = 'X';
            }
            let mut output = Vec::new();
            let mut has_sequence = false;
            for line in grid {
                let line: String = line.iter().collect::<String>();
                if line.contains("XXXXXXXXXXXXXXXXXXXXX") {
                    has_sequence = true;
                }
                output.push(line);
            }
            if has_sequence {
                println!("{}", i);
                println!("{}", output.join("\n"));
                println!();
            }
        }
        Ok(())
    }
}