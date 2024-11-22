use std::{
    error::Error,
    fmt::Debug,
    fs::File,
    io::{BufReader, Read, Write},
    str::FromStr,
};

use termcolor::{Buffer, ColorSpec, WriteColor};

pub type MyResult<T> = Result<T, Box<dyn Error>>;

pub trait ReadParsable: Sized {
    fn parse_from_reader(reader: &mut BufReader<impl Read>) -> MyResult<Self>;
}

impl<A: FromStr> ReadParsable for A
where
    <A as FromStr>::Err: 'static + std::error::Error,
{
    fn parse_from_reader(reader: &mut BufReader<impl Read>) -> MyResult<A> {
        let mut content = String::new();
        reader.read_to_string(&mut content)?;
        Ok(A::from_str(&content)?)
    }
}
pub struct DaySolution {
    #[allow(clippy::type_complexity)]
    pub solve: Box<dyn Fn(&str) -> MyResult<Vec<u8>>>,
    pub canonical_name: &'static str,
    pub alternative_names: Vec<&'static str>,
}
impl Debug for DaySolution {
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

pub fn make_day_solution<
    InputFormat: ReadParsable + 'static,
    A: Debug + 'static,
    B: Debug + 'static,
>(
    solution_filename: &'static str,
    solve_1: fn(&InputFormat) -> MyResult<A>,
    solve_2: fn(&InputFormat) -> MyResult<B>,
) -> DaySolution {
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
    DaySolution {
        solve: Box::new(move |input_filename| {
            let input_file = File::open(input_filename)?;
            let input = InputFormat::parse_from_reader(&mut BufReader::new(input_file))?;
            let result_1 = solve_1(&input);
            let result_2 = solve_2(&input);
            let mut display_buffer = Buffer::ansi();
            display_buffer
                .set_color(ColorSpec::new().set_fg(Some(termcolor::Color::Yellow)))
                .unwrap();
            display_buffer.write_all(b"Part 1:\n")?;
            display_buffer.write_all(&pretty_print(result_1))?;
            display_buffer
                .set_color(ColorSpec::new().set_fg(Some(termcolor::Color::Yellow)))
                .unwrap();
            display_buffer.write_all(b"\nPart 2:\n")?;
            display_buffer.write_all(&pretty_print(result_2))?;
            display_buffer.write_all(b"\n")?;
            display_buffer.reset()?;
            Ok(display_buffer.into_inner())
        }),
        canonical_name: no_leading_digits,
        alternative_names,
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::VecDeque, io::Write};

    use super::*;

    #[test]
    fn read_ints() -> MyResult<()> {
        let mut reader = VecDeque::new();
        reader.write_all(b"123")?;
        let read = i32::parse_from_reader(&mut BufReader::new(reader))?;
        assert_eq!(read, 123);
        Ok(())
    }
}
