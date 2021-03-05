# `::unwind_safe`

[![Repository](https://img.shields.io/badge/repository-GitHub-brightgreen.svg)](https://github.com/danielhenrymantilla/unwind_safe.rs)
[![Latest version](https://img.shields.io/crates/v/unwind_safe.svg)](https://crates.io/crates/unwind_safe)
[![Documentation](https://docs.rs/unwind_safe/badge.svg)](https://docs.rs/unwind_safe)
[![MSRV](https://img.shields.io/badge/MSRV-1.42.0-white)](https://gist.github.com/danielhenrymantilla/8e5b721b3929084562f8f65668920c33)
[![License](https://img.shields.io/crates/l/unwind_safe.svg)](https://github.com/danielhenrymantilla/unwind_safe.rs/blob/master/LICENSE-ZLIB)
[![CI](https://github.com/danielhenrymantilla/unwind_safe.rs/workflows/CI/badge.svg)](https://github.com/danielhenrymantilla/unwind_safe.rs/actions)

### Readable unwind-safe code thanks to a try-finally-looking builder pattern

```rust
let mut body_called = false;
let mut finally_called = false;

// Let's imagine some code being run in a context where
// panics do not affect us (`panic::catch_unwind`), or some
// executor running stuff on another thread…
let _ = ::crossbeam::thread::scope(|s| drop(s.spawn(|_| {

    let ft = {
        ::unwind_safe::with_state(())
            .try_eval(|_| {
                body_called = true;
                if ::rand::random() {
                    panic!();
                } else {
                    42
                }
            })
            .finally(|_| { // <- The point of this crate!
                finally_called = true;
            })
    };
    // This is only reached when `try_eval` does not panic, obviously.
    assert_eq!(ft, 42);

})));

// Whatever code path was taken, the finally block is always executed
// (that's the point of this crate!).
// From a place that survives the panic (if any), we thus observe:
assert!(body_called);
assert!(finally_called);
```

#### With an actual owned state

If the destructor requires access to an owned `State`<sup>1</sup> in the
`finally` / deferred block, (`type State = …:` of your choosing),

 1. you can feed that to the `::unwind_safe::with_state::<State>` API
    entry-point;

 1. the `.try_eval(|state: &mut State| { … })` block will then have access to an
    _exclusive_ borrow (`&mut`) to it through the closure's parameter,

 1. and the `.finally` block will get access to that state in an owned fashion
    through its own closure parameter: `.finally(|state: State| { … })`.

<small><sup>1</sup> This "owned" state may still be a borrow, _e.g._,
`type State = &mut …;`</small>

### Can `unsafe` code rely on the `finally` code always being run?

Yes! That's the point of the crate, and why it is so named: you can use this
`.finally` pattern to ensure your `unsafe` code is unwind-safe ✅

### Similar to `::scopeguard`

This is similar to [`::scopeguard::defer!`](
https://docs.rs/scopeguard/1.*/scopeguard/macro.defer.html), but for the
added ability to get owned access in the `finally` / `defer`-red block
while still letting the main block have `&mut` references to it.

It is thus actually the same as [`::scopeguard::guard`](
https://docs.rs/scopeguard/1.*/scopeguard/fn.guard.html)! The only (but crucial,
imho) difference between these two is the readability of the code: with
`.try_eval(…).finally(…)`, it is more obvious that the code in the
`.finally(…)` part is running _after_ the one on the main block, which is not
obvious at first sight with `::scopeguard`'s API (it requires knowing how it
works).
