use aoc_utils::MyResult;

pub type InputFormat = String;

pub fn solve_1(input: &InputFormat) -> MyResult<String> {
    Ok(format!("Hello {}", input))
}

pub fn solve_2(_input: &InputFormat) -> MyResult<String> {
    Err("not implemented")?
}
