use aoc_utils::MyResult;

macro_rules! parse_single {
    (type=$type:ty, str=$expr:expr) => {
        $type::parse($expr)
    };
    (type=$type:ty, separator=$separator:literal, str=$expr:expr) => {
        $type::parse_separated($expr, $separator)
    };
}

macro_rules! single_read {
    () => {
        read_until_end::<$type>()
    };
    ($lit:literal) => {
        read_until($lit)
    };
}

macro_rules! make_reader {
    (struct=$struct_name:ident $(name=$name:ident, type=$type:ty, $(until=$lit:literal)?, $(separator=$separator:literal)?),*) => {
        fn reader() -> $struct_name {
            $(let $name = parse_single!(type=$type, $(separator=$separator,)? str=single_read!($($lit)?));)*
            return $struct_name {
                $($name),*
            }
        }
    }
}

macro_rules! formatted_struct {
    (in_process $struct_name:ident ()-> ($($result:tt)*)) => {
        struct $struct_name {
            $($result)*
        }
    };
    (struct $struct_name:ident
        {
            $($leading_literal:literal)?
            $(
                $(#[separated_by=$separator:literal])?
                $name:ident : $type:ty
                $(,$lit:literal)?
            ),*
        }) => {
        formatted_struct!{in_process $struct_name ($($name : $type),* ) -> ( )}
        // make_reader!{struct=$struct_name $(name=$name, type=$type, $(until=$lit)?, $(separator=$separator)?),*}
    };
    (in_process $struct_name:ident ($($name:ident : $type:ty),* ) -> ($($result:tt)*)) => {
        formatted_struct!{in_process $struct_name ( )-> ($($result)*  $($name:$type),*  ) }
    };

}

formatted_struct! {
    struct Foo {
        "game"
        foo:String,
        "bz",
        #[separated_by=","]
        baz:Vec<i32>,
        "bar",
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
