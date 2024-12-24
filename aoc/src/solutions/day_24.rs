use std::{
    borrow::BorrowMut,
    cell::RefCell,
    collections::HashMap,
    fmt::Debug,
    sync::{Arc, Weak},
};

use aoc_utils::{formatted_struct, make_recursive_fn, DaySolution, MyResult};

formatted_struct! {
    #[derive(Debug)]
    pub struct Param {
        key: String,
        ": ",
        value: i32,
    }
}

formatted_struct! {
    #[derive(Debug)]
    pub struct Gate {
        lh: String,
        " ",
        op: String,
        " ",
        rh: String,
        " -> ",
        output: String,
    }
}

formatted_struct! {
    #[derive(Debug)]
    pub struct InputFormat {
        #[separated_by="\n"]
        params: Vec<Param>,
        "\n\n",
        #[separated_by="\n"]
        gates: Vec<Gate>,
    }
}

pub struct Solution;

// let values = Arc::new_cyclic(
//     |values_ref: &Weak<RefCell<HashMap<&str, Box<dyn FnMut() -> i32>>>>| {
//         RefCell::new({
//             let mut values: HashMap<&str, Box<dyn FnMut() -> i32>> = HashMap::new();
//             for param in &input.params {
//                 let old = values.insert(param.key.as_str(), Box::new(|| param.value));
//                 assert!(old.is_none());
//             }
//             for gate in &input.gates {
//                 let mut cached = None;
//                 let values_ref = values_ref.clone();
//                 let eval = move || -> i32 {
//                     *cached.get_or_insert_with(|| {
//                         let rc = values_ref.upgrade().unwrap();
//                         let mut values = rc.try_borrow_mut().unwrap();
//                         let lh = values.get_mut(gate.lh.as_str()).unwrap()();
//                         let rh = values.get_mut(gate.rh.as_str()).unwrap()();
//                         lh + rh
//                     })
//                 };
//                 let old = values.insert(&gate.output, Box::new(eval));
//                 assert!(old.is_none());
//             }
//             values
//         })
//     },
// );
// for (key, value_fn) in values.try_borrow_mut().unwrap() {}

impl DaySolution for Solution {
    type InputFormat = InputFormat;
    fn solve_1(input: &InputFormat) -> MyResult<impl Debug + 'static> {
        let params = input
            .params
            .iter()
            .map(|p| (p.key.as_str(), p.value))
            .collect::<HashMap<_, _>>();
        let gates = input
            .gates
            .iter()
            .map(|g| {
                (
                    g.output.as_str(),
                    (g.lh.as_str(), g.op.as_str(), g.rh.as_str()),
                )
            })
            .collect::<HashMap<_, _>>();
        let mut eval = make_recursive_fn(|eval: &mut dyn FnMut(String) -> i32, name| {
            if let Some(value) = params.get(name.as_str()) {
                return *value;
            }
            let (lh_name, op, rh_name) = gates[name.as_str()];
            let lh = eval(lh_name.to_string());
            let rh = eval(rh_name.to_string());
            match op {
                "AND" => lh & rh,
                "XOR" => lh ^ rh,
                "OR" => lh | rh,
                _ => unimplemented!(),
            }
        });
        let mut z_gates = gates
            .keys()
            .cloned()
            .filter(|name| name.starts_with('z'))
            .collect::<Vec<_>>();
        z_gates.sort_by(|lh, rh| rh.cmp(lh));
        let bin_result = z_gates
            .into_iter()
            .map(|name| eval(name.to_string()).to_string())
            .collect::<Vec<_>>()
            .join("");
        Ok(i64::from_str_radix(&bin_result, 2))
    }
    fn preferred_sample_input() -> i32 {
        1
    }
}
