use std::fmt::Debug;

use aoc_utils::{formatted_struct, DaySolution, MyResult};

formatted_struct! {
    #[derive(Debug)]
    pub struct InputFormat {
        "Register A: ",
        initial_a: i64,
        "\nRegister B: ",
        initial_b: i64,
        "\nRegister C: ",
        initial_c: i64,
        "\n\nProgram: ",
        #[separated_by=","]
        program: Vec<i64>,
    }
}

struct State<'a> {
    a: i64,
    b: i64,
    c: i64,

    ip: i64,
    program: &'a [i64],
    output: Vec<i64>,
}

impl<'s> State<'s> {
    fn combo_value(&self, operand: i64) -> i64 {
        match operand {
            0..4 => operand,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            7 => panic!("invalid operand 7"),
            _ => panic!("invalid operand range"),
        }
    }
    fn adv(&mut self, operand: i64) {
        self.a /= 1 << self.combo_value(operand);
    }
    fn bxl(&mut self, operand: i64) {
        self.b ^= operand;
    }
    fn bst(&mut self, operand: i64) {
        self.b = self.combo_value(operand) & 7;
    }
    fn jnz(&mut self, operand: i64) {
        if self.a == 0 {
            return;
        }
        self.ip = operand;
    }
    fn bxc(&mut self, _operand: i64) {
        self.b ^= self.c;
    }
    fn out(&mut self, operand: i64) {
        self.output.push(self.combo_value(operand) & 7);
    }
    fn bdv(&mut self, operand: i64) {
        self.b = self.a / (1 << self.combo_value(operand));
    }
    fn cdv(&mut self, operand: i64) {
        self.c = self.a / (1 << self.combo_value(operand));
    }
    fn exec_instruction(&mut self, opcode: i64, operand: i64) {
        match opcode {
            0 => self.adv(operand),
            1 => self.bxl(operand),
            2 => self.bst(operand),
            3 => self.jnz(operand),
            4 => self.bxc(operand),
            5 => self.out(operand),
            6 => self.bdv(operand),
            7 => self.cdv(operand),
            _ => panic!("unexpected opcode"),
        }
    }
    fn exec(&mut self) {
        while (0..(self.program.len() as i64)).contains(&self.ip) {
            let opcode = self.program[self.ip as usize];
            self.ip += 1;
            let operand = self.program[self.ip as usize];
            self.ip += 1;
            self.exec_instruction(opcode, operand);
        }
    }
}

pub struct Solution;

impl DaySolution for Solution {
    type InputFormat = InputFormat;
    fn solve_1(input: &InputFormat) -> MyResult<impl Debug + 'static> {
        let mut state = State {
            a: input.initial_a,
            b: input.initial_b,
            c: input.initial_c,
            ip: 0,
            program: &input.program,
            output: vec![],
        };
        state.exec();
        Ok(state
            .output
            .iter()
            .map(i64::to_string)
            .collect::<Vec<_>>()
            .join(","))
    }
}
