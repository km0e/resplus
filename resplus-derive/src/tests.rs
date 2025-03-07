use resplus_derive::flog;
use resplus_impl as resplus;

use test_util::*;

#[test]
fn no_args() {
    assert_result!(flog!(f0()), "source: Error\n  f0()");
    assert_result!(flog!(f1(1)), "source: Error\n  f1(_)");
    assert_result!(flog!(f2(1, 1)), "source: Error\n  f2(_, _)");
}

#[test]
fn with_literal() {
    assert_result!(flog!(f1(1), 0), "source: Error\n  f1(1)");
    assert_result!(flog!(f2(1, 1), 0, 1), "source: Error\n  f2(1, 1)");
    assert_result!(flog!(f2(1, 1), 0..1, 1), "source: Error\n  f2(1, 1)");
    assert_result!(flog!(f3(1, 1, 1), ..), "source: Error\n  f3(1, 1, 1)");
}

#[test]
fn with_variable() {
    let a = 1;
    assert_result!(flog!(f1(a), 0), "source: Error\n  f1(1)");
    assert_result!(flog!(f2(a, a), 0, 1), "source: Error\n  f2(1, 1)");
    assert_result!(flog!(f2(a, a), 0..1, 1), "source: Error\n  f2(1, 1)");
    assert_result!(flog!(f3(a, a, a), ..), "source: Error\n  f3(1, 1, 1)");
}

#[test]
fn with_expr() {
    assert_result!(flog!(f1(1 + 1), 0), "source: Error\n  f1(2)");
    assert_result!(flog!(f2(1 + 1, 1 + 1), 0, 1), "source: Error\n  f2(2, 2)");
    assert_result!(
        flog!(f2(1 + 1, 1 + 1), 0..1, 1),
        "source: Error\n  f2(2, 2)"
    );
    assert_result!(
        flog!(f3(1 + 1, 1 + 1, 1 + 1), ..),
        "source: Error\n  f3(2, 2, 2)"
    );
}

#[test]
fn mixed_args() {
    assert_result!(flog!(f2(1, 1), 1), "source: Error\n  f2(_, 1)");
    assert_result!(flog!(f2(1, 1), 0), "source: Error\n  f2(1, _)");
    assert_result!(flog!(f2(1, 1), 0, 1), "source: Error\n  f2(1, 1)");
    assert_result!(flog!(f2(1, 1), 0..1, 1), "source: Error\n  f2(1, 1)");
    assert_result!(flog!(f3(1, 1, 1), 0, 1), "source: Error\n  f3(1, 1, _)");
    assert_result!(flog!(f3(1, 1, 1), 0..1, 2), "source: Error\n  f3(1, _, 1)");
}
#[cfg(feature = "async")]
#[tokio::test]
async fn async_no_args() {
    async_assert_result!(flog!(af0()), "source: Error\n  af0()");
    async_assert_result!(flog!(af1(1)), "source: Error\n  af1(_)");
    async_assert_result!(flog!(af2(1, 1)), "source: Error\n  af2(_, _)");
}
