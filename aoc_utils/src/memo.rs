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
}
