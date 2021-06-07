#![cfg_attr(feature = "nightly",
    cfg_attr(all(), doc = include_str!("../README.md")),
)]
#![allow(nonstandard_style)]

#[must_use = "You need to call `.try_eval()`"]
pub
const
fn with_state<State> (state: State)
  -> Runner<State>
{
    Runner { state }
}

pub
struct Runner<State> {
    state: State,
}

pub
struct RunnerWithTryEval<State, try_eval> {
    state: State,
    try_eval: try_eval,
}

impl<State> Runner<State> {
    #[must_use =
        "`.try_eval()` is lazy and does nothing: you need to call `.finally()`"
    ]
    pub
    fn try_eval<R, try_eval> (
        self: Runner<State>,
        __: try_eval,
    ) -> RunnerWithTryEval<State, try_eval>
    where
        try_eval: FnOnce(&'_ mut State) -> R,
    {
        let Runner { state } = self;
        RunnerWithTryEval {
            state,
            try_eval: __,
        }
    }

    #[doc(hidden)] pub
    fn finally<R, F> (
        self: Runner<State>,
        _: F,
    ) -> R
    where
        Self : __::missing_try_eval<F>,
    {
        <Self as __::missing_try_eval<F>>::unreachable()
    }
}

mod __ {
    pub trait missing_try_eval<F> {
        fn unreachable() -> !;
    }
}

impl<State, try_eval> RunnerWithTryEval<State, try_eval> {
    pub
    fn finally<R, finally> (
        self: RunnerWithTryEval<State, try_eval>,
        __: finally,
    ) -> R
    where
        try_eval: FnOnce(&'_ mut State) -> R,
        finally: FnOnce(State),
    {
        use ::core::mem::ManuallyDrop as MD;

        struct WithDrop<State, Finally>
        where
            Finally : FnOnce(State),
        {
            state: MD<State>,
            finally: MD<Finally>,
        }

        impl<State, Finally> Drop for WithDrop<State, Finally>
        where
            Finally : FnOnce(State),
        {
            fn drop (self: &'_ mut Self)
            {
                unsafe {
                    let state = MD::take(&mut self.state);
                    let finally = MD::take(&mut self.finally);
                    let () = finally(state);
                }
            }
        }

        let RunnerWithTryEval { state, try_eval } = self;
        let ref mut state_with_drop = WithDrop {
            state: MD::new(state),
            finally: MD::new(__),
        };
        try_eval(&mut state_with_drop.state)
    }
}

#[cfg(all(doc, feature = "nightly"))]
#[cfg_attr(all(doc, feature = "nightly"),
    cfg_attr(all(), doc = include_str!("compile_fail_tests.md")),
)]
mod compile_fail_tests {}
