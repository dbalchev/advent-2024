use std::io::{stdout, Write};

use aoc_utils::MyResult;
use clap::{builder::PossibleValue, CommandFactory, FromArgMatches, Parser, ValueEnum};
use solutions::make_day_solutions;

mod solutions;

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
    // println!("{:?}", day_solutions);
    let cli = Cli::from_arg_matches(
        &Cli::command()
            .mut_arg("day", |day_arg| {
                day_arg.value_parser(
                    day_solutions
                        .iter()
                        .map(|day_solution| {
                            PossibleValue::new(day_solution.canonical_name)
                                .aliases(&day_solution.alternative_names)
                        })
                        .collect::<Vec<_>>(),
                )
            })
            .get_matches(),
    )?;
    let day_solution = day_solutions
        .into_iter()
        .find(|d| d.canonical_name == cli.day || d.alternative_names.contains(&&cli.day[..]))
        .unwrap();
    let solution_result = (day_solution.solve)(&cli.input_file)?;
    stdout().write_all(&solution_result)?;
    Ok(())
}
