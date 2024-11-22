use aoc_utils::MyResult;

macro_rules! single_read {
    ($name:ident, $type:ty) => {
        let $name = read_until_end::<$type>();
    };
    ($name:ident, $type:ty, $lit:literal) => {
        let $name = read_until::<$type>($lit);
    };
}

macro_rules! make_reader {
    ($struct_name:ident, $($name:ident : $type:ty , $($lit:literal)?),*) => {
        fn reader() -> $struct_name {
            $(single_read!($name, $type $(,$lit)?);)*
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
    (struct $struct_name:ident {$($name:ident : $type:ty , $($lit:literal)?),* }) => {
        formatted_struct!{in_process $struct_name ($($name : $type , $($lit)?),* ) -> ( )}
        // make_reader!{$struct_name, $($name : $type , $($lit)?),*}
    };
    (in_process $struct_name:ident ($($name:ident : $type:ty , $($lit:literal)?),* ) -> ($($result:tt)*)) => {
        formatted_struct!{in_process $struct_name ( )-> ($($result)*  $($name:$type),*  ) }
    };

}

formatted_struct! {
    struct Foo {
            foo:String,
            "bz",
            baz:i32,
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
