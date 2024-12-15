use std::fmt::Debug;

use aoc_utils::{formatted_struct, Chars, DaySolution, MyResult};

formatted_struct! {
    #[derive(Debug)]
    pub struct Map {
        #[separated_by="\n"]
        rows: Vec<Chars>,
    }
}

formatted_struct! {
    #[derive(Debug)]
    pub struct InputFormat {
        map: Map,
        "\n\n",
        instructions: Chars,
    }
}

pub struct Solution;

#[derive(Debug)]
struct State {
    map: Vec<Vec<char>>,
    current_pos: (usize, usize),
}

impl From<&Map> for State {
    fn from(value: &Map) -> Self {
        let mut map = value
            .rows
            .iter()
            .map(|Chars(row)| row.clone())
            .collect::<Vec<_>>();
        let current_pos = map
            .iter()
            .enumerate()
            .filter_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(j, &c)| if c == '@' { Some((i, j)) } else { None })
                    .next()
            })
            .next()
            .expect("must have @");
        map[current_pos.0][current_pos.1] = '.';
        State { map, current_pos }
    }
}

fn decode_dir(char_direction: char) -> (isize, isize) {
    match char_direction {
        '<' => (0, -1),
        '>' => (0, 1),
        '^' => (-1, 0),
        'v' => (1, 0),
        _ => panic!("unknown direction '{}'", char_direction),
    }
}
impl State {
    fn push(&mut self, at: (usize, usize), direction: (isize, isize)) {
        if self.map[at.0][at.1] != 'O' {
            return;
        }
        let next_space = (
            (at.0 as isize + direction.0) as usize,
            (at.1 as isize + direction.1) as usize,
        );
        self.push(next_space, direction);
        if self.map[next_space.0][next_space.1] != '.' {
            return;
        }
        self.map[next_space.0][next_space.1] = self.map[at.0][at.1];
        self.map[at.0][at.1] = '.';
    }
    fn move_robot_with_dir(&mut self, direction: (isize, isize)) {
        let next_space = (
            (self.current_pos.0 as isize + direction.0) as usize,
            (self.current_pos.1 as isize + direction.1) as usize,
        );
        self.push(next_space, direction);
        if self.map[next_space.0][next_space.1] == '.' {
            self.current_pos = next_space;
        }
    }
    fn move_robot(&mut self, char_direction: char) {
        self.move_robot_with_dir(decode_dir(char_direction));
    }
    fn gps_score(&self) -> usize {
        self.map
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .map(|(j, &c)| if c == 'O' { 100 * i + j } else { 0 })
                    .sum::<usize>()
            })
            .sum()
    }
}

impl DaySolution for Solution {
    type InputFormat = InputFormat;
    fn solve_1(input: &InputFormat) -> MyResult<impl Debug + 'static> {
        let mut state = State::from(&input.map);
        for &char in &input.instructions.0 {
            if char == '\n' {
                continue;
            }
            state.move_robot(char);
            // println!("{:?}", state);
        }
        Ok(state.gps_score())
    }
    fn preferred_sample_input() -> i32 {
        0
    }
}
