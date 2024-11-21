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
        return Ok(A::from_str(&content)?);
    }
}

pub struct DaySolution {
    pub day_no: i32,
    pub solve: Box<dyn FnOnce(&str) -> MyResult<Vec<u8>>>,
}

fn pretty_print<A: Debug + 'static>(result: MyResult<A>) -> Vec<u8> {
    let mut display_buffer = Buffer::ansi();
    match result {
        Ok(x) => {
            display_buffer
                .set_color(&ColorSpec::new().set_fg(Some(termcolor::Color::Green)))
                .unwrap();
            display_buffer.write(format!("{:?}", x).as_bytes()).unwrap();
        }
        Err(e) => {
            display_buffer
                .set_color(&ColorSpec::new().set_fg(Some(termcolor::Color::Red)))
                .unwrap();
            display_buffer.write(e.to_string().as_bytes()).unwrap();
        }
    }
    return display_buffer.into_inner();
}

pub fn make_day_solution<
    InputFormat: ReadParsable + 'static,
    A: Debug + 'static,
    B: Debug + 'static,
    F1: FnOnce(&InputFormat) -> MyResult<A> + 'static,
    F2: FnOnce(&InputFormat) -> MyResult<B> + 'static,
>(
    _solution_filename: &str,
    solve_1: F1,
    solve_2: F2,
) -> DaySolution {
    DaySolution {
        day_no: 0,
        solve: Box::new(|input_filename| {
            let input_file = File::open(input_filename)?;
            let input = InputFormat::parse_from_reader(&mut BufReader::new(input_file))?;
            let result_1 = solve_1(&input);
            let result_2 = solve_2(&input);
            let mut display_buffer = Buffer::ansi();
            display_buffer
                .set_color(&ColorSpec::new().set_fg(Some(termcolor::Color::Yellow)))
                .unwrap();
            display_buffer.write_all(b"Part 1:\n")?;
            display_buffer.write_all(&pretty_print(result_1))?;
            display_buffer
                .set_color(&ColorSpec::new().set_fg(Some(termcolor::Color::Yellow)))
                .unwrap();
            display_buffer.write_all(b"\nPart 2:\n")?;
            display_buffer.write_all(&pretty_print(result_2))?;
            display_buffer.write_all(b"\n")?;
            display_buffer.reset()?;
            Ok(display_buffer.into_inner())
        }),
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
