use aoc_utils::{MyResult, Parsable, ParseBuffer};
use regex::Regex;

macro_rules! parse_single {
    (type=$type:ty, str=$expr:expr) => {
        <$type as Parsable>::parse($expr)?
    };
    (type=$type:ty, separator=$separator:literal, str=$expr:expr) => {
        $type::parse_separated($expr, $separator)
    };
}

macro_rules! single_read {
    ($buffer:ident) => {
        $buffer.read_to_end()
    };
    ($buffer:ident, $lit:literal) => {
        $buffer.read_until(Regex::new($lit).unwrap())?
    };
}

macro_rules! make_reader {
    (
        struct=$struct_name:ident
        $(leading_literal=$leading_literal:literal)?
        $(name=$name:ident, type=$type:ty {$(until=$lit:literal)? $(separator=$separator:literal)?}),*
    ) => {
        impl Parsable for $struct_name{
            fn parse(text: &str) -> MyResult<$struct_name> {
                let mut buffer = ParseBuffer::new(text);
                $(buffer.skip(Regex::new($leading_literal).unwrap())?)?;
                $(let $name = parse_single!(type=$type, $(separator=$separator,)? str=single_read!(buffer, $($lit)?));)*
                Ok($struct_name {
                    $($name),*
                })
            }
        }
    }
}

macro_rules! formatted_struct {
    (in_process $struct_name:ident $(meta=$($struct_meta:meta),*)? ()-> ($($result:tt)*)) => {
        $(#[$($struct_meta),*])?
        struct $struct_name {
            $($result)*
        }
    };
    (
        $(#[$($struct_meta:meta),*])?
        struct $struct_name:ident
        {
            $($leading_literal:literal)?
            $(
                $(#[separated_by=$separator:literal])?
                $name:ident : $type:ty
                $(,$lit:literal)?
            ),*
        }) => {
        formatted_struct!{in_process $struct_name $(meta=$($struct_meta),*)? ($($name : $type),* ) -> ( )}
        make_reader!{struct=$struct_name $(leading_literal=$leading_literal)? $(name=$name, type=$type {$(until=$lit)? $(separator=$separator)?}),*}
    };
    (in_process $struct_name:ident $(meta=$($struct_meta:meta),*)? ($($name:ident : $type:ty),* ) -> ($($result:tt)*)) => {
        formatted_struct!{in_process $struct_name $(meta=$($struct_meta),*)? ( )-> ($($result)*  $($name:$type),*  ) }
    };

}

formatted_struct! {
    #[derive(PartialEq, Eq, Debug)]
    struct Foo {
        "game"
        foo:String,
        "bz",
        // #[separated_by=","]
        // baz:Vec<i32>,
        // "bar",
        bar:i32,
        "baz"
    }
}

pub type InputFormat = String;

pub fn solve_1(input: &InputFormat) -> MyResult<String> {
    // let f = Foo {
    //     foo: "x".to_string(),
    //     bar: 3,
    // };
    Ok(format!("Hello {}", input))
}

pub fn solve_2(_input: &InputFormat) -> MyResult<String> {
    Err("not implemented")?
}

mod tests {
    use super::*;

    #[test]
    fn parse_foo() -> MyResult<()> {
        let parsed = Foo::parse("gamef00bz123baz")?;
        assert_eq!(
            parsed,
            Foo {
                foo: "f00".to_string(),
                bar: 123
            }
        );
        Ok(())
    }
}
