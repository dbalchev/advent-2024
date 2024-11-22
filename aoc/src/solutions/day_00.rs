use aoc_utils::MyResult;

macro_rules! formatted_struct {
    (in_process $struct_name:ident ()-> ($($result:tt)*)) => {
        struct $struct_name {
            $($result)*
        }
    };
    (struct $struct_name:ident {$($name:ident : $type:ty , $($lit:literal)?),* }) => {
        formatted_struct!{in_process $struct_name ($($name : $type , $($lit)?),* ) -> ( )}
    };
    (in_process $struct_name:ident ($($name:ident : $type:ty , $($lit:literal)?),* ) -> ($($result:tt)*)) => {
        formatted_struct!{in_process $struct_name ( )-> ($($result)*  $($name:$type),*  ) }
    };

}

formatted_struct! {
    struct Foo {
            foo:String,
            "bar",
            bar:i32,
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
