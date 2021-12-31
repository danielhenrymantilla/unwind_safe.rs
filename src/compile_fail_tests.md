```rust,compile_fail
#![deny(unused_must_use)]
::unwind_safe::with_state(())
;
```

```rust,compile_fail
#![deny(unused_must_use)]
::unwind_safe::with_state(())
    .try_eval(|_| {
        /* … */
    })
;
```

```rust,compile_fail
#![deny(unused_must_use)]
::unwind_safe::try_eval(|| ())
;
```

```rust,compile_fail
::unwind_safe::with_state(())
    .finally(|_| {
        /* … */
    })
;
```
