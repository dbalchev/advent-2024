use std::{collections::HashMap, hash::Hash};

struct HashMapMemo<F, State, Result> {
    memo: HashMap<State, Result>,
    computation: F,
}

impl<F, State, Result> HashMapMemo<F, State, Result>
where
    Result: Copy,
    F: Copy + Fn(&mut dyn FnMut(State) -> Result, State) -> Result,
    State: Eq + Hash + Clone,
{
    fn get(&mut self, state: State) -> Result {
        if let Some(x) = self.memo.get(&state) {
            return *x;
        };
        let new_result = {
            let computation = self.computation;
            let mut recursive_call = |other_state| self.get(other_state);
            computation(&mut recursive_call, state.clone())
        };
        self.memo.insert(state, new_result);
        new_result
    }
}

pub fn make_recursive_fn<F, State, Result>(computation: F) -> impl FnMut(State) -> Result
where
    Result: Copy,
    F: Copy + Fn(&mut dyn FnMut(State) -> Result, State) -> Result,
    State: Eq + Hash + Clone,
{
    let mut hmm = HashMapMemo {
        memo: HashMap::new(),
        computation,
    };
    move |state| hmm.get(state)
}

#[macro_export]
macro_rules! cartesian_range {
    ($computation:block, [$i:ident, $low:expr, $high:expr]) => {
        for $i in ($low..$high) $computation
    };
    ($computation:block, [$first_i:ident, $first_low:expr, $first_high:expr], $([$i:ident, $low:expr,$high:expr]),+) => {
        for $first_i in $first_low..$first_high {
            $crate::cartesian_range!($computation, $([$i, $low,$high]),+)
        }
    }
}

#[macro_export]
macro_rules! cartesian_index_and_size {
    ([$i:expr, $low:expr, $high:expr]) => {
        ($i - $low, $high - $low)
    };
    ([$first_i:expr, $first_low:expr, $first_high:expr], $([$i:expr, $low:expr,$high:expr]),+) => {
        {
            let (partial_index, partial_size) = $crate::cartesian_index_and_size!($([$i, $low,$high]),+);
            (partial_index + partial_size * ($first_i - $first_low), ($first_high - $first_low) * partial_size)
        }
    };
}

#[macro_export]
macro_rules! make_getter {
    ($result:ident, $([$i:ident, $low:expr,$high:expr]),+) => {
        |$($i),+| {
            let (index, _) = $crate::cartesian_index_and_size!($([$i, $low,$high]),+);
            $result[index as usize]
        }
    };
    (move $result:ident, $([$i:ident, $low:expr,$high:expr]),+) => {
        move |$($i),+| {
            let (index, _) = $crate::cartesian_index_and_size!($([$i, $low,$high]),+);
            $result[index as usize]
        }
    };
}

#[macro_export]
macro_rules! sequential_memo {
    ($f:ident[$($i:ident in [$low:expr,$high:expr]),+] = $computation:expr) => {
        {
            let (_, total_size) = $crate::cartesian_index_and_size!($([$low, $low,$high]),+);
            let mut result = Vec::with_capacity(total_size);
            $crate::cartesian_range!({
                let (index, _size) = $crate::cartesian_index_and_size!($([$i, $low,$high]),+);
                assert_eq!(result.len(), index);
                let new_element = {
                    let $f = $crate::make_getter!(result, $([$i, $low,$high]),+);
                    $computation
                };
                result.push(new_element);
                // println!("index={}/{}", index, _size);
                // println!("new_element={}", new_element);
                // $(println!("{}={}", stringify!($i), $i);)+
                // println!();
            }, $([$i, $low, $high]),+);
            $crate::make_getter!(move result, $([$i, $low,$high]),+)
        }
    };
}
#[cfg(test)]
mod tests {

    use std::cell::RefCell;

    use super::*;

    #[test]
    fn test_recursive_fn() {
        let call_count = RefCell::new(0);
        let fib_r = |r: &mut dyn FnMut(i32) -> i32, n| {
            *call_count.borrow_mut() += 1;
            match n {
                0 | 1 => 1,
                _ => r(n - 1) + r(n - 2),
            }
        };
        let mut fib = make_recursive_fn(fib_r);
        assert_eq!(fib(5), 8);
        assert_eq!(*call_count.borrow(), 6);
    }
    #[test]
    fn test_sequential_memo() {
        let f = sequential_memo!(
            f[i in [0, 3], j in [1, 3]] = if i == 0 {3 * i + 2 * j} else {i * f(i - 1, j) + j * 5}
        );
        assert_eq!(f(2, 2), 38);
    }
}
