use std::io::{stdout, Write};

use aoc_utils::{make_day_solution, DaySolution, MyResult};
use clap::{builder::PossibleValue, CommandFactory, FromArgMatches, Parser, ValueEnum};

mod solutions;

fn make_day_solutions() -> Vec<DaySolution> {
    use solutions::day_00;
    vec![make_day_solution(
        "day_00",
        day_00::solve_1,
        day_00::solve_2,
    )]
}

#[derive(Clone, Copy, ValueEnum)]
enum Input {
    Sample,
    Real,
}

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    day: String,
    #[arg(short, long)]
    input_file: String,
}

fn main() -> MyResult<()> {
    let day_solutions = make_day_solutions();
    let cli = Cli::from_arg_matches(
        &Cli::command()
            .mut_arg("day", |day_arg| {
                day_arg.value_parser(
                    day_solutions
                        .iter()
                        .map(|day_solution| PossibleValue::new(day_solution.day_name))
                        .collect::<Vec<_>>(),
                )
            })
            .get_matches(),
    )?;
    let day_solution = day_solutions
        .into_iter()
        .find(|d| d.day_name == cli.day)
        .unwrap();
    let solution_result = (day_solution.solve)(&cli.input_file)?;
    stdout().write_all(&solution_result)?;
    Ok(())
}
