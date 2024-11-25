use crate::MyResult;
use regex::Regex;

#[macro_export]
macro_rules! parse_single {
    (type=$type:ty, str=$expr:expr) => {
        <$type as $crate::Parsable>::parse($expr)?
    };
    (type=$type:ty, separator=$separator:literal, str=$expr:expr) => {
        <$type as $crate::SeparatorParsable>::parse_separated_by(
            $expr,
            Regex::new($separator).unwrap(),
        )?
    };
}

#[macro_export]
macro_rules! single_read {
    ($buffer:ident) => {
        $buffer.read_to_end()
    };
    ($buffer:ident, $lit:literal) => {
        $buffer.read_until(Regex::new($lit).unwrap())?
    };
}

#[macro_export]
macro_rules! make_reader {
    (
        struct=$struct_name:ident
        $(leading_literal=$leading_literal:literal)?
        $(name=$name:ident, type=$type:ty {$(until=$lit:literal)? $(separator=$separator:literal)?}),*
    ) => {
        impl $crate::Parsable for $struct_name{
            fn parse(text: &str) -> MyResult<$struct_name> {
                let mut buffer = $crate::ParseBuffer::new(text);
                $(buffer.skip(Regex::new($leading_literal).unwrap())?)?;
                $(let $name = $crate::parse_single!(type=$type, $(separator=$separator,)? str=$crate::single_read!(buffer$(, $lit)?));)*
                Ok($struct_name {
                    $($name),*
                })
            }
        }
    }
}

#[macro_export]
macro_rules! make_item {
    ($item_type:tt $item_name:ident $(meta=$($item_meta:meta),*)? {$($body_token:tt)*}) => {
        $(#[$($item_meta),*])?
        $item_type $item_name {
            $($body_token)*
        }
    };
}

#[macro_export]
macro_rules! formatted_struct {
    (
        $(#[$($struct_meta:meta),*])?
        struct $struct_name:ident
        {
            $($leading_literal:literal,)?
            $(
                $(#[separated_by=$separator:literal])?
                $name:ident : $type:ty,
                $($lit:literal)?
            ),*
        }
    ) => {
        $crate::make_item!{struct $struct_name $(meta=$($struct_meta),*)? { $($name:$type),*}}
        $crate::make_reader!{struct=$struct_name $(leading_literal=$leading_literal)? $(name=$name, type=$type {$(until=$lit)? $(separator=$separator)?}),*}
    };
    (
        $(#[$($enum_meta:meta),*])?
        enum $enum_name:ident
        {
            $(
                $variant_name:ident {
                    $($leading_literal:literal,)?
                    $(
                        $(#[separated_by=$separator:literal])?
                        $name:ident : $type:ty,
                        $($lit:literal)?
                    ),*
                },
            )+
        }
    ) => {
        $crate::make_item!{
            enum $enum_name $(meta=$($enum_meta),*)?
            {
                $(
                    $variant_name
                    {
                        $($name:$type),*
                    }
                ),+
            }
        }
        // $crate::make_reader!{struct=$enum_name $(leading_literal=$leading_literal)? $(name=$name, type=$type {$(until=$lit)? $(separator=$separator)?}),*}
    };
}

mod tests {

    formatted_struct! {
        #[derive(PartialEq, Eq, Debug)]
        struct TestLeadingInnerAndTrailing {
            "game",
            first:String,
            "bz",
            #[separated_by=","]
            bz:Vec<i32>,
            "bar",
            bar:i32,
            "baz"
        }
    }

    formatted_struct! {
        #[derive(PartialEq, Eq, Debug)]
        struct TestNoTrailing {
            "game",
            first:String,
            "bz",
            bar:i32,
        }
    }

    formatted_struct! {
        enum VariantTest {
            Foo {
                "foo",
                bar: String,
                "baz",
                bz: i32,
            },
            Fiz {
                "fiz",
                buz: i32,
            },
        }
    }

    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use crate::Parsable;

    #[test]
    fn parse_leading_inner_and_trailing() -> MyResult<()> {
        let parsed = TestLeadingInnerAndTrailing::parse("gamef00bz1,2,3bar123baz")?;
        assert_eq!(
            parsed,
            TestLeadingInnerAndTrailing {
                first: "f00".to_string(),
                bz: vec![1, 2, 3],
                bar: 123
            }
        );
        Ok(())
    }
    #[test]
    fn parse_no_trailing() -> MyResult<()> {
        let parsed = TestNoTrailing::parse("gamef00bz123")?;
        assert_eq!(
            parsed,
            TestNoTrailing {
                first: "f00".to_string(),
                bar: 123
            }
        );
        Ok(())
    }
}
