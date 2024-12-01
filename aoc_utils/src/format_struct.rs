use crate::MyResult;

#[macro_export]
macro_rules! make_regex {
    ($expr:expr) => {
        {
            use std::sync::LazyLock;
            use $crate::Regex;
            static COMPILED_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new($expr).unwrap());
            LazyLock::force(&COMPILED_REGEX)
        }
    };
}

#[macro_export]
macro_rules! parse_single {
    (type=$type:ty, str=$expr:expr) => {
        <$type as $crate::Parsable>::parse($expr)?
    };
    (type=$type:ty, separator=$separator:literal, str=$expr:expr) => {
        <$type as $crate::SeparatorParsable>::parse_separated_by(
            $expr,
            $crate::make_regex!($separator),
        )?
    };
}

#[macro_export]
macro_rules! single_read {
    ($buffer:ident) => {
        $buffer.read_to_end()
    };
    ($buffer:ident, $lit:literal) => {
        $buffer.read_until($crate::make_regex!($lit))?
    };
}

#[macro_export]
macro_rules! make_reader_body {
    (
        constructor=$constructor_name:ident
        text=($text:expr)
        $(leading_literal=$leading_literal:literal)?
        $(name=$name:ident, type=$type:ty {$(until=$lit:literal)? $(separator=$separator:literal)?}),*
    ) => {
        let mut buffer = $crate::ParseBuffer::new($text);
        $(buffer.skip($crate::make_regex!($leading_literal))?)?;
        $(let $name = $crate::parse_single!(type=$type, $(separator=$separator,)? str=$crate::single_read!(buffer$(, $lit)?));)*
        Ok($constructor_name {
            $($name),*
        })
    }
}

#[macro_export]
macro_rules! make_item {
    ($item_type:tt $item_name:ident vis=($vis:vis) $(meta=$($item_meta:meta),*)? {$($body_token:tt)*}) => {
        $(#[$($item_meta),*])?
        $vis $item_type $item_name {
            $($body_token)*
        }
    };
}

#[macro_export]
macro_rules! formatted_struct {
    (
        $(#[$($struct_meta:meta),*])?
        $vis: vis struct $struct_name:ident
        {
            $($leading_literal:literal,)?
            $(
                $(#[separated_by=$separator:literal])?
                $name:ident : $type:ty,
                $($lit:literal)?
            ),*
            $(,)?
        }
    ) => {
        $crate::make_item!{struct $struct_name vis=($vis) $(meta=$($struct_meta),*)? { $($name:$type),*}}

        impl $crate::Parsable for $struct_name{
            fn parse(text: &str) -> MyResult<$struct_name> {
                $crate::make_reader_body!{
                    constructor=$struct_name
                    text=(text)
                    $(leading_literal=$leading_literal)?
                    $(name=$name, type=$type {$(until=$lit)? $(separator=$separator)?}),*
                }
            }
        }
    };
    (
        $(#[$($enum_meta:meta),*])?
        $vis:vis enum $enum_name:ident
        {
            $(
                $variant_name:ident {
                    $($leading_literal:literal,)?
                    $(
                        $(#[separated_by=$separator:literal])?
                        $name:ident : $type:ty,
                        $($lit:literal)?
                    ),*
                    $(,)?
                },
            )+
        }
    ) => {
        $crate::make_item!{
            enum $enum_name vis=($vis) $(meta=$($enum_meta),*)?
            {
                $(
                    $variant_name
                    {
                        $($name:$type),*
                    }
                ),+
            }
        }
        impl $crate::Parsable for $enum_name {
            fn parse(text: &str) -> MyResult<$enum_name> {
                use $enum_name::*;
                let errors = [
                    $(
                        {
                            let result = (|| -> MyResult<$enum_name> {
                                $crate::make_reader_body!{
                                    constructor=$variant_name
                                    text=(text)
                                    $(leading_literal=$leading_literal)?
                                    $(name=$name, type=$type {$(until=$lit)? $(separator=$separator)?}),*
                                }
                            })();
                            match result {
                                Ok(x) => return Ok(x),
                                Err(e) => e,
                            }
                        },
                    )+
                ];
                
                let mut error_text = concat!("Could not find a match for enum ", stringify!($enum_name), "\n").to_string();
                for error in errors {
                    error_text.push_str("    ");
                    error_text.push_str(&error.to_string());
                    error_text.push_str("\n");
                }
                Err(From::from(error_text))
            }
        }
    };
}

mod tests {

    formatted_struct! {
        #[derive(PartialEq, Eq, Debug)]
        pub struct TestLeadingInnerAndTrailing {
            "game",
            first:String,
            "bz",
            #[separated_by=","]
            bz:Vec<i32>,
            "bar",
            bar:i32,
            "baz",
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
        #[derive(PartialEq, Eq, Debug)]
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
                "buz"
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
    #[test]
    fn parse_enum_variant_1() -> MyResult<()> {
        let parsed = VariantTest::parse("foob@rbaz3")?;
        assert_eq!(
            parsed,
            VariantTest::Foo {
                bar: "b@r".to_string(),
                bz: 3
            }
        );
        Ok(())
    }
    #[test]
    fn parse_enum_variant_2() -> MyResult<()> {
        let parsed = VariantTest::parse("fiz15buz")?;
        assert_eq!(parsed, VariantTest::Fiz { buz: 15 });
        Ok(())
    }
    #[test]
    fn parse_enum_err() {
        let parsed = VariantTest::parse("faz").unwrap_err();
        assert_eq!(
            parsed.to_string(), 
            "Could not find a match for enum VariantTest\n    skip didn't find Regex(\"foo\") when searching in \"faz\" as part of \"faz\"\n    skip didn't find Regex(\"fiz\") when searching in \"faz\" as part of \"faz\"\n"
        );
    }
}
