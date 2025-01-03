use std::fmt::Debug;

use aoc_utils::{formatted_struct, make_recursive_fn, DaySolution, MyResult};

formatted_struct! {
    #[derive(Debug)]
    pub struct ButtonMoves {
        "X\\+",
        x_delta: i64,
        ", Y\\+",
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

fn invert_discrete(m: [[i64; 2]; 2]) -> ([[i64; 2]; 2], i64) {
    let [[a, b], [c, d]] = m;
    let denominator = a * d - b * c;
    let numerator = [[d, -b], [-c, a]];
    (numerator, denominator)
}

fn mul(m: [[i64; 2]; 2], v: [i64; 2]) -> [i64; 2] {
    let [[a, b], [c, d]] = m;
    let [x, y] = v;
    [a * x + b * y, c * x + d * y]
}

impl ClawMachine {
    fn _solve_1(&self) -> Option<i64> {
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
    fn math_solve_1(&self) -> Option<i64> {
        self.math_solve(self.prize_x, self.prize_y)
    }
    fn math_solve_2(&self) -> Option<i64> {
        const DELTA: i64 = 10_000_000_000_000;
        self.math_solve(DELTA + self.prize_x, DELTA + self.prize_y)
    }
    fn math_solve(&self, target_x: i64, target_y: i64) -> Option<i64> {
        // a * a_x + b * b_x = x
        // a * a_y + b * b_y = y
        // P (a b) = (x y)
        // (a b) = P ^^ -1 (x y)
        let m = [
            [self.a.x_delta, self.b.x_delta],
            [self.a.y_delta, self.b.y_delta],
        ];
        let (im_n, im_d) = invert_discrete(m);
        let [a_n, b_n] = mul(im_n, [target_x, target_y]);
        if im_d == 0 {
            todo!("not yet handling 0 det")
        }
        if a_n % im_d != 0 || b_n % im_d != 0 {
            return None;
        }
        Some(a_n / im_d * 3 + b_n / im_d)
    }
}

pub struct Solution;

impl DaySolution for Solution {
    type InputFormat = InputFormat;
    fn solve_1(input: &InputFormat) -> MyResult<impl Debug + 'static> {
        let results = input
            .claw_machines
            .iter()
            .map(ClawMachine::math_solve_1)
            .collect::<Vec<_>>();
        // Ok(results)
        Ok(results.into_iter().flatten().sum::<i64>())
    }
    fn solve_2(input: &InputFormat) -> MyResult<impl Debug + 'static> {
        let results = input
            .claw_machines
            .iter()
            .map(ClawMachine::math_solve_2)
            .collect::<Vec<_>>();
        // Ok(results)
        Ok(results.into_iter().flatten().sum::<i64>())
    }
}
