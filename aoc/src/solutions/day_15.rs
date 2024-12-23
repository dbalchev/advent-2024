use core::panic;
use std::{collections::HashSet, fmt::Debug};

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

#[derive(Clone, Debug, PartialEq, Eq)]
enum Object {
    Empty,
    Wall,
    Crate { id: usize },
}
fn all_crate_pos(left_pos: (usize, usize)) -> impl IntoIterator<Item = (usize, usize)> {
    [left_pos, (left_pos.0, left_pos.1 + 1)]
}

struct Map2State {
    map: Vec<Vec<Object>>,
    crates_left_pos: Vec<(usize, usize)>,
    robot_pos: (usize, usize),
}

impl From<&Map> for Map2State {
    fn from(value: &Map) -> Self {
        let height = value.rows.len();
        let width = value.rows[0].0.len() * 2;
        let mut map = vec![vec![Object::Empty; width]; height];
        let mut crates_left_pos = Vec::new();
        let mut robot_pos = None;
        for (row_no, row) in value.rows.iter().enumerate() {
            for (col_no, object_char) in row.0.iter().enumerate() {
                let col_no = col_no * 2;
                match *object_char {
                    '@' => {
                        assert!(robot_pos.is_none());
                        robot_pos = Some((row_no, col_no));
                    }
                    '#' => {
                        map[row_no][col_no..(col_no + 2)].fill(Object::Wall);
                    }
                    'O' => {
                        map[row_no][col_no..(col_no + 2)].fill(Object::Crate {
                            id: crates_left_pos.len(),
                        });
                        crates_left_pos.push((row_no, col_no));
                    }
                    '.' => (),
                    _ => panic!(),
                };
            }
        }

        Map2State {
            map,
            crates_left_pos,
            robot_pos: robot_pos.unwrap(),
        }
    }
}

impl Map2State {
    fn push_instructions(
        &self,
        at: (usize, usize),
        direction: (isize, isize),
    ) -> Option<HashSet<(usize, (usize, usize))>> {
        let crate_id = match self.map[at.0][at.1] {
            Object::Crate { id } => id,
            Object::Empty => return Some(HashSet::new()),
            Object::Wall => return None,
        };
        let crate_pos = all_crate_pos(self.crates_left_pos[crate_id])
            .into_iter()
            .collect::<Vec<_>>();
        let pushed_pos = crate_pos
            .iter()
            .map(|&p| {
                (
                    (p.0 as isize + direction.0) as usize,
                    (p.1 as isize + direction.1) as usize,
                )
            })
            .filter(|p| !crate_pos.contains(p))
            .collect::<Vec<_>>();
        let mut result = HashSet::new();
        for &p in &pushed_pos {
            result.extend(self.push_instructions(p, direction)?);
        }
        let current_pos = self.crates_left_pos[crate_id];

        result.insert((
            crate_id,
            (
                (current_pos.0 as isize + direction.0) as usize,
                (current_pos.1 as isize + direction.1) as usize,
            ),
        ));

        Some(result)
    }
    fn move_robot(&mut self, direction: (isize, isize)) {
        assert_eq!(self.map[self.robot_pos.0][self.robot_pos.1], Object::Empty);
        let next_space = (
            (self.robot_pos.0 as isize + direction.0) as usize,
            (self.robot_pos.1 as isize + direction.1) as usize,
        );
        let instructions = match self.push_instructions(next_space, direction) {
            None => return,
            Some(x) => x,
        };
        assert_eq!(
            instructions.len(),
            instructions
                .iter()
                .map(|(crate_id, _)| crate_id)
                .collect::<HashSet<_>>()
                .len()
        );
        for &(crate_id, _) in &instructions {
            let old_left_pos = self.crates_left_pos[crate_id];
            let space = &mut self.map[old_left_pos.0][old_left_pos.1..(old_left_pos.1 + 2)];
            assert_eq!(
                *space,
                [
                    Object::Crate { id: crate_id },
                    Object::Crate { id: crate_id }
                ]
            );
            space.fill(Object::Empty);
        }

        for &(crate_id, new_left_pos) in &instructions {
            let space = &mut self.map[new_left_pos.0][new_left_pos.1..(new_left_pos.1 + 2)];
            assert_eq!(*space, [Object::Empty, Object::Empty]);
            space.fill(Object::Crate { id: crate_id });
            self.crates_left_pos[crate_id] = new_left_pos;
        }

        assert_eq!(self.map[self.robot_pos.0][self.robot_pos.1], Object::Empty);
        assert_eq!(self.map[next_space.0][next_space.1], Object::Empty);
        self.robot_pos = next_space;
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
    fn solve_2(input: &InputFormat) -> MyResult<impl Debug + 'static> {
        let mut state = Map2State::from(&input.map);
        for &char in &input.instructions.0 {
            if char == '\n' {
                continue;
            }
            state.move_robot(decode_dir(char));
            // println!("{:?}", state);
        }
        let gps_score = state
            .crates_left_pos
            .iter()
            .map(|&(i, j)| 100 * i + j)
            .sum::<usize>();
        Ok(gps_score)
    }
    fn preferred_sample_input() -> i32 {
        0
    }
}
