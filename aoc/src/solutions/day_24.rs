use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
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
    #[derive(Debug, Clone)]
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
    fn solve_2(input: &Self::InputFormat) -> MyResult<impl Debug + 'static> {
        let mut current_gates = input.gates.clone();
        let mut swaps = Vec::new();
        let add_swap =
            |a: String, b: String, current_gates: &mut Vec<Gate>, swaps: &mut Vec<String>| {
                *current_gates = current_gates
                    .iter_mut()
                    .map(|g| {
                        if g.output == a {
                            Gate {
                                output: b.clone(),
                                ..g.clone()
                            }
                        } else if g.output == b {
                            Gate {
                                output: a.clone(),
                                ..g.clone()
                            }
                        } else {
                            g.clone()
                        }
                    })
                    .collect::<Vec<_>>();
                swaps.extend([a, b]);
            };
        'solve_loop: loop {
            macro_rules! query_element {
                ($g:ident, param=$v:expr) => {
                    [$g.lh.as_str(), $g.rh.as_str()].contains(&$v)
                };
                ($g:ident, $k:ident=$v:expr) => {
                    ($g.$k.as_str() == $v)
                };
            }
            macro_rules! query_gates {
                (select $($r:ident),+ where $($k:tt=$v:expr),*) => {
                    current_gates.iter().filter(|&g| $(query_element!(g,$k=$v))&&+).flat_map(|g| [$(g.$r.as_str()),+]).collect::<HashSet<_>>()
                };
            }
            let z_gates = &input
                .gates
                .iter()
                .map(|g| g.output.as_str())
                .filter(|name| name.starts_with('z'))
                .collect::<Vec<_>>();

            assert_eq!(
                Vec::from_iter(
                    query_gates!(select output where param = "x00", param="y00", op = "XOR")
                ),
                vec!["z00"]
            );
            for i in 1..(z_gates.len() - 1) {
                let x = format!("x{:02}", i);
                let x = x.as_str();
                let y = format!("y{:02}", i);
                let y = y.as_str();
                let z = format!("z{:02}", i);

                let xor_inner_1 = [
                    query_gates!(select output where param=x, op="XOR"),
                    query_gates!(select output where param=y, op="XOR"),
                ]
                .into_iter()
                .flatten()
                .collect::<HashSet<_>>();
                let xor_inner_2 = query_gates!(select lh, rh where output=z, op="XOR");
                if xor_inner_1.len() == 1 {
                    let xor_inner_2_candidates = query_gates!(select output where param=xor_inner_1.iter().next().unwrap(), op="XOR");

                    if xor_inner_2.len() != 2 {
                        assert_eq!(
                            xor_inner_2_candidates.len(),
                            1,
                            "{:?}",
                            xor_inner_2_candidates
                        );
                        println!(
                            "{} is swapped with {:?} [xor_inner_2 = {:?}]",
                            z, xor_inner_2_candidates, xor_inner_2
                        );
                        let z = z.to_string();
                        let new_z = xor_inner_2_candidates.iter().next().unwrap().to_string();
                        add_swap(z, new_z, &mut current_gates, &mut swaps);
                        continue 'solve_loop;
                    } else {
                        assert_eq!(xor_inner_2.len(), 2, "{:?} {:?}", xor_inner_1, xor_inner_2);
                        let diff = xor_inner_2.difference(&xor_inner_1).collect::<Vec<_>>();
                        if diff.len() != 1 {
                            let xor_inner_2 = Vec::from_iter(xor_inner_2.clone());
                            let op0 = *query_gates!(select op where output=xor_inner_2[0])
                                .iter()
                                .next()
                                .unwrap();
                            assert!(["OR", "AND"].contains(&op0));
                            let not_carry = if op0 == "OR" {
                                xor_inner_2[1]
                            } else {
                                xor_inner_2[0]
                            };
                            let xor_node = xor_inner_1.iter().next().unwrap().to_string();
                            println!(
                                "{} is swapped with {} [xor_inner = [{:?}, {:?}],]",
                                xor_node, not_carry, xor_inner_1, xor_inner_2,
                            );

                            add_swap(
                                xor_node,
                                not_carry.to_string(),
                                &mut current_gates,
                                &mut swaps,
                            );
                            continue 'solve_loop;
                        }
                    }
                } else {
                    println!("{:?} {:?}", xor_inner_1, xor_inner_2);
                    panic!()
                };
            }

            println!("{:?}", z_gates.len());

            break;
        }
        swaps.sort();
        Ok(swaps.join(","))
    }
    fn preferred_sample_input() -> i32 {
        1
    }
}
