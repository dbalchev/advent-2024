use aoc_utils::MyResult;

macro_rules! formatted_struct {
    (struct $struct_name:ident {$($token:tt)*}) => {
        formatted_struct!{in_process $struct_name ($($token)*) -> ( )}
    };
    (in_process $struct_name:ident ($first_name:ident : $first_type:ty , $($lit:literal, $name:ident : $type:ty),*) -> ($($result:tt)*)) => {
        formatted_struct!{in_process $struct_name ( )-> ($($result)*  $first_name:$first_type, $($name:$type),*) }
    };
    (in_process $struct_name:ident ()-> ($($result:tt)*)) => {
        struct $struct_name {
            $($result)*
        }
    }
}

formatted_struct! {
    struct Foo {
            foo:String,
            "bar",
            bar:i32
    }
}

pub type InputFormat = String;

pub fn solve_1(input: &InputFormat) -> MyResult<String> {
    let f = Foo {
        foo: "x".to_string(),
        bar: 3,
    };
    Ok(format!("Hello {}", input))
}

pub fn solve_2(_input: &InputFormat) -> MyResult<String> {
    Err("not implemented")?
}
