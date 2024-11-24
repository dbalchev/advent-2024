use std::{
    error::Error,
    io::{BufReader, Read},
    str::FromStr,
};

mod day_solution;

pub use crate::day_solution::{make_day_solution, DaySolution};

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
