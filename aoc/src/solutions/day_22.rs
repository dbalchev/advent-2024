use std::fmt::Debug;

use aoc_utils::{formatted_struct, DaySolution, MyResult};
use rayon::prelude::*;

formatted_struct! {
    #[derive(Debug)]
    pub struct InputFormat {
        #[separated_by="\n"]
        initial_secrets: Vec<i64>,
    }
}

pub struct Solution;

const MODULO: i64 = 16_777_216;

fn evolve(mut x: i64) -> i64 {
    x = (x ^ (x * 64)) % MODULO;
    x = (x ^ (x / 32)) % MODULO;
    x = (x ^ (x * 2048)) % MODULO;
    x
}

fn make_delta_sequences() -> Vec<[i64; 4]> {
    (0..130321)
        .map(|mut x| {
            let mut result = [-10; 4];
            result[0] = (x % 19) - 9;
            x /= 19;
            result[1] = (x % 19) - 9;
            x /= 19;
            result[2] = (x % 19) - 9;
            x /= 19;
            result[3] = (x % 19) - 9;

            result
        })
        .collect::<Vec<_>>()
}

fn sell_price(delta_sequence: &[i64; 4], prices: &[i64], deltas: &[i64]) -> i64 {
    for (window, sell_price) in deltas.windows(4).zip(&prices[4..]) {
        if window == delta_sequence {
            return *sell_price;
        }
    }
    0
}

impl DaySolution for Solution {
    type InputFormat = InputFormat;
    fn solve_1(input: &InputFormat) -> MyResult<impl Debug + 'static> {
        let mut sum = 0;
        for &initial_secret in &input.initial_secrets {
            let mut x = initial_secret;
            for _ in 0..2000 {
                x = evolve(x);
            }
            sum += x;
        }
        Ok(sum)
    }
    fn solve_2(input: &InputFormat) -> MyResult<impl Debug + 'static> {
        let mut price_sequences = Vec::with_capacity(input.initial_secrets.len());
        for &initial_secret in &input.initial_secrets {
            let mut x = initial_secret;
            let mut prices = Vec::with_capacity(2001);
            prices.push(x % 10);
            for _ in 0..2000 {
                x = evolve(x);
                prices.push(x % 10);
            }
            price_sequences.push(prices);
        }
        let price_deltas = price_sequences
            .iter()
            .map(|price| {
                price
                    .iter()
                    .zip(&price[1..])
                    .map(|(p, np)| np - p)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let delta_sequences = make_delta_sequences();
        let best_price = delta_sequences
            .par_iter()
            .map(|delta_sequence| {
                price_sequences
                    .iter()
                    .zip(&price_deltas)
                    .map(|(prices, deltas)| sell_price(delta_sequence, prices, deltas))
                    .sum::<i64>()
            })
            .max()
            .unwrap();
        Ok(best_price)
    }
    fn preferred_sample_input() -> i32 {
        5
    }
}
