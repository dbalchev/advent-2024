use std::{
    error::Error,
    fmt::Debug,
    fs::File,
    io::{Read, Write},
    time::{Duration, Instant},
};

use termcolor::{Buffer, ColorSpec, WriteColor};

use crate::{MyResult, Parsable};

pub struct ExistentialDaySolution {
    #[allow(clippy::type_complexity)]
    pub solve: Box<dyn Fn(&str) -> MyResult<Vec<u8>>>,
    pub canonical_name: &'static str,
    pub leading_zeros_name: &'static str,
    pub alternative_names: Vec<&'static str>,
    pub preferred_sample_input: i32,
}
impl Debug for ExistentialDaySolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DaySolution")
            .field("canonical_name", &self.canonical_name)
            .field("alternative_names", &self.alternative_names)
            .finish()
    }
}

fn pretty_print<A: Debug + 'static>(result: MyResult<A>) -> Vec<u8> {
    let mut display_buffer = Buffer::ansi();
    match result {
        Ok(x) => {
            display_buffer
                .set_color(ColorSpec::new().set_fg(Some(termcolor::Color::Green)))
                .unwrap();
            display_buffer
                .write_all(format!("{:?}", x).as_bytes())
                .unwrap();
        }
        Err(e) => {
            display_buffer
                .set_color(ColorSpec::new().set_fg(Some(termcolor::Color::Red)))
                .unwrap();
            display_buffer.write_all(e.to_string().as_bytes()).unwrap();
        }
    }
    display_buffer.into_inner()
}

pub trait DaySolution {
    type InputFormat;

    fn solve_1(_input: &Self::InputFormat) -> MyResult<impl Debug + 'static> {
        Err::<(), Box<dyn Error>>(From::from("solve_1 not implemented"))
    }
    fn solve_2(_input: &Self::InputFormat) -> MyResult<impl Debug + 'static> {
        Err::<(), Box<dyn Error>>(From::from("solve_2 not implemented"))
    }
    fn preferred_sample_input() -> i32 {
        0
    }
}

fn measure_time<R, F: FnOnce() -> R>(computation: F) -> (R, Duration) {
    let start_time = Instant::now();
    let result = computation();
    let end_time = Instant::now();
    let diff = end_time - start_time;
    (result, diff)
}

pub fn make_day_solution<A: DaySolution>(solution_filename: &'static str) -> ExistentialDaySolution
where
    A::InputFormat: Parsable,
{
    let no_rs_sufix = solution_filename.trim_end_matches(".rs");
    let no_day_prefix = no_rs_sufix.trim_start_matches("day_");
    let no_leading_digits: &str = no_day_prefix.trim_start_matches("0");
    let no_leading_digits = if no_leading_digits.is_empty() {
        "0"
    } else {
        no_leading_digits
    };
    let mut alternative_names = vec![solution_filename, no_rs_sufix];
    if no_leading_digits != no_day_prefix {
        alternative_names.push(no_day_prefix);
    }
    ExistentialDaySolution {
        solve: Box::new(move |input_filename| {
            let mut input_file = File::open(input_filename)?;
            let mut file_content = String::new();
            input_file.read_to_string(&mut file_content)?;
            let file_content = file_content.trim_end_matches("\n");
            let input = A::InputFormat::parse(file_content)?;
            let (result_1, part_1_time) = measure_time(|| A::solve_1(&input));
            let mut display_buffer = Buffer::ansi();
            display_buffer
                .set_color(ColorSpec::new().set_fg(Some(termcolor::Color::Yellow)))
                .unwrap();
            display_buffer.write_all(
                &format!("Part 1: ({:.3}s)\n", part_1_time.as_secs_f64()).into_bytes(),
            )?;
            display_buffer.write_all(&pretty_print(result_1))?;
            display_buffer
                .set_color(ColorSpec::new().set_fg(Some(termcolor::Color::Yellow)))
                .unwrap();
            let (result_2, part_2_time) = measure_time(|| A::solve_2(&input));
            display_buffer.write_all(
                &format!("\nPart 2: ({:.3}s)\n", part_2_time.as_secs_f64()).into_bytes(),
            )?;
            display_buffer.write_all(&pretty_print(result_2))?;
            display_buffer.write_all(b"\n")?;
            display_buffer.reset()?;
            Ok(display_buffer.into_inner())
        }),
        canonical_name: no_leading_digits,
        leading_zeros_name: no_day_prefix,
        alternative_names,
        preferred_sample_input: A::preferred_sample_input(),
    }
}
