use std::fmt::Debug;

use aoc_utils::{formatted_struct, make_recursive_fn, DaySolution, MyResult};

formatted_struct! {
    #[derive(Debug)]
    pub struct ButtonMoves {
        "X+",
        x_delta: i64,
        ", Y+",
        y_delta:i64,
        "\n",
    }
}

formatted_struct! {
    #[derive(Debug)]
    pub struct ClawMachine {
        "Button A: ",
        a: ButtonMoves,
        "Button B: ",
        b: ButtonMoves,
        "Prize: X=",
        prize_x: i64,
        ", Y=",
        prize_y: i64,
    }
}

formatted_struct! {
    #[derive(Debug)]
    pub struct InputFormat {
        #[separated_by="\n\n"]
        claw_machines: Vec<ClawMachine>,
    }
}

impl ClawMachine {
    fn solve_1(&self) -> Option<i64> {
        let mut memo = make_recursive_fn(
            |rec_memo: &mut dyn FnMut((i64, i64)) -> Option<i64>, (x, y)| {
                if x == 0 && y == 0 {
                    return Some(0);
                }
                if x < 0 || y < 0 {
                    return None;
                }
                let a_result = rec_memo((x - self.a.x_delta, y - self.a.y_delta)).map(|x| x + 3);
                let b_result = rec_memo((x - self.b.x_delta, y - self.b.y_delta)).map(|x| x + 1);
                a_result.into_iter().chain(b_result).min()
            },
        );
        memo((self.prize_x, self.prize_y))
    }
}

pub struct Solution;

impl DaySolution for Solution {
    type InputFormat = InputFormat;
    fn solve_1(input: &InputFormat) -> MyResult<impl Debug + 'static> {
        let results = input
            .claw_machines
            .iter()
            .map(ClawMachine::solve_1)
            .collect::<Vec<_>>();
        Ok(results.into_iter().filter_map(|x| x).sum::<i64>())
    }
}
