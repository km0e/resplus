use resplus_derive::{define, flog};
use resplus_impl as resplus;

use test_util::*;

#[test]
fn flog_no_args() {
    assert_result!(flog!(f0()), "source: Error\n  f0()");
    assert_result!(flog!(f1(1)), "source: Error\n  f1(_)");
    assert_result!(flog!(f2(1, 1)), "source: Error\n  f2(_, _)");
}

#[test]
fn flog_with_literal() {
    assert_result!(flog!(f1(1), 0), "source: Error\n  f1(1)");
    assert_result!(flog!(f2(1, 1), 0, 1), "source: Error\n  f2(1, 1)");
    assert_result!(flog!(f2(1, 1), 0..1, 1), "source: Error\n  f2(1, 1)");
    assert_result!(flog!(f3(1, 1, 1), ..), "source: Error\n  f3(1, 1, 1)");
}

#[test]
fn flog_with_variable() {
    let a = 1;
    assert_result!(flog!(f1(a), 0), "source: Error\n  f1(1)");
    assert_result!(flog!(f2(a, a), 0, 1), "source: Error\n  f2(1, 1)");
    assert_result!(flog!(f2(a, a), 0..1, 1), "source: Error\n  f2(1, 1)");
    assert_result!(flog!(f3(a, a, a), ..), "source: Error\n  f3(1, 1, 1)");
}

#[test]
fn flog_with_expr() {
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
fn flog_mixed_args() {
    assert_result!(flog!(f2(1, 1), 1), "source: Error\n  f2(_, 1)");
    assert_result!(flog!(f2(1, 1), 0), "source: Error\n  f2(1, _)");
    assert_result!(flog!(f2(1, 1), 0, 1), "source: Error\n  f2(1, 1)");
    assert_result!(flog!(f2(1, 1), 0..1, 1), "source: Error\n  f2(1, 1)");
    assert_result!(flog!(f3(1, 1, 1), 0, 1), "source: Error\n  f3(1, 1, _)");
    assert_result!(flog!(f3(1, 1, 1), 0..1, 2), "source: Error\n  f3(1, _, 1)");
}

#[cfg(feature = "async")]
#[tokio::test]
async fn flog_async_no_args() {
    async_assert_result!(flog!(af0()), "source: Error\n  af0()");
    async_assert_result!(flog!(af1(1)), "source: Error\n  af1(_)");
    async_assert_result!(flog!(af2(1, 1)), "source: Error\n  af2(_, _)");
}

#[test]
fn flog_method_no_args() {
    let t = Test;
    assert_result!(flog!(t.f0()), "source: Error\n  f0()");
    assert_result!(flog!(t.f1(1)), "source: Error\n  f1(_)");
    assert_result!(flog!(t.f2(1, 1)), "source: Error\n  f2(_, _)");
}

#[test]
fn flog_method_with_literal() {
    let t = Test;
    assert_result!(flog!(t.f1(1), 0), "source: Error\n  f1(1)");
    assert_result!(flog!(t.f2(1, 1), 0, 1), "source: Error\n  f2(1, 1)");
    assert_result!(flog!(t.f2(1, 1), 0..1, 1), "source: Error\n  f2(1, 1)");
    assert_result!(flog!(t.f3(1, 1, 1), ..), "source: Error\n  f3(1, 1, 1)");
}

#[derive(Debug)]
enum ErrorSource {}

impl std::fmt::Display for ErrorSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error")
    }
}

define!(ErrorSource);
