use resplus_derive::flog;
use resplus_impl as resplus;

use test_util::*;

macro_rules! assert {
    ($r:expr, $e:expr) => {
        assert_eq!(
            || -> Result<(), resplus_impl::Error<Error>> { Ok($r) }()
                .unwrap_err()
                .to_string(),
            $e
        )
    };
}

#[test]
fn no_args() {
    assert!(flog!(f0()), "source: Error\n  f0()");
    assert!(flog!(f1(1)), "source: Error\n  f1(_)");
    assert!(flog!(f2(1, 1)), "source: Error\n  f2(_, _)");
}

#[test]
fn with_literal() {
    assert!(flog!(f1(@1)), "source: Error\n  f1(1)");
    assert!(flog!(f2(@1, @1)), "source: Error\n  f2(1, 1)");
}

#[test]
fn with_variable() {
    let a = 1;
    assert!(flog!(f1(@a)), "source: Error\n  f1(1)");
    assert!(flog!(f2(@a, @a)), "source: Error\n  f2(1, 1)");
}

#[test]
fn with_expr() {
    assert!(flog!(f1(@1+1)), "source: Error\n  f1(2)");
    assert!(flog!(f2(@1+1,@1+1)), "source: Error\n  f2(2, 2)");
}

#[test]
fn mixed_args() {
    assert!(flog!(f2(1, @1)), "source: Error\n  f2(_, 1)");
    assert!(flog!(f2(@1, 1)), "source: Error\n  f2(1, _)");
}

#[test]
fn ft() {}
