use std::io::{stdout, Write};

use aoc_utils::{make_day_solution, DaySolution, MyResult};
use clap::{command, Parser, Subcommand};

mod solutions;

fn make_day_solutions() -> Vec<DaySolution> {
    use solutions::day_00;
    vec![make_day_solution(
        "day_00",
        day_00::solve_1,
        day_00::solve_2,
    )]
}

#[derive(Subcommand)]
enum Command {
    Test,
    Real,
}

#[derive(Parser)]
struct Cli {
    #[arg(short, long, group = "input")]
    day: i32,
    #[command(subcommand)]
    command: Command,
    #[arg(short, long, group = "input")]
    file: Option<String>,
}

fn main() -> MyResult<()> {
    let cli = Cli::parse();
    let day_solutions = make_day_solutions();
    let day_solution = day_solutions
        .into_iter()
        .find(|d| d.day_no == cli.day)
        .unwrap();
    let solution_result = (day_solution.solve)("inputs/test/00.txt")?;
    stdout().write_all(&solution_result)?;
    Ok(())
}
