use std::str::FromStr;

use regex::Regex;

use crate::MyResult;

pub trait Parsable: Sized {
    fn parse(text: &str) -> MyResult<Self>;
}

pub trait SeparatorParsable: Sized {
    fn parse_separated_by(text: &str, separator: &Regex) -> MyResult<Self>;
}

impl<A: FromStr> Parsable for A
where
    <A as FromStr>::Err: 'static + std::error::Error,
{
    fn parse(text: &str) -> MyResult<A> {
        Ok(Self::from_str(text)?)
    }
}

impl<A: Parsable> SeparatorParsable for Vec<A> {
    fn parse_separated_by(text: &str, separator: &Regex) -> MyResult<Self> {
        separator
            .split(text)
            .map(A::parse)
            .collect::<MyResult<Vec<A>>>()
    }
}

pub struct ParseBuffer<'a> {
    full_input: &'a str,
    remaining_input: &'a str,
}

impl<'a> ParseBuffer<'a> {
    pub fn new(text: &'a str) -> ParseBuffer<'a> {
        ParseBuffer {
            full_input: text,
            remaining_input: text,
        }
    }
    pub fn skip(&mut self, skip_pattern: &Regex) -> MyResult<()> {
        match skip_pattern.find_at(self.remaining_input, 0) {
            Some(skip_match) => {
                self.remaining_input = &self.remaining_input[skip_match.end()..];
                Ok(())
            }
            None => {
                let error_message = format!(
                    "skip didn't find {:?} when searching in {:?} as part of {:?}",
                    skip_pattern, self.remaining_input, self.full_input
                );
                Err(From::from(error_message))
            }
        }
    }
    pub fn read_until(&mut self, end_pattern: &Regex) -> MyResult<&'a str> {
        let remaining_input = self.remaining_input;
        match end_pattern.find(remaining_input) {
            Some(end_match) => {
                self.remaining_input = &remaining_input[end_match.end()..];
                Ok(&remaining_input[..end_match.start()])
            }
            None => {
                let error_message = format!(
                    "read_until didn't find {:?} when searching in {:?} as part of {:?}",
                    end_pattern, self.remaining_input, self.full_input
                );
                Err(From::from(error_message))
            }
        }
    }
    pub fn read_to_end(self) -> &'a str {
        self.remaining_input
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn skip() -> MyResult<()> {
        let mut buffer = ParseBuffer::new("foobar");
        buffer.skip(&Regex::new("foo").unwrap())?;
        assert_eq!(buffer.read_to_end(), "bar");
        Ok(())
    }
    #[test]
    fn read_until() -> MyResult<()> {
        let mut buffer = ParseBuffer::new("foobarbaz");
        assert_eq!(buffer.read_until(&Regex::new("bar").unwrap())?, "foo");
        assert_eq!(buffer.read_to_end(), "baz");
        Ok(())
    }
    #[test]
    fn parse_separated() -> MyResult<()> {
        let separator = Regex::new(", ").unwrap();
        assert_eq!(
            Vec::<i32>::parse_separated_by("1, 2, 3", &separator)?,
            vec![1, 2, 3]
        );
        Ok(())
    }
}
