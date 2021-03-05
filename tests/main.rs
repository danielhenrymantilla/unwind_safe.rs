#[test]
fn main ()
{
    use ::std::panic;

    let mut body_called = false;
    let mut finally_called = false;

    let _ = panic::catch_unwind(panic::AssertUnwindSafe(|| -> ::core::convert::Infallible {
        struct NotCopy();
        ::unwind_safe::with_state(NotCopy())
            .try_eval(|_: &'_ mut NotCopy| {
                body_called = true;
                panic!();
            })
            .finally(|_: NotCopy| {
                finally_called = true;
            })
    }));
    assert!(body_called);
    assert!(finally_called);
}
